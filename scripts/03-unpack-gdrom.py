#!/usr/bin/env python3
"""Step 3: unpack a Dreamcast GD-ROM image and list the ISO9660 tree.

Handles the two shapes that actually show up:
  * .gdi  -> text descriptor + trackNN.bin/raw files; the data track is the
             large MODE1/2352 one (usually track03.bin).
  * .iso  -> already a 2048-byte-per-sector ISO, used directly.
  * .cdi  -> refused with instructions (needs a CDI-aware tool).

MODE1/2352 -> ISO conversion is done inline: per 2352-byte sector, bytes
16..2064 are the 2048-byte user data. No external binaries required.
"""
from __future__ import annotations

import hashlib
import pathlib
import struct
import sys

SECTOR_RAW = 2352
SECTOR_DATA = 2048
MODE1_DATA_OFFSET = 16

def detect_lba_base(iso: bytes, root_extent: int) -> int:
    """GD-ROM filesystems use disc-absolute LBAs; a track extracted on its own
    starts at 0. Probe candidate bases and accept the one where the root
    directory record actually parses."""
    for base in (0, 45000, 45000 + 300):
        off = (root_extent - base) * SECTOR_DATA
        if off < 0 or off + 34 > len(iso):
            continue
        rec_len = iso[off]
        name_len = iso[off + 32]
        if 34 <= rec_len <= 255 and name_len == 1 and iso[off + 33] == 0:
            return base
    return 0


