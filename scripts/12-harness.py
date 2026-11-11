#!/usr/bin/env python3
"""Functional-equivalence harness for recovered SH-4 C.

Two modes, with deliberately different strength:

  host <file.c> --main <driver.c>
      Compile the recovered C for the host and run it against a driver that
      asserts known results. Valid for integer/pointer/control-flow code, which
      is most of a loader, allocator or fixed-step integrator. Says nothing about
      float rounding or calling-convention details.

  sh4 <file.c>
      Compile for SH-4 (-ml -m4-single-only) and inspect the generated code:
      little-endian ELF32/SH, single-precision FPU registers (fr, never dr),
      argument registers r4..r7, result in r0. Catching these here means a
      recovered function that silently changed precision mode or ABI never gets
      marked as verified.

Neither mode executes SH-4 code: bare-metal sh-elf ELF does not run under
qemu-sh4-static (that needs an SH-4 Linux ABI binary). Execution of the real
target belongs to an emulator, not to a unit harness.
"""
from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys
import tempfile

FPU_TRAPS = [
    ("fmac", "vector-multiply-accumulate rounding: one rounding step, not two"),
    ("fipr", "inner-product rounding differs from a scalar loop"),
    ("ftrv", "transform rounding differs from four scalar dots"),
    ("fschg", "changes FPSCR.SZ (64-bit moves); moves after it use dr pairs"),
    ("flds", "single-precision load; pairing with fld changes mode"),
]

PASS, FAIL, WARN = "ok", "FAIL", "warn"


def run(cmd: list[str], **kw) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, capture_output=True, text=True, **kw)


def check_present(names: list[str]) -> list[str]:
    return [n for n in names if run(["which", n]).returncode != 0]


def host_mode(source: pathlib.Path, driver: pathlib.Path, cc: str) -> int:
    with tempfile.TemporaryDirectory() as tmp:
        binary = pathlib.Path(tmp) / "harness"
        build = run([cc, "-O2", "-Wall", "-Wextra", str(source), str(driver), "-o", str(binary)])
        if build.returncode != 0:
            print(build.stderr.strip())
            print(f"{FAIL} compile")
            return 1
        print(f"{PASS} compile (host {cc})")
        if build.stderr.strip():
            print(f"{WARN} warnings:\n{build.stderr.strip()}")
        result = run([str(binary)])
        print(result.stdout.rstrip() or "(no output)")
        if result.returncode != 0:
            print(result.stderr.strip())
            print(f"{FAIL} driver reported mismatch (exit {result.returncode})")
            return 1
        print(f"{PASS} all vectors matched on host")
        return 0


def sh4_mode(source: pathlib.Path, gcc: str, objdump: str) -> int:
    # The stage-1 driver has no target program prefix, so it looks for the host
    # `as`. -B points it at a directory of target-named tool symlinks.
    toolbin = pathlib.Path.home() / "opt/sh-elf/toolbin"
    extra = [f"-B{toolbin}"] if toolbin.is_dir() else []
    with tempfile.TemporaryDirectory() as tmp:
        obj = pathlib.Path(tmp) / "out.o"
        build = run([gcc, *extra, "-ml", "-m4-single-only", "-O2", "-Wall", "-c", str(source), "-o", str(obj)])
        if build.returncode != 0:
            print(build.stderr.strip())
            print(f"{FAIL} sh-elf-gcc compile")
            return 1
        print(f"{PASS} compile (sh-elf-gcc -ml -m4-single-only)")

        header = run([objdump, "-f", str(obj)]).stdout
        machine_ok = "SH" in header or "sh" in header
        print(f"{PASS if machine_ok else FAIL} object machine type SH")
        if not machine_ok:
            print(header)

        dis = run([objdump, "-d", "-EL", str(obj)]).stdout
        mnemonic = re.compile(r"^\s+[0-9a-f]+:\s+[0-9a-f ]+\t(\w+)", re.M)
        mnemonics = mnemonic.findall(dis)
        dr_use = re.findall(r"\bdr\d+\b", dis)
        fr_use = re.findall(r"\bfr\d+\b", dis)

        precision_ok = bool(fr_use) and not dr_use
        print(f"{PASS if precision_ok else WARN} single-precision mode (fr={len(fr_use)}, dr={len(dr_use)})")
        if dr_use:
            print(f"  dr registers present: {sorted(set(dr_use))[:8]} — precision mode drifted")
        if not fr_use:
            print("  no hardware FPU instructions: built without -m4-single-only (soft float)")

        args = [r for r in ("r4", "r5", "r6", "r7") if re.search(rf"\b{r}\b", dis)]
        print(f"{PASS if args else WARN} argument registers used: {', '.join(args) or 'none'}")
        print(f"{PASS if re.search(r'\br0\b', dis) else WARN} r0 used as result register")

        for name, why in FPU_TRAPS:
            if re.search(rf"\b{name}\b", dis):
                print(f"{WARN} {name} present — {why}")

        print("\n--- disassembly ---")
        print(dis.strip()[:4000])
        return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest="mode", required=True)

    h = sub.add_parser("host", help="compile and run against vectors")
    h.add_argument("source", type=pathlib.Path)
    h.add_argument("--main", type=pathlib.Path, required=True)
    h.add_argument("--cc", default="gcc")

    s = sub.add_parser("sh4", help="compile for SH-4 and inspect the code")
    s.add_argument("source", type=pathlib.Path)
    s.add_argument("--gcc", default=str(pathlib.Path.home() / "opt/sh-elf/bin/sh-elf-gcc"))
    s.add_argument("--objdump", default="sh4-elf-objdump")

    args = parser.parse_args()

    missing = check_present(["sh4-elf-objdump"]) if args.mode == "sh4" else []
    if missing:
        print(f"{FAIL} missing tools: {', '.join(missing)}", file=sys.stderr)
        return 1

    if args.mode == "host":
        return host_mode(args.source, args.main, args.cc)
    return sh4_mode(args.source, args.gcc, args.objdump)


if __name__ == "__main__":
    raise SystemExit(main())
