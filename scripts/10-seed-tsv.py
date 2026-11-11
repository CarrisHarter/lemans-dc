#!/usr/bin/env python3
r"""Generate a rea/Ghidra seed TSV for a raw SH-4 image.

rea applies `REA_GHIDRA_SEED_FILE` before auto-analysis, which is the only way
to reach code that no static reference points at: computed jumps, jump tables,
function pointers stored in data, and vtable targets.

Format (one line each, '#' comments):
    address<TAB>kind[<TAB>name]      kind = function | code | label

Rules enforced here, because a bad seed aborts the whole import:
  * addresses are 0x hex inside the mapped range
  * SH-4 instructions are 16-bit, so function/code seeds must be even
  * labels need a name and must match [A-Za-z_][\w.]*

Input is a JSON hotlist, so the same list can carry notes and later become the
address->purpose ledger:

    {"seeds": [
       {"address": "0x8c010100", "kind": "function", "name": "boot_entry"},
       {"address": "0x8c012340", "kind": "code", "why": "jump table target"}
    ]}
"""
from __future__ import annotations

import json
import pathlib
import re
import sys

NAME_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_.]*$")
KINDS = ("function", "code", "label")


def build(hotlist: pathlib.Path, base: int, size: int) -> str:
    data = json.loads(hotlist.read_text())
    entries = data.get("seeds")
    if not isinstance(entries, list):
        raise SystemExit(f"{hotlist}: expected a 'seeds' array")

    lines = [f"# generated from {hotlist.name}; base=0x{base:x} size=0x{size:x}"]
    seen: set[tuple[int, str]] = set()
    for index, entry in enumerate(entries):
        address_text = entry.get("address")
        kind = entry.get("kind")
        name = entry.get("name")
        if not isinstance(address_text, str):
            raise SystemExit(f"seed {index}: address must be a string")
        address = int(address_text, 16)
        if kind not in KINDS:
            raise SystemExit(f"seed {index}: kind must be one of {KINDS}")
        if not (base <= address < base + size):
            raise SystemExit(
                f"seed {index}: 0x{address:x} is outside 0x{base:x}..0x{base + size:x}"
            )
        if kind in ("function", "code") and address % 2:
            raise SystemExit(f"seed {index}: 0x{address:x} is odd; SH-4 fetches 16-bit words")
        if kind == "label" and (not isinstance(name, str) or not NAME_RE.match(name)):
            raise SystemExit(f"seed {index}: a label needs a valid name")
        if (address, kind) in seen:
            continue
        seen.add((address, kind))
        row = f"0x{address:x}\t{kind}"
        if kind == "label":
            row += f"\t{name}"
        lines.append(row)
    return "\n".join(lines) + "\n"


def main() -> int:
    if len(sys.argv) < 4:
        print(__doc__)
        return 2
    hotlist = pathlib.Path(sys.argv[1]).expanduser()
    base = int(sys.argv[2], 16)
    image = pathlib.Path(sys.argv[3]).expanduser()
    text = build(hotlist, base, image.stat().st_size)
    out = hotlist.with_suffix(".tsv")
    out.write_text(text)
    rows = sum(1 for line in text.splitlines() if line and not line.startswith("#"))
    print(f"wrote {out} ({rows} seeds)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
