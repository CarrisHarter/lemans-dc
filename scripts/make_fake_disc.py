#!/usr/bin/env python3
"""Build a synthetic Dreamcast-shaped disc image to test steps 3-4 offline.

Produces, in fixtures/fake-disc/:
  disc.gdi          GD-ROM descriptor (4 tracks: 3 audio/raw + 1 data)
  track01.bin       dummy audio track
  track02.raw       dummy raw track
  track03.bin       MODE1/2352 data track holding a tiny ISO9660 filesystem
                    with IP.BIN and 1ST_READ.BIN

1ST_READ.BIN starts with the real Katana header so the header parser in
scripts/04-extract-1stread.py is exercised with realistic bytes.
"""
from __future__ import annotations

import pathlib
import struct

SECTOR_RAW = 2352
SECTOR_DATA = 2048


def both32(value: int) -> bytes:
    return struct.pack("<I", value) + struct.pack(">I", value)


def both16(value: int) -> bytes:
    return struct.pack("<H", value) + struct.pack(">H", value)


def dir_record(name: bytes, lba: int, size: int, flags: int = 0) -> bytes:
    rec = bytearray()
    rec.append(0)
    rec.append(0)
    rec += both32(lba)
    rec += both32(size)
    rec += bytes([124, 1, 1, 0, 0, 0, 0])
    rec.append(flags)
    rec.append(0)
    rec.append(0)
    rec += both16(1)
    rec.append(len(name))
    rec += name
    if len(rec) % 2:
        rec.append(0)
    rec[0] = len(rec)
    return bytes(rec)


def katana_header(boot_name: str, title: str) -> bytes:
    head = bytearray(0x100)
    head[0x00:0x10] = b"SEGA SEGAKATANA "
    head[0x10:0x20] = b"SEGA ENTERPRISES"
    head[0x20:0x30] = b"0000 CD-ROM1/1"
    head[0x30:0x38] = b"E       "
    head[0x38:0x40] = b"00000000"
    head[0x40:0x4A] = b"T-9707N   "
    head[0x4A:0x50] = b"V1.000"
    head[0x50:0x58] = b"20010329"
    head[0x60:0x70] = boot_name.encode().ljust(16, b" ")
    head[0x70:0x80] = b"SEGA ENTERPRISES"
    head[0x80:0x80 + len(title)] = title.encode()
    return bytes(head)


def build_iso(files: dict[str, bytes]) -> bytes:
    sectors: list[bytes] = [bytes(SECTOR_DATA) for _ in range(16)]

    # Layout: 16 PVD, 17 terminator, 18 root dir, 19.. file data.
    data_lba = 19
    placed: list[tuple[bytes, int, int, int]] = []
    payload = bytearray()
    for name, blob in files.items():
        lba = data_lba + len(payload) // SECTOR_DATA
        placed.append((f"{name};1".encode(), lba, len(blob), 0))
        payload += blob
        payload += bytes((-len(blob)) % SECTOR_DATA)

    root = bytearray()
    root += dir_record(b"\x00", 18, SECTOR_DATA, flags=0x02)
    root += dir_record(b"\x01", 18, SECTOR_DATA, flags=0x02)
    for name, lba, size, flags in placed:
        root += dir_record(name, lba, size, flags)
    root = root.ljust(SECTOR_DATA, b"\x00")

    pvd = bytearray(SECTOR_DATA)
    pvd[0] = 1
    pvd[1:6] = b"CD001"
    pvd[6] = 1
    pvd[8:40] = b" " * 32
    pvd[40:72] = b"LEMANS_DC_FAKE".ljust(32, b" ")
    pvd[80:88] = both32(20)
    pvd[120:124] = both16(1)
    pvd[124:128] = both16(1)
    pvd[128:132] = both16(SECTOR_DATA)   # logical block size (2 bytes, both-endian)
    pvd[132:140] = both32(0)             # path table size (8 bytes, both-endian)
    pvd[156:190] = dir_record(b"\x00", 18, SECTOR_DATA, flags=0x02)

    term = bytearray(SECTOR_DATA)
    term[0] = 255
    term[1:6] = b"CD001"
    term[6] = 1

    sectors.append(bytes(pvd))
    sectors.append(bytes(term))
    sectors.append(bytes(root))
    sectors.append(bytes(payload))
    return b"".join(sectors)


def to_raw_mode1(iso: bytes) -> bytes:
    out = bytearray()
    for i in range(0, len(iso), SECTOR_DATA):
        user = iso[i:i + SECTOR_DATA].ljust(SECTOR_DATA, b"\x00")
        head = bytearray(b"\x00" * 16)
        head[0] = 0x00
        head[1:4] = b"\x00\x00\x00"
        head[15] = 1
        out += bytes(head) + user + bytes(SECTOR_RAW - 16 - SECTOR_DATA)
    return bytes(out)


def main() -> int:
    out = pathlib.Path.home() / "Projects/lemans-dc/fixtures/fake-disc"
    out.mkdir(parents=True, exist_ok=True)

    boot = katana_header("1ST_READ.BIN", "LE MANS 24 HOURS") + bytes(0x300)
    # A few plausible SH-4 words so the file is not all zeroes.
    boot = boot[:0x100] + bytes.fromhex("13 64 00 e0 01 70 10 41 fc 8b 0b 00 09 00") + boot[0x10E:]
    ip = katana_header("1ST_READ.BIN", "LE MANS 24 HOURS")

    iso = build_iso({"IP.BIN": ip, "1ST_READ.BIN": boot})
    (out / "track03.bin").write_bytes(to_raw_mode1(iso))
    (out / "track01.bin").write_bytes(bytes(2352 * 4))
    (out / "track02.raw").write_bytes(bytes(2352 * 4))
    (out / "disc.gdi").write_text(
        "3\n"
        '1 0 4 2352 track01.bin 0\n'
        '2 300 0 2352 track02.raw 0\n'
        "3 45000 4 2352 track03.bin 0\n"
    )
    print(f"built fake disc in {out}")
    for p in sorted(out.iterdir()):
        print(f"  {p.name}: {p.stat().st_size} bytes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
