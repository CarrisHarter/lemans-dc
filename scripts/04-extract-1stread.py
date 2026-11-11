#!/usr/bin/env python3
"""Step 4: pull 1ST_READ.BIN out of an unpacked ISO and parse the Katana header.

Inputs:  work/unpacked/<name>.iso (from 03-unpack-gdrom.py)
Outputs: analysis/1st_read.bin, analysis/ip.bin, analysis/header.json

Scrambling: a Katana bootstrap begins with "SEGA SEGAKATANA ". When that magic
is absent the payload is either scrambled or not a boot binary; this script
reports which and refuses to guess, because the descramble step needs KOS's
sector-inverse tool and must be verified against the real image.

IP.BIN / 1ST_READ.BIN header layout (Katana):
  0x00 16 hardware id      "SEGA SEGAKATANA "
  0x10 16 maker id         "SEGA ENTERPRISES"
  0x20 16 device info
  0x30  8 area symbols
  0x38  8 peripherals
  0x40 10 product number
  0x4A  6 version
  0x50  8 release date (YYYYMMDD), 16 bytes reserved
  0x60 16 boot filename
  0x70 16 company
  0x80 128 title
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import math
import pathlib
import struct
import sys

SECTOR_DATA = 2048
MAGIC = b"SEGA SEGAKATANA "
DEFAULT_LOAD_BASE = 0x8C010000

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


def _load_scrambler():
    """Import the sibling Comstedt port without requiring a package layout."""
    here = pathlib.Path(__file__).resolve().parent
    spec = importlib.util.spec_from_file_location("dc_scramble", here / "scramble.py")
    if spec is None or spec.loader is None:
        raise SystemExit("cannot load scripts/scramble.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def dir_records(iso: bytes, extent: int, length: int):
    data = iso[extent * SECTOR_DATA:(extent * SECTOR_DATA) + length]
    i = 0
    while i < len(data):
        rec_len = data[i]
        if rec_len == 0:
            i = ((i // SECTOR_DATA) + 1) * SECTOR_DATA
            continue
        yield data[i:i + rec_len]
        i += rec_len


def find_files(iso: bytes, base: int = 0) -> dict[str, tuple[int, int]]:
    """Map upper-case basename -> (extent_lba, size) for every ISO9660 file."""
    pvd = iso[16 * SECTOR_DATA:17 * SECTOR_DATA]
    root = pvd[156:156 + 34]
    root_extent = struct.unpack("<I", root[2:6])[0]
    root_size = struct.unpack("<I", root[10:14])[0]
    if base == 0:
        base = detect_lba_base(iso, root_extent)
    found: dict[str, tuple[int, int]] = {}
    stack = [(root_extent - base, root_size)]
    seen = set()
    while stack:
        extent, size = stack.pop()
        if (extent, size) in seen:
            continue
        seen.add((extent, size))
        for rec in dir_records(iso, extent, size):
            name_len = rec[32]
            name = rec[33:33 + name_len]
            if name in (b"\x00", b"\x01"):
                continue
            flags = rec[25]
            entry_extent = struct.unpack("<I", rec[2:6])[0]
            entry_size = struct.unpack("<I", rec[10:14])[0]
            text = name.decode("ascii", "replace").split(";")[0].upper()
            if flags & 0x02:
                stack.append((entry_extent, entry_size))
            else:
                found[text] = (entry_extent, entry_size)
    return found


def extract(iso: bytes, extent: int, size: int) -> bytes:
    start = extent * SECTOR_DATA
    return iso[start:start + size]


def entropy(data: bytes) -> float:
    if not data:
        return 0.0
    counts = [0] * 256
    for byte in data:
        counts[byte] += 1
    total = len(data)
    return -sum((c / total) * math.log2(c / total) for c in counts if c)


def parse_header(blob: bytes) -> dict:
    def field(off: int, length: int) -> str:
        return blob[off:off + length].decode("ascii", "replace").rstrip("\x00 ").strip()

    return {
        "magic": field(0x00, 16),
        "magic_ok": blob[0:16] == MAGIC,
        "maker_id": field(0x10, 16),
        "device_info": field(0x20, 16),
        "area_symbols": field(0x30, 8),
        "peripherals": field(0x38, 8),
        "product_number": field(0x40, 10),
        "version": field(0x4A, 6),
        "release_date": field(0x50, 8),
        "boot_filename": field(0x60, 16),
        "company": field(0x70, 16),
        "title": field(0x80, 128),
    }


def main() -> int:
    if len(sys.argv) < 2:
        raise SystemExit("usage: 04-extract-1stread.py /path/to/unpacked.iso")
    iso_path = pathlib.Path(sys.argv[1]).expanduser().resolve()
    iso = iso_path.read_bytes()
    if iso[16 * SECTOR_DATA + 1:16 * SECTOR_DATA + 6] != b"CD001":
        raise SystemExit(f"{iso_path}: not an ISO9660 image")

    pvd_root = iso[16 * SECTOR_DATA + 156:16 * SECTOR_DATA + 190]
    root_extent = struct.unpack("<I", pvd_root[2:6])[0]
    lba_base = detect_lba_base(iso, root_extent)
    files = find_files(iso, lba_base)
    if not files:
        raise SystemExit("no files found in the ISO9660 tree")

    wanted_boot = next((n for n in files if n.endswith("1ST_READ.BIN")), None)
    if wanted_boot is None:
        raise SystemExit(
            "1ST_READ.BIN not found. Files present: " + ", ".join(sorted(files)[:40])
        )
    ip_name = next((n for n in files if n.endswith("IP.BIN")), None)

    out_dir = pathlib.Path.home() / "Projects/lemans-dc/analysis"
    out_dir.mkdir(parents=True, exist_ok=True)

    boot_extent, boot_size = files[wanted_boot]
    raw_boot = extract(iso, boot_extent - lba_base, boot_size)

    descrambled = False
    boot = raw_boot
    if raw_boot[:len(MAGIC)] != MAGIC:
        scrambler = _load_scrambler()
        candidate = scrambler.descramble(raw_boot)
        if candidate[:len(MAGIC)] == MAGIC:
            descrambled = True
            (out_dir / "1st_read.scrambled.bin").write_bytes(raw_boot)
            boot = candidate
            print(f"  descrambled {len(raw_boot)} bytes (Katana magic recovered)")
        else:
            print("  magic still absent after descramble attempt")

    (out_dir / "1st_read.bin").write_bytes(boot)

    ip_blob = None
    if ip_name:
        ip_extent, ip_size = files[ip_name]
        ip_blob = extract(iso, ip_extent - lba_base, ip_size)
        (out_dir / "ip.bin").write_bytes(ip_blob)

    header = parse_header(boot)
    ip_header = parse_header(ip_blob) if ip_blob else None

    scrambled = not header["magic_ok"]
    report = {
        "source_iso": str(iso_path),
        "iso_sha256": sha256(iso),
        "boot_entry": wanted_boot,
        "boot_extent_lba": boot_extent,
        "boot_size": boot_size,
        "boot_sha256": sha256(boot),
        "descrambled_by_katana_permutation": descrambled,
        "header": header,
        "ip_header": ip_header,
        "shannon_entropy_bits_per_byte": round(entropy(boot[:0x1000]), 3),
        "scrambled": scrambled,
        "load_base": hex(DEFAULT_LOAD_BASE),
        "entry_point_guess": hex(DEFAULT_LOAD_BASE + 0x100),
        "notes": [],
    }
    if scrambled:
        report["notes"].append(
            "No Katana magic at offset 0. Descramble before analysis; do not "
            "map this file as SH-4 code yet."
        )
    else:
        report["notes"].append(
            "Katana magic present. The 0x00-0xFF region is the bootstrap header, "
            "not code; map the file at load_base and use the reported entry."
        )

    (out_dir / "header.json").write_text(json.dumps(report, indent=2) + "\n")

    print(f"1ST_READ.BIN  {boot_size} bytes  sha256={report['boot_sha256']}")
    for key, value in header.items():
        print(f"  {key:<16} {value!r}")
    print(f"  entropy(0x1000)  {report['shannon_entropy_bits_per_byte']}")
    print(f"  scrambled        {report['scrambled']}")
    print(f"\nwrote {out_dir}/1st_read.bin and {out_dir}/header.json")
    return 0 if not scrambled else 2


if __name__ == "__main__":
    raise SystemExit(main())
