# Progress

Updated 2026-11-11.

| metric | value |
| --- | --- |
| functions exported | 5,211 |
| strings decoded | 10,983 (5,612 Ghidra string Data) |
| oracle coverage (static, mnemonic) | 99.70% of 969,383 instructions |
| functions auto-named | 310 (proposals, unreviewed) |
| oracle tests | 50 passing |
| systems green | 0 |

## Instruction gaps remaining

`div1` (1,001 instructions, 0.10%) and the vector FPU set (`ftrv`, `fipr`,
`frchg`, `fschg`, `fsca`, `fsrra`, `fcnvsd`; 303 total). Both are left
`Unimplemented` on purpose: `div1`'s non-restoring step could not be validated
against a trusted division sequence, and the vector instructions need the
matrix-register model. A wrong oracle is worse than a loud gap.
Added this session: full `mov.b`/`mov.w`/`mov.l` load/store matrix (all addressing
forms), `tst`/`and`/`or`/`xor`, `cmp/eq|hs|ge|hi|gt|pl|pz`, `extu`/`exts`, all
shifts, `bra`/`bsr`/`mova`, `stc`/`ldc`, `not`/`neg`, and FPU `fcmp`, `float`,
`ftrc`, `flds`/`fsts` with a real FPUL register.

## Named from evidence

`analysis/labels.tsv` — 310 proposals from Ghidra string labels. Engine shows
C++ RTTI names (`ReflectionMapping`, `GOKART`, exception typeinfo), which is a
second naming channel beyond the debug strings.

## Next

1. Extend the oracle with the gap list above; re-run the census until >99%.
2. Resolve the top debug strings to functions through a live Ghidra session.
3. Start the spec + verify loop on the 310 named functions.
