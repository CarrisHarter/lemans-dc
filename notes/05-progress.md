# Progress

Updated 2026-11-11.

| metric | value |
| --- | --- |
| functions exported | 5,211 |
| strings decoded | 10,983 (5,612 Ghidra string Data) |
| oracle coverage (static, mnemonic) | 79.6% of 969,383 instructions |
| functions auto-named | 310 (proposals, unreviewed) |
| systems green | 0 |

## Instruction gaps, by static frequency

`mov.b` 3.76%, `tst` 1.85%, `mov.w` 1.83%, `bra` 1.61%, `extu.b` 1.17%,
`cmp/eq` 0.81%, `mova` 0.67%, `lds.l` 0.62%, `sts.l` 0.59%, `shll2` 0.51%,
`bsr` 0.50%, `and` 0.36%. Add these plus indexed addressing before porting.

## Named from evidence

`analysis/labels.tsv` — 310 proposals from Ghidra string labels. Engine shows
C++ RTTI names (`ReflectionMapping`, `GOKART`, exception typeinfo), which is a
second naming channel beyond the debug strings.

## Next

1. Extend the oracle with the gap list above; re-run the census until >99%.
2. Resolve the top debug strings to functions through a live Ghidra session.
3. Start the spec + verify loop on the 310 named functions.
