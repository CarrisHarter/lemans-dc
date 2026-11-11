#!/usr/bin/env python3
"""Propose function names from the string labels in Ghidra's exported pseudocode.

Ghidra already resolved which functions touch which strings; the export encodes
that as `PTR_s_<mangled text>_<address>`. This reads those labels, maps the
address back to the decoded string, and proposes a snake_case function name.

Output is a proposal list for review — never a silent rename.

Usage: name-from-ghidra-strings.py <dir-of-.c> <binary> [--tsv out] [--md map]
"""
from __future__ import annotations

import argparse
import pathlib
import re

LABEL = re.compile(r"PTR_s_[A-Za-z0-9_]*?_([0-9a-f]{8})\b")
# The address in a Ghidra string label is the pointer's address, so the text
# between `s_` and the trailing address is often the only recoverable evidence.
LABEL_TEXT = re.compile(r"(?:PTR_)?s_([A-Za-z0-9_]{6,}?)_[0-9a-f]{6,8}\b")
IDENT_START = re.compile(
    r"^(?:[_@]?[A-Za-z][A-Za-z0-9]{2,40})"      # identifier-ish prefix
    r"(?=\s*(?:[-:]|\s{2,}|$))"                  # followed by a separator
)
NOISE = re.compile(r"[\x00-\x1f]")

# Words that make poor function names even when they start a message.
STOPWORDS = {
    "the", "this", "that", "error", "warning", "assert", "failed", "failure",
    "unable", "cannot", "can", "could", "invalid", "out", "in", "not", "no",
}

PREFIX_MAP = {
    "am": "audio_",          # amStreamSetVolume
    "lm": "lm_",             # lmTrackInfo
}


def snake(text: str) -> str:
    text = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", text)
    text = re.sub(r"[^A-Za-z0-9]+", "_", text).strip("_").lower()
    return re.sub(r"_{2,}", "_", text)


def scan_strings(data: bytes, min_len: int = 6) -> dict[int, str]:
    out: dict[int, str] = {}
    start = None
    for i, byte in enumerate(data):
        printable = 0x20 <= byte < 0x7F
        if printable and start is None:
            start = i
        elif not printable and start is not None:
            if i - start >= min_len:
                text = data[start:i].decode("ascii").strip()
                if text:
                    out[0x8C010000 + start] = text
            start = None
    return out


def propose(text: str) -> str | None:
    if NOISE.search(text):
        return None
    m = IDENT_START.match(text)
    if m:
        candidate = m.group(0)
        if candidate.lower() not in STOPWORDS and len(candidate) >= 6:
            return snake(candidate)
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("dir", type=pathlib.Path, help="directory of exported .c files")
    ap.add_argument("binary", type=pathlib.Path)
    ap.add_argument("--tsv", type=pathlib.Path, default=pathlib.Path("analysis/labels.tsv"))
    ap.add_argument("--md", type=pathlib.Path, default=pathlib.Path("notes/address-map.md"))
    args = ap.parse_args()

    strings = scan_strings(args.binary.read_bytes())
    rows = []
    for path in sorted(args.dir.glob("*.c")):
        try:
            address = int(path.stem, 16)
        except ValueError:
            continue
        text = path.read_text(errors="replace")
        candidates = []
        for match in LABEL.finditer(text):
            addr = int(match.group(1), 16)
            value = strings.get(addr)
            if value:
                candidates.append((addr, value))
        if not candidates:
            # Fall back to the label text itself ("s_loading_track_objects_manager").
            for match in LABEL_TEXT.finditer(text):
                phrase = match.group(1).replace("_", " ").strip()
                name = propose(phrase)
                if name:
                    candidates.append((0, phrase))
                    break
        if not candidates:
            continue
        best = None
        for addr, value in candidates:
            name = propose(value)
            if name:
                best = (name, addr, value)
                break
        if best:
            rows.append((address, best[0], best[2][:70], best[1]))

    seen: dict[str, int] = {}
    unique = []
    for address, name, evidence, saddr in sorted(rows):
        count = seen.get(name, 0)
        seen[name] = count + 1
        if count:
            name = f"{name}_{count + 1}"
        unique.append((address, name, evidence, saddr))

    args.tsv.parent.mkdir(parents=True, exist_ok=True)
    with args.tsv.open("w") as fh:
        fh.write("address\tproposed\tevidence\tstring_address\n")
        for address, name, evidence, saddr in unique:
            fh.write(f"0x{address:08x}\t{name}\t{evidence}\t0x{saddr:08x}\n")

    print(f"strings decoded {len(strings)}")
    print(f"functions with named strings {len(rows)}")
    print(f"proposals {len(unique)} -> {args.tsv}")
    for address, name, evidence, _ in unique[:10]:
        print(f"  0x{address:08x}  {name:<32} {evidence[:50]}")

    lines = [
        "## Auto-proposed names (Ghidra string labels)",
        "",
        "From `tools/name-from-ghidra-strings.py`: each name comes from a string",
        "Ghidra resolved as referenced by that function. Evidence column is the",
        "string itself. Names are proposals until a spec verifies the behaviour.",
        "",
        "| address | proposed | evidence |",
        "| --- | --- | --- |",
    ]
    for address, name, evidence, _ in unique[:300]:
        safe = evidence.replace("|", "\\|")
        lines.append(f"| `0x{address:08x}` | `{name}` | {safe} |")
    if len(unique) > 300:
        lines.append(f"| … | | {len(unique) - 300} more in analysis/labels.tsv |")

    md = args.md
    existing = md.read_text() if md.exists() else "# Address map\n"
    marker = "\n## Auto-proposed names (Ghidra string labels)"
    if marker in existing:
        existing = existing.split(marker)[0]
    md.write_text(existing.rstrip() + "\n\n" + "\n".join(lines) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