def sha256(path: pathlib.Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def parse_gdi(gdi: pathlib.Path) -> list[dict]:
    tracks = []
    for line in gdi.read_text(errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) < 5:
            continue
        number, lba, kind, sector_size, *rest = parts
        # Real .gdi lines are: track lba type sector_size filename offset
        # (the trailing offset is 0 for ordinary dumps, not part of the name).
        if len(rest) >= 2 and rest[-1].lstrip("-").isdigit():
            rest = rest[:-1]
        tracks.append(
            {
                "number": number,
                "lba": int(lba),
                "type": int(kind),
                "sector_size": int(sector_size),
                "file": gdi.parent / " ".join(rest).strip('"'),
            }
        )
    return tracks


def parse_cue(cue: pathlib.Path) -> list[dict]:
    """Tracks declared by a .cue: FILE "name" BINARY / TRACK nn MODEx/2352."""
    tracks, current = [], None
    for line in cue.read_text(errors="replace").splitlines():
        line = line.strip()
        if line.startswith("FILE "):
            start, end = line.find('"'), line.rfind('"')
            current = {"file": cue.parent / line[start + 1:end], "track": None,
                       "mode": None, "sector_size": SECTOR_RAW}
            tracks.append(current)
        elif line.startswith("TRACK ") and current is not None:
            parts = line.split()
            current["track"] = parts[1]
            current["mode"] = parts[2]
            # AUDIO tracks carry no sector size; keep the raw 2352 default.
            current["sector_size"] = (
                int(parts[2].split("/")[1]) if "/" in parts[2] else SECTOR_RAW
            )
    return tracks


def raw_to_iso(raw_path: pathlib.Path, iso_path: pathlib.Path) -> None:
    size = raw_path.stat().st_size
    if size % SECTOR_RAW != 0:
        raise SystemExit(f"{raw_path}: size {size} is not a multiple of 2352")
    sectors = size // SECTOR_RAW
    with raw_path.open("rb") as src, iso_path.open("wb") as dst:
        for _ in range(sectors):
            sector = src.read(SECTOR_RAW)
            mode = sector[15]
            if mode == 1:
                dst.write(sector[MODE1_DATA_OFFSET:MODE1_DATA_OFFSET + SECTOR_DATA])
            elif mode == 2:
                # Form 1 XA: 8-byte subheader after the 16-byte header.
                dst.write(
                    sector[MODE1_DATA_OFFSET + 8:MODE1_DATA_OFFSET + 8 + SECTOR_DATA]
                )
            else:
                raise SystemExit(f"unexpected sector mode {mode} at sector {_}")
    print(f"  wrote {iso_path.name}: {sectors} sectors -> {iso_path.stat().st_size} bytes")


def dir_records(iso: bytes, extent: int, length: int):
    data = iso[extent * SECTOR_DATA:(extent * SECTOR_DATA) + length]
    i = 0
    while i < len(data):
        rec_len = data[i]
        if rec_len == 0:
            # Records do not straddle sectors; jump to the next one.
            i = ((i // SECTOR_DATA) + 1) * SECTOR_DATA
            continue
        yield data[i:i + rec_len]
        i += rec_len


def walk(iso: bytes, extent: int, length: int, prefix: str = "") -> list[tuple[str, int]]:
    out = []
    for rec in dir_records(iso, extent, length):
        name_len = rec[32]
        name = rec[33:33 + name_len]
        if name in (b"\x00", b"\x01"):
            continue
        flags = rec[25]
        entry_extent = struct.unpack("<I", rec[2:6])[0]
        entry_size = struct.unpack("<I", rec[10:14])[0]
        text = name.decode("ascii", "replace").split(";")[0]
        path = f"{prefix}{text}"
        out.append((path + ("/" if flags & 0x02 else ""), entry_size))
        if flags & 0x02:
            out.extend(walk(iso, entry_extent, entry_size, path + "/"))
    return out


def main() -> int:
    src = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").expanduser().resolve()
    if not src.exists():
        raise SystemExit(f"missing: {src}")

    out_dir = pathlib.Path.home() / "Projects/lemans-dc/work/unpacked"
    out_dir.mkdir(parents=True, exist_ok=True)
    iso_path: pathlib.Path

    if src.suffix.lower() == ".gdi":
        tracks = parse_gdi(src)
        print(f"{src.name}: {len(tracks)} tracks")
        for t in tracks:
            exists = t["file"].exists()
            size = t["file"].stat().st_size if exists else 0
            print(
                f"  track {t['number']} lba={t['lba']} type={t['type']} "
                f"sector={t['sector_size']} size={size} exists={exists} {t['file'].name}"
            )
        data = [t for t in tracks if t["sector_size"] == SECTOR_RAW and t["type"] == 4]
        if not data:
            data = [t for t in tracks if t["sector_size"] == SECTOR_RAW]
        if not data:
            raise SystemExit("no raw data track found in the .gdi descriptor")
        track = max(data, key=lambda t: t["file"].stat().st_size)
        print(f"  data track: {track['file'].name} sha256={sha256(track['file'])}")
        iso_path = out_dir / (src.stem + "-track" + track["number"] + ".iso")
        raw_to_iso(track["file"], iso_path)
    elif src.suffix.lower() == ".cue":
        tracks = parse_cue(src)
        print(f"{src.name}: {len(tracks)} tracks")
        for t in tracks:
            size = t["file"].stat().st_size if t["file"].exists() else 0
            print(f"  track {t['track']} {t['mode']:<12} size={size} {t['file'].name}")
        data = [t for t in tracks if t["mode"] == "MODE1/2352" and t["file"].exists()]
        if not data:
            raise SystemExit("no MODE1/2352 data track in the .cue")
        track = max(data, key=lambda t: t["file"].stat().st_size)
        print(f"  data track: {track['file'].name} sha256={sha256(track['file'])}")
        iso_path = out_dir / (src.stem + "-track" + str(track["track"]) + ".iso")
        raw_to_iso(track["file"], iso_path)
    elif src.suffix.lower() == ".iso":
        iso_path = src
        print(f"using ISO directly: {iso_path} sha256={sha256(iso_path)}")
    elif src.suffix.lower() == ".cdi":
        raise SystemExit(
            ".cdi needs a CDI-aware extractor (e.g. `cdi2iso`, or extract with "
            "an emulator). Prefer supplying a .gdi dump: the tracks are plain files."
        )
    else:
        raise SystemExit(f"unrecognised image type: {src.name}")

    iso = iso_path.read_bytes()
    pvd = iso[16 * SECTOR_DATA:17 * SECTOR_DATA]
    if pvd[1:6] != b"CD001":
        raise SystemExit("no ISO9660 primary volume descriptor at sector 16")
    root = pvd[156:156 + 34]
    root_extent = struct.unpack("<I", root[2:6])[0]
    root_size = struct.unpack("<I", root[10:14])[0]
    label = pvd[40:72].decode("ascii", "replace").strip()
    base = detect_lba_base(iso, root_extent)
    print(f"volume id: {label!r}  root lba {root_extent} (base {base})")

    entries = walk(iso, root_extent - base, root_size)
    listing = out_dir / (iso_path.stem + ".filelist.txt")
    with listing.open("w") as fh:
        for path, size in entries:
            fh.write(f"{size:>12}  {path}\n")
    print(f"  {len(entries)} entries -> {listing}")

    wanted = ("1ST_READ.BIN", "IP.BIN")
    for path, size in entries:
        if path.upper().rstrip("/").endswith(wanted):
            print(f"  FOUND {path} ({size} bytes)")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
