#!/usr/bin/env python3
"""Rewrite spec files into the one schema the readers actually implement.

Two silent-ignore shapes have cost real evidence: a system register nested under
`"special"` (both readers look at the top level) and an FPU register named
`"fr4"` inside `"regs"` (readers only accept `rN` there). Both made a spec seed
nothing while still passing. Rewriting is safe: the intended seed becomes real.
"""
import json
import pathlib
import re
import sys

SPECS = pathlib.Path(__file__).resolve().parent.parent / "notes" / "specs"
SPECIAL = {"sr", "mach", "macl", "gbr", "vbr", "ssr", "spc", "sgr", "dbr", "fpul", "fpscr"}


def normalize(doc: dict) -> bool:
    before = json.dumps(doc, sort_keys=True)
    if isinstance(doc.get("special"), dict):
        for key, value in doc.pop("special").items():
            doc.setdefault(key, value)
    regs = doc.get("regs")
    if isinstance(regs, dict):
        moved = {}
        for key in list(regs):
            m = re.fullmatch(r"fr(\d+)", key)
            if m:
                moved[key] = regs.pop(key)
        if moved:
            fr = doc.setdefault("fr", {})
            for key, value in moved.items():
                fr[f"f{re.fullmatch(r'fr(\d+)', key).group(1)}"] = value
    if isinstance(doc.get("fr"), dict):
        doc["fr"] = {re.sub(r"^fr(\d+)$", r"f\1", k): v for k, v in doc["fr"].items()}
    if isinstance(regs, dict):
        doc["regs"] = {k: v for k, v in regs.items() if k not in SPECIAL}
        # A top-level system register that was smuggled into regs belongs outside.
        for key in regs:
            if key in SPECIAL:
                doc.setdefault(key, regs[key])
    return json.dumps(doc, sort_keys=True) != before


def main(argv):
    paths = [pathlib.Path(p) for p in argv[1:]] or sorted(SPECS.glob("0x*.json"))
    changed = 0
    for p in paths:
        doc = json.loads(p.read_text())
        if normalize(doc):
            p.write_text(json.dumps(doc, indent=2) + "\n")
            changed += 1
    print(f"normalized {changed}/{len(paths)} specs")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
