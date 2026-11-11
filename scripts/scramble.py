#!/usr/bin/env python3
"""Katana 1ST_READ.BIN scrambler/descrambler (Marcus Comstedt's algorithm).

Ported from KallistiOS `utils/scramble/scramble.c`, which is Comstedt's tool.
The permutation is generated from the *file size*:

    seed = size & 0xffff
    rand(): seed = (seed * 2109 + 9273) & 0x7fff; return (seed + 0xc000) & 0xffff

Data is processed in 32-byte slices, in chunks that start at 2 MiB and halve
down to 32 bytes. For each chunk of n slices the index table is built with

    x = (rand() * i) >> 16      for i = n-1 .. 0, swapping idx[i] and idx[x]

Descrambling reads slice idx[i] and writes sequentially; scrambling reads
sequentially and writes slice idx[i]. They are exact inverses.

Usage:
    scramble.py --descramble IN OUT
    scramble.py --scramble   IN OUT
    scramble.py --selftest   [FILE]
"""
from __future__ import annotations

import pathlib
import sys

MAXCHUNK = 2048 * 1024
SLICE = 32


class Prng:
    __slots__ = ("seed",)

    def __init__(self, seed: int) -> None:
        self.seed = seed & 0xFFFF

    def rand(self) -> int:
        self.seed = (self.seed * 2109 + 9273) & 0x7FFF
        return (self.seed + 0xC000) & 0xFFFF


def permutation(prng: Prng, slices: int) -> list[int]:
    """Index table for one chunk: idx[i] is the slice written/read at i."""
    idx = list(range(slices))
    for i in range(slices - 1, -1, -1):
        x = (prng.rand() * i) >> 16
        idx[i], idx[x] = idx[x], idx[i]
    return idx


def _chunks(size: int):
    """Yield (offset, chunk_size, raw) in Comstedt's order.

    `raw` marks the final tail, which the C tool copies verbatim without
    building a permutation and without consuming the PRNG.
    """
    offset = 0
    remaining = size
    chunk = MAXCHUNK
    while chunk >= SLICE:
        while remaining >= chunk:
            yield offset, chunk, False
            offset += chunk
            remaining -= chunk
        chunk >>= 1
    if remaining:
        yield offset, remaining, True


def descramble(data: bytes) -> bytes:
    """Comstedt `scramble -d': out[idx[i]] = in[i] (permuted write)."""
    out = bytearray(len(data))
    prng = Prng(len(data))
    for offset, chunk, raw in _chunks(len(data)):
        if raw:
            out[offset:offset + chunk] = data[offset:offset + chunk]
            continue
        order = permutation(prng, chunk // SLICE)[::-1]
        for i, dst_slice in enumerate(order):
            src = offset + i * SLICE
            dst = offset + dst_slice * SLICE
            out[dst:dst + SLICE] = data[src:src + SLICE]
    return bytes(out)


def scramble(data: bytes) -> bytes:
    """Comstedt `scramble': out[i] = in[idx[i]] (permuted read)."""
    out = bytearray(len(data))
    prng = Prng(len(data))
    for offset, chunk, raw in _chunks(len(data)):
        if raw:
            out[offset:offset + chunk] = data[offset:offset + chunk]
            continue
        # The C tool walks i = n-1..0 while writing sequentially, so the
        # effective slice order is the reversed index table.
        order = permutation(prng, chunk // SLICE)[::-1]
        for i, src in enumerate(order):
            dst = offset + i * SLICE
            out[dst:dst + SLICE] = data[offset + src * SLICE:offset + src * SLICE + SLICE]
    return bytes(out)


def selftest(path: pathlib.Path | None) -> int:
    import hashlib

    payload = path.read_bytes() if path else bytes(range(256)) * 4099
    scrambled = scramble(payload)
    restored = descramble(scrambled)
    ok_roundtrip = restored == payload
    ok_scrambled_differs = scrambled != payload
    print(f"payload          {len(payload)} bytes  sha256={hashlib.sha256(payload).hexdigest()[:16]}")
    print(f"scrambled        differs={ok_scrambled_differs}")
    print(f"descramble(scr)  round-trip={'ok' if ok_roundtrip else 'MISMATCH'}")
    print(f"scramble(scr)    round-trip={'ok' if scramble(scrambled) == payload else 'MISMATCH'}")
    # A scrambled boot binary must not keep the Katana magic at offset 0.
    magic = payload[:16] == b"SEGA SEGAKATANA "
    print(f"magic at 0       original={magic} scrambled={scrambled[:16] == b'SEGA SEGAKATANA '}")
    return 0 if (ok_roundtrip and ok_scrambled_differs) else 1


def main() -> int:
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return 2
    mode = args[0]
    if mode == "--selftest":
        path = pathlib.Path(args[1]) if len(args) > 1 else None
        return selftest(path)
    if mode not in ("--descramble", "--scramble") or len(args) != 3:
        print(__doc__)
        return 2
    src, dst = pathlib.Path(args[1]), pathlib.Path(args[2])
    data = src.read_bytes()
    out = descramble(data) if mode == "--descramble" else scramble(data)
    dst.write_bytes(out)
    print(f"{mode[2:]:<10} {src} -> {dst}  ({len(data)} bytes)")
    if dst.read_bytes()[:16] == b"SEGA SEGAKATANA ":
        print("  Katana magic present after transform")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
