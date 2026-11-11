#!/usr/bin/env python3
"""Propose function names from the debug strings a function references.

Offline: no Ghidra session, no per-string round trip. It decodes the two SH-4
PC-relative literal forms, matches their targets against a scanned string table,
and maps the referencing instruction to the function that contains it.

  mov.l @(disp,PC),Rn   1101 nnnn dddddddd   target = ((pc+4) & ~3) + disp*4
  mova  @(disp,PC),R0   1100 0111 dddddddd   target = ((pc+4) & ~3) + disp*4

A name is only proposed when the string looks like an identifier (the engine's
diagnostics are written as "functionName - what went wrong"), so this prints
proposals, never silent renames.

Usage: name-from-strings.py <binary> <symbols.tsv> [--out labels.tsv] [--md map.md]
"""
from __future__ import annotations

import argparse
import pathlib
import re
import sys

IDENT = re.compile(r"^([A-Za-z_][A-Za-z0-9_]{4,40})(?:\s*[-:]\s|\s*$)")
KNOWN_EXT = re.compile(r"^[A-Za-z0-9_]{6,40}$")


def load_functions(symbols: pathlib.Path) -> list[tuple[int, int, str]]:
    """(start, end_inclusive, name) from the export's symbols.tsv."""
    out = []
    for line in symbols.read_text().splitlines()[1:]:
        parts = line.split("\t")
        if len(parts) < 5:
            continue
        try:
            start = int(parts[0], 16)
        except ValueError:
            continue
        ranges = parts[4]
        end = start
        if "-" in ranges:
            try:
                end = int(ranges.split(";")[0].split("-")[1], 16)
            except (ValueError, IndexError):
                pass
        out.append((start, end, parts[1]))
    out.sort()
    return out


def scan_strings(data: bytes, min_len: int = 6) -> dict[int, str]:
    strings: dict[int, str] = {}
    start = None
    for i, byte in enumerate(data):
        printable = 0x20 <= byte < 0x7F
        if printable and start is None:
            start = i
        elif not printable and start is not None:
            if i - start >= min_len:
                text = data[start:i].decode("ascii").strip()
                if text:
                    strings[start] = text
            start = None
    return strings


def find_references(data: bytes) -> list[tuple[int, int, str]]:
    """(pc, referenced_address, how) for PC-relative data access.

    `mov.l @(disp,PC),Rn` loads a *literal slot*; the interesting address is the
    32-bit word stored there (that is where the string pointer lives). `mova`
    computes the address directly.
    """
    refs = []
    base = 0x8C010000
    for off in range(0, len(data) - 1, 2):
        op = data[off] | (data[off + 1] << 8)
        top = op & 0xF000
        if top == 0xD000:
            disp = op & 0xFF
            pc = base + off
            slot = ((pc + 4) & ~3) + disp * 4
            rel = slot - base
            if 0 <= rel <= len(data) - 4:
                value = int.from_bytes(data[rel:rel + 4], "little")
                refs.append((pc, value, "mov.l"))
        elif (op & 0xFF00) == 0xC700:
            disp = op & 0xFF
            pc = base + off
            refs.append((pc, ((pc + 4) & ~3) + disp * 4, "mova"))
    return refs


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("binary", type=pathlib.Path)
    ap.add_argument("symbols", type=pathlib.Path)
    ap.add_argument("--out", type=pathlib.Path, default=pathlib.Path("analysis/labels.tsv"))
    ap.add_argument("--md", type=pathlib.Path, default=pathlib.Path("notes/address-map.md"))
    args = ap.parse_args()

    data = args.binary.read_bytes()
    strings = scan_strings(data)
    functions = load_functions(args.symbols)
    refs = find_references(data)
    print(f"strings scanned     {len(strings)}")
    print(f"literal loads       {len(refs)}")
    print(f"functions           {len(functions)}")

    # function start -> list of (string address, text)
    per_function: dict[int, list[tuple[int, str]]] = {}
    starts = [f[0] for f in functions]

    import bisect

    def containing(pc: int):
        i = bisect.bisect_right(starts, pc) - 1
        if i < 0:
            return None
        start, end, name = functions[i]
        return (start, end, name) if start <= pc <= end else None

    hits = 0
    for pc, target, _how in refs:
        text = strings.get(target)
        if text is None:
            continue
        fn = containing(pc)
        if fn is None:
            continue
        per_function.setdefault(fn[0], []).append((target, text))
        hits += 1
    print(f"string xrefs        {hits}")
    print(f"functions w/ strings {len(per_function)}")

    rows = []
    for start, candidates in per_function.items():
        name = next((f[2] for s, e, f in functions if s == start), f"sub_{start:08x}")
        proposal = None
        for _addr, text in sorted(candidates):
            m = IDENT.match(text)
            if m and KNOWN_EXT.match(m.group(1)) and "_" in m.group(1) or (m and len(m.group(1)) > 8):
                proposal = m.group(1).lower()
                break
        if proposal is None:
            continue
        rows.append((start, name, proposal, candidates[0][1][:70]))

    args.out.parent.mkdir(parents=True, exist_ok=True)
    with args.out.open("w") as fh:
        fh.write("address\tcurrent\tproposed\tevidence\n")
        for start, name, proposal, text in sorted(rows):
            fh.write(f"0x{start:08x}\t{name}\t{proposal}\t{text}\n")
    print(f"proposals           {len(rows)}  -> {args.out}")

    lines = [
        "## Auto-proposed names (debug strings)",
        "",
        "Generated by `tools/name-from-strings.py` from literal-pool xrefs.",
        "`evidence` is the string the function references; review before trusting.",
        "",
        "| address | current | proposed | evidence |",
        "| --- | --- | --- | --- |",
    ]
    for start, name, proposal, text in sorted(rows)[:200]:
        lines.append(f"| `0x{start:08x}` | {name} | `{proposal}` | {text} |")
    if len(rows) > 200:
        lines.append(f"| … | | | {len(rows) - 200} more in analysis/labels.tsv |")

    md = args.md
    existing = md.read_text() if md.exists() else "# Address map\n"
    marker = "\n## Auto-proposed names (debug strings)"
    if marker in existing:
        existing = existing.split(marker)[0]
    md.write_text(existing.rstrip() + "\n\n" + "\n".join(lines) + "\n")
    print(f"map                 {md}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
