#!/usr/bin/env python3
"""Read files out of a Dreamcast GD-ROM image that spans several tracks.

Why this exists: GD-ROM filesystems use disc-absolute LBAs while each track is a
separate .bin, and the high-density area can have audio tracks in the middle
(track 3 = data, track 4 = audio, track 5 = data). A single-track ISO conversion
therefore cannot resolve half the extents.

Usage:
    discfs.py <image.cue> --list
    discfs.py <image.cue> --extract NAME [NAME ...] --out DIR
"""
from __future__ import annotations

import argparse
import pathlib
import struct
import sys

SECTOR_RAW = 2352
SECTOR_DATA = 2048
DATA_OFFSET = 16
HIGH_DENSITY_LBA = 45000


class Track:
    def __init__(self, path: pathlib.Path, mode: str, index01: int):
        self.path = path
        self.mode = mode
        self.index01 = index01
        self.start_lba = 0
        self.sectors = path.stat().st_size // SECTOR_RAW if path.exists() else 0

    @property
    def is_data(self) -> bool:
        return self.mode.startswith("MODE1") or self.mode.startswith("MODE2")

    def read_sector(self, index: int) -> bytes:
        # `index` is relative to the start of the LBA range assigned to this
        # track, which already includes the INDEX 01 pregap sectors.
        with self.path.open("rb") as fh:
            fh.seek(index * SECTOR_RAW)
            raw = fh.read(SECTOR_RAW)
        return raw[DATA_OFFSET:DATA_OFFSET + SECTOR_DATA]


def parse_cue(cue: pathlib.Path) -> list[Track]:
    tracks: list[Track] = []
    current_file = None
    for line in cue.read_text(errors="replace").splitlines():
        line = line.strip()
        if line.startswith("FILE "):
            a, b = line.find('"'), line.rfind('"')
            current_file = cue.parent / line[a + 1:b]
        elif line.startswith("TRACK ") and current_file is not None:
            parts = line.split()
            tracks.append(Track(current_file, parts[2], 0))
        elif line.startswith("INDEX 01") and tracks:
            mm, ss, ff = line.split()[2].split(":")
            tracks[-1].index01 = int(mm) * 60 * 75 + int(ss) * 75 + int(ff)

    # Assign start LBAs: the first data track of the high-density area is pinned
    # at 45000; every later track continues from the previous track's end.
    seen_low = False
    for track in tracks:
        if not seen_low:
            track.start_lba = 0
            seen_low = track.is_data
            if track.start_lba == 0 and track.is_data:
                track.start_lba = 0
        else:
            track.start_lba = tracks[tracks.index(track) - 1].start_lba + \
                              tracks[tracks.index(track) - 1].sectors

    # The audio tracks between the single- and high-density areas shift the
    # high-density start to the fixed 45000 boundary.
    for track in tracks:
        if track.is_data and track.start_lba < HIGH_DENSITY_LBA and seen_low:
            pass
    high = next((t for t in tracks if t.is_data and t.start_lba != 0), None)
    if high is not None:
        high.start_lba = HIGH_DENSITY_LBA
        idx = tracks.index(high)
        for t in tracks[idx + 1:]:
            prev = tracks[tracks.index(t) - 1]
            t.start_lba = prev.start_lba + prev.sectors
    return tracks


def find_track(tracks: list[Track], lba: int) -> Track:
    for t in tracks:
        if t.start_lba <= lba < t.start_lba + t.sectors:
            return t
    raise SystemExit(f"LBA {lba} is not inside any data track")


def read_span(tracks: list[Track], lba: int, size: int) -> bytes:
    out = bytearray()
    while len(out) < size:
        t = find_track(tracks, lba)
        out += t.read_sector(lba - t.start_lba)
        lba += 1
    return bytes(out[:size])


def walk(tracks: list[Track], base: int) -> dict[str, tuple[int, int]]:
    pvd = read_span(tracks, base + 16, SECTOR_DATA)
    root = pvd[156:190]
    extent = struct.unpack("<I", root[2:6])[0]
    size = struct.unpack("<I", root[10:14])[0]
    files: dict[str, tuple[int, int]] = {}
    stack = [(extent, size)]
    while stack:
        extent, size = stack.pop()
        data = read_span(tracks, extent, size)
        i = 0
        while i < len(data):
            rec_len = data[i]
            if rec_len == 0:
                i = ((i // SECTOR_DATA) + 1) * SECTOR_DATA
                continue
            rec = data[i:i + rec_len]
            i += rec_len
            name_len = rec[32]
            name = rec[33:33 + name_len]
            if name in (b"\x00", b"\x01"):
                continue
            flags = rec[25]
            entry_extent = struct.unpack("<I", rec[2:6])[0]
            entry_size = struct.unpack("<I", rec[10:14])[0]
            text = name.decode("ascii", "replace").split(";")[0]
            if flags & 0x02:
                stack.append((entry_extent, entry_size))
            else:
                files[text.upper()] = (entry_extent, entry_size)
    return files


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("cue", type=pathlib.Path)
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--extract", nargs="*", default=[])
    ap.add_argument("--out", type=pathlib.Path)
    args = ap.parse_args()

    tracks = parse_cue(args.cue)
    for t in tracks:
        kind = "data" if t.is_data else "audio"
        print(f"  {t.path.name:<46} {kind:<5} start={t.start_lba:<7} sectors={t.sectors}")

    files = walk(tracks, HIGH_DENSITY_LBA)
    if args.list or not args.extract:
        for name in sorted(files):
            print(f"  {files[name][1]:>10}  {name}")
        return 0

    out = args.out or pathlib.Path(".")
    out.mkdir(parents=True, exist_ok=True)
    for wanted in args.extract:
        key = pathlib.Path(wanted).name.upper()
        if key not in files:
            print(f"missing: {wanted}", file=sys.stderr)
            continue
        extent, size = files[key]
        blob = read_span(tracks, extent, size)
        target = out / pathlib.Path(wanted).name
        target.write_bytes(blob)
        print(f"  {target}  {size} bytes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
