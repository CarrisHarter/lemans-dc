Empty until a real disc image is supplied.

Populated by `scripts/04-extract-1stread.py`, which writes `1st_read.bin`,
`ip.bin` and `header.json`. The synthetic-disc outputs used for regression live
in `fixtures/fake-disc/expected/` instead, so they can never be mistaken for a
real extraction.
