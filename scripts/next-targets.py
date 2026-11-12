#!/usr/bin/env python3
"""Rank unported functions that a single-function spec can actually pin.

A port is only verifiable when the original's control flow stays inside the
function: every `jsr`/`jmp @rN` must resolve to a target that is already ported
(or to a literal we can name), and no target may be loaded from memory we
cannot predict. This walks the disassembly, resolves the register targets that
come from literal pools and immediates, and lists candidates ascending.

Usage:
  next-targets.py [--limit N] [--max-insns N] [--all]
      --limit      how many candidates to print (default 12)
      --max-insns  skip long functions (default 400)
      --all        include functions whose callees are unported
"""

import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SYMBOLS = ROOT / "analysis" / "export" / "symbols.tsv"
ASM = ROOT / "analysis" / "export" / "disassembly.asm"
PORTS = ROOT / "rust" / "lemans-verify" / "src" / "ports"

ADDR_RE = re.compile(r"^([0-9a-f]{8}):\s+[0-9a-f]{2} [0-9a-f]{2}\s+(\S+)(?:\s+(.*))?$")
LITERAL_RE = re.compile(r"!\s*([0-9a-f]{8})\b")
DIRECT_JS = {"bsr": "call", "bra": "jump", "bt": "cond", "bf": "cond",
             "bt.s": "cond", "bf.s": "cond"}
INDIRECT = {"jsr": "call", "jmp": "jump"}
WRITES_RN = re.compile(r"^mov\.l\s+0x[0-9a-f]+,\s*(r\d+)\s*!?\s*([0-9a-f]{8})?$")


def load_symbols():
    out = []
    for line in SYMBOLS.read_text().splitlines()[1:]:
        parts = line.split("\t")
        if len(parts) < 5:
            continue
        addr = int(parts[0], 16)
        ranges = []
        for piece in re.split(r"[;,\s]+", parts[4].strip()):
            if "-" in piece and piece.startswith("0x"):
                a, b = piece.split("-")
                ranges.append((int(a, 16), int(b, 16)))
        out.append((addr, parts[1], ranges))
    return out


def load_code():
    """address -> (mnemonic, operands, comment)"""
    code = {}
    for line in ASM.read_text(errors="replace").splitlines():
        m = ADDR_RE.match(line)
        if not m:
            continue
        rest = m.group(3) or ""
        if "!" in rest:
            ops, comment = rest.split("!", 1)
            comment = comment.strip()
        else:
            ops, comment = rest, ""
        code[int(m.group(1), 16)] = (m.group(2), ops.strip(), comment)
    return code


def resolve_call(insns, idx, reg):
    """Value of `reg` at the call site, from the nearest literal/immediate load."""
    for j in range(idx - 1, max(-1, idx - 8), -1):
        _, mnem, ops, comment = insns[j]
        if mnem == "mov.l" and ops.endswith(reg):
            if comment:
                return int(comment.split()[0], 16), "literal"
            return None, "literal-unknown"
        if mnem == "mov" and ops.startswith("#") and ops.endswith(reg):
            return int(ops[1:].split(",")[0], 16), "immediate"
        if mnem in ("mov", "add", "mov.l", "mova") and ops.endswith(reg):
            return None, "computed"
    return None, "unknown"


def analyse(addr, ranges, code):
    insns = []
    for a, b in ranges:
        for pc in range(a, b + 1, 2):
            if pc in code:
                insns.append((pc, *code[pc]))
    callees, hazards, unresolved = set(), [], []
    for i, (pc, mnem, ops, comment) in enumerate(insns):
        if mnem in INDIRECT and ops.startswith("@"):
            reg = ops[1:].strip()
            target, how = resolve_call(insns, i, reg)
            if target is None:
                hazards.append(f"0x{pc:08x} {mnem} {ops} ({how})")
            elif ranges[0][0] <= target <= ranges[-1][1]:
                continue  # self-recursion / internal jump
            else:
                callees.add(target)
        elif mnem in DIRECT_JS:
            target = None
            if ops.startswith("0x"):
                target = int(ops.split(",")[0], 16)
            elif comment.startswith("8c"):
                target = int(comment.split()[0], 16)
            if target is None:
                continue
            inside = any(a <= target <= b for a, b in ranges)
            if mnem == "bsr" and not inside:
                callees.add(target)
            elif mnem in ("bra",) and not inside:
                callees.add(target)  # tail call
    return callees, hazards, len(insns)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=12)
    ap.add_argument("--max-insns", type=int, default=400)
    ap.add_argument("--all", action="store_true")
    args = ap.parse_args()

    ports = {int(p.stem[2:], 16) for p in PORTS.glob("p_*.rs")}
    blocked_text = (ROOT / "notes" / "address-map.md").read_text()
    blocked = {int(a, 16) for a in re.findall(
        r"^\| `0x([0-9a-f]{8})`", blocked_text.split("## Blocked", 1)[1].split("\n## ", 1)[0], re.M)}

    symbols = load_symbols()
    code = load_code()
    by_addr = {a: (n, r) for a, n, r in symbols}

    rows = []
    for addr, name, ranges in sorted(symbols):
        if not ranges or addr in ports or addr in blocked:
            continue
        callees, hazards, n = analyse(addr, ranges, code)
        unported = sorted(c for c in callees if c not in ports)
        rows.append((addr, name, n, hazards, unported, sorted(callees)))

    ready, pending = [], []
    for addr, name, n, hazards, unported, callees in rows:
        entry = dict(addr=addr, name=name, insns=n, hazards=hazards,
                     unported=unported, callees=callees)
        if not hazards and not unported:
            ready.append(entry)
        elif not hazards and unported and args.all:
            pending.append(entry)

    def show(items, title):
        print(f"# {title}: {len(items)}")
        shown = 0
        for e in items:
            if e["insns"] > args.max_insns:
                continue
            print(f"0x{e['addr']:08x}  insns {e['insns']:4d}  callees "
                  f"{' '.join(f'0x{c:08x}' for c in e['callees']) or '-'}"
                  f"{'  MISSING ' + ' '.join(f'0x{c:08x}' for c in e['unported']) if e['unported'] else ''}")
            shown += 1
            if shown >= args.limit:
                break
        print()

    show(ready, "ready (no unresolved callee, no indirect hazard)")
    if pending:
        show(pending, "waiting on callees")

    small = [e for e in ready if e["insns"] <= args.max_insns]
    print(f"# smallest ready: {len(small)} under {args.max_insns} insns; "
          f"median {sorted(e['insns'] for e in small)[len(small)//2] if small else 0}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
