#!/usr/bin/env python3
"""Randomized differential audit: every registered port vs the original bytes.

The hand-written specs prove the paths their author thought of. This audit
attacks the other direction: random inputs over a dense pool of memory, run
through both the oracle and the port, comparing exactly what the verifier
compares (r0-r15, sr/T, fr, every store outside the synthetic stack).

A case is *invalid* (skipped, not a failure) when the original itself faults or
exhausts the step budget — random data is not a valid program state, and a port
has no way to model an unmodelled hardware fault. A case where the original
halts cleanly and the port disagrees is a real suspect: the spec that broke it
is kept under /tmp/lm-fuzz/ for replay through verify-function.sh.

Usage:
  fuzz-audit.py [--cases N] [--jobs N] [--only 0x8c0b92f4,0x8c0112f4]
                [--port-root DIR] [--keep-going]

Exit status 1 if any suspect was found, 0 if every case passed or was invalid.
"""

import argparse
import concurrent.futures as futures
import hashlib
import json
import os
import pathlib
import random
import subprocess
import sys

ROOT = pathlib.Path(os.environ.get("LEMANS_ROOT", pathlib.Path.home() / "Projects/lemans-dc"))
BIN = ROOT / "analysis" / "lemans24.bin"
ORACLE = ROOT / "rust" / "sh4-oracle" / "target" / "release" / "sh4-oracle"
VERIFY = ROOT / "rust" / "lemans-verify" / "target" / "release" / "lemans-verify"
PORTS = ROOT / "rust" / "lemans-verify" / "src" / "ports"
SCRATCH = pathlib.Path("/tmp/lm-fuzz")

POOL = 0x8C200000
POOL_SIZE = 0x8000
STACK = 0x8C300000
STACK_TOP = 0x8C308000
STEPS = 20000

SMALL = [0, 1, 2, 3, 4, 7, 8, 16, 31, 32, 46, 64, 0xFF, 0x100, 0x7FFFFFFF, 0xFFFFFFFF]


def spec_for(addr: int, case: int) -> dict:
    """Deterministic pseudo-random seed for (address, case)."""
    h = hashlib.sha256(f"{addr:08x}:{case}".encode()).digest()
    rng = random.Random(int.from_bytes(h[:8], "little"))

    def ptr() -> int:
        # 1/2/4-byte aligned pointer inside the dense pool
        return POOL + rng.randrange(0, POOL_SIZE - 16, rng.choice((1, 2, 4)))

    def arg() -> int:
        r = rng.random()
        if r < 0.62:
            return ptr()
        if r < 0.92:
            return rng.choice(SMALL)
        return rng.randrange(0, 0x10000)

    # Fill modes: uniform random data dereferences as wild pointers, so most
    # cases fault in the original before the port is exercised. Structured
    # fills reach deeper paths and raise the valid-case rate.
    mode = rng.choice(("random", "zeros", "ones", "ptr-dense", "ptrs"))
    if mode == "random":
        pool = bytes(rng.randrange(256) for _ in range(POOL_SIZE))
    elif mode == "zeros":
        pool = bytes(POOL_SIZE)
    elif mode == "ones":
        pool = b"\xff" * POOL_SIZE
    else:
        words = []
        while len(words) * 4 < POOL_SIZE:
            if mode == "ptrs" and rng.random() < 0.5:
                words.append(rng.choice(SMALL))
            else:
                words.append(POOL + rng.randrange(0, POOL_SIZE - 16, 4))
        pool = b"".join(w.to_bytes(4, "little") for w in words)[:POOL_SIZE]
    # Stack: random pool pointers, so functions reading caller args get valid data.
    stack = b"".join(ptr().to_bytes(4, "little") for _ in range(0x400))

    spec = {
        "entry": f"0x{addr:08x}",
        "regs": {f"r{i}": arg() for i in range(15)},
        "fr": {f"f{i}": rng.uniform(0.5, 1000.0) for i in range(16)},
        "memory": [
            {"addr": f"0x{POOL:08x}", "hex": pool.hex()},
            {"addr": f"0x{STACK_TOP:08x}", "hex": stack.hex()},
        ],
        "steps": STEPS,
    }
    spec["regs"]["r15"] = f"0x{STACK_TOP:08x}"
    if rng.random() < 0.5:
        spec["sr"] = 1  # exercise the T-flag entry state (top level, as the readers expect)
    return spec


