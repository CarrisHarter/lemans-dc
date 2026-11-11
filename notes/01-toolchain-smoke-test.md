# SH-4 toolchain smoke test — result

Date: 2026-11-11
Ghidra: 12.1.4 PUBLIC (ghidra_12.1.4_PUBLIC_20260921.zip,
sha256 `ddac49f903da9d5bac833e5cc79395098b9c33cfd3279be5f31bd00387d2d4db`, matches release notes)
JDK: Temurin 21.0.13+9 (`~/opt/jdk-21.0.13+9`,
sha256 `d7f5d5cf865a2902b70ff549b97586a7d3f234e016f041ab447ea740c7d66d1c`, matches Adoptium API)

## Correction to the plan

`SuperH:LE:32:SH-4` does **not** exist in Ghidra 12.x. That module was renamed.

| what | correct value |
| --- | --- |
| processor module | `Ghidra/Processors/SuperH4` (separate from `SuperH`, which is BE SH-1/SH-2/SH-2A only) |
| language id | `SuperH4:LE:32:default` |
| compiler spec | `default` (also `windows`) |
| BE variant | `SuperH4:BE:32:default` |

## Proof

`scripts/make_sh4_fixture.py` emits 56 hand-assembled bytes mapped at `0x8C010000`.
Headless run (`scripts/06-headless-smoke.sh`) with
`-processor SuperH4:LE:32:default -cspec default -loader BinaryLoader -loader-baseAddr 0x8C010000`:

- decompiler native present: `Ghidra/Features/Decompiler/os/linux_x86_64/decompile`
- 23 instructions decoded from 56 bytes; FPU decodes (`fmul fr4,fr0`, `fmov.s fr4,@r0`)
- PC-relative literal resolved: `mov.l 0x8c010020,r1`
- indirect call typed: `(*(code *)PTR_entry_8c010000_8c010020)(5);`
- loop decompiled correctly:
  `do { iVar1 = iVar1 + 1; in_r1 = in_r1 + -1; } while (in_r1 != 0);`

Note: Ghidra prints SH-4 operands GNU-style (`mov r1,r4` = r4 <- r1).

## Consequences for the real target

1. Raw loader creates **no entry point**, and auto-analysis creates **0 functions**
   even after a sweep. Two functions existed only because the script named them.
   The real `1ST_READ.BIN` will need explicit seeding (`-postScript` seeding or
   `REA_GHIDRA_SEED_FILE`) at the Katana entry (`0x8C010100`) and at call targets.
2. `imageBase` stays `0x00000000` while the block lives at `0x8C010000`
   (`block ram 0x8c010000-0x8c010037`), so all address bookkeeping is block-relative.
3. rea's raw-binary lane must be patched to emit `SuperH4:LE:32:default`
   (step 9), and its architecture name should be the real one, e.g. `sh4`.
