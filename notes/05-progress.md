# Progress

Updated 2026-11-11.

| metric | value |
| --- | --- |
| functions exported | 5,211 |
| strings decoded | 10,983 (5,612 Ghidra string Data) |
| oracle coverage (static, mnemonic) | 96.67% of 969,383 instructions |
| functions auto-named | 310 (proposals, unreviewed) |
| oracle tests | 15 passing |
| systems green | 0 |

## Instruction gaps remaining (all < 0.08% each)

`pref`, `tst.b`, `mac.w`, `cmp/str`, `shal`, `mac.l`, `subc`, plus a long tail.
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