def run_case(addr: int, case: int) -> tuple[str, dict | None]:
    spec = spec_for(addr, case)
    SCRATCH.mkdir(exist_ok=True)
    path = SCRATCH / f"0x{addr:08x}.{case}.json"
    path.write_text(json.dumps(spec, indent=2) + "\n")
    try:
        proc = subprocess.run(
            [str(VERIFY), str(BIN), f"0x{addr:08x}", str(path), "--oracle", str(ORACLE)],
            capture_output=True, text=True, timeout=60,
        )
    except subprocess.TimeoutExpired:
        # The oracle halted (otherwise the verifier would have said so), so a
        # port that still runs after a minute is looping: a real bug, not an
        # invalid input.
        return "hang", {"spec": str(path), "output": "port did not return within 60s"}
    out = (proc.stdout + proc.stderr).strip()
    if proc.returncode == 0:
        path.unlink(missing_ok=True)
        return "pass", None
    if "oracle did not halt cleanly" in out or "oracle exited" in out:
        path.unlink(missing_ok=True)
        return "invalid", None
    if "bad spec" in out:
        # A malformed generated spec is a bug in this script, never a finding.
        raise SystemExit(f"fuzz-audit generated an invalid spec: {out.splitlines()[-1]}")
    if "no port registered" in out:
        path.unlink(missing_ok=True)
        return "invalid", None
    return "suspect", {"spec": str(path), "output": out}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--cases", type=int, default=25)
    ap.add_argument("--jobs", type=int, default=max(2, (os.cpu_count() or 4) - 1))
    ap.add_argument("--only", default="")
    ap.add_argument("--port-root", type=pathlib.Path, default=PORTS)
    ap.add_argument("--max-suspects", type=int, default=0, help="stop after N suspects (0 = never)")
    args = ap.parse_args()

    if args.only:
        addresses = [int(a, 16) for a in args.only.split(",") if a.strip()]
    else:
        addresses = sorted(int(p.stem[2:], 16) for p in args.port_root.glob("p_*.rs"))

    todo = [(a, c) for a in addresses for c in range(args.cases)]
    tally = {"pass": 0, "invalid": 0, "suspect": 0, "hang": 0}
    suspects: list[tuple[int, dict]] = []
    with futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        pending = {pool.submit(run_case, a, c): (a, c) for a, c in todo}
        for fut in futures.as_completed(pending):
            addr, case = pending[fut]
            try:
                kind, detail = fut.result()
            except subprocess.TimeoutExpired:
                kind, detail = "hang", {"spec": "unknown", "output": "outer timeout"}
            tally[kind] += 1
            if kind in ("suspect", "hang"):
                suspects.append((addr, detail))
                print(f"SUSPECT 0x{addr:08x} case {case}: {detail['output'].splitlines()[0][:120]}",
                      flush=True)
                print(f"        spec kept: {detail['spec']}", flush=True)
                if args.max_suspects and len(suspects) >= args.max_suspects:
                    for p in pending:
                        p.cancel()
                    break

    print(f"\ncases {len(todo)}  pass {tally['pass']}  invalid {tally['invalid']}  "
          f"suspect {tally['suspect']}  hang {tally['hang']}  ports {len(addresses)}")
    if suspects:
        print("\nsuspects (replay with scripts/verify-function.sh <addr> analysis/lemans24.bin <spec>):")
        seen = set()
        for addr, detail in suspects:
            if addr in seen:
                continue
            seen.add(addr)
            print(f"  0x{addr:08x}  {detail['spec']}")
    return 1 if suspects or tally["hang"] else 0


if __name__ == "__main__":
    sys.exit(main())
