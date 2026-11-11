# Status — Dreamcast SH-4 pipeline

Updated 2026-11-11.

## Done and verified

| step | state | evidence |
| --- | --- | --- |
| 5 Ghidra 12.1.4 + JDK 21 | **done** | `~/opt/ghidra_12.1.4_PUBLIC`, `~/opt/jdk-21.0.13+9`; both archives hash-verified against upstream |
| 6 headless SH-4 analysis | **done** | `notes/01-toolchain-smoke-test.md`, `scripts/06-headless-smoke.sh` |
| 7 rea + clean doctor | **done** | rea 6.8.0; doctor: ghidra/version/native_decompiler/java all `ok: true` |
| 8 rea registered in pi | **done** | `~/.pi/agent/mcp.json`, `pi mcp list` → `connected, 254 tools` |
| 9 SH-4 lane patch | **done, tested** | 7 files, 94 insertions; 36 unit tests in `rawBinary.test.ts` pass; diff at `notes/rea-sh4-sh4-lane.patch` |
| 10 rea against an SH-4 image | **done on fixtures** | `decompile` returns real SH-4 pseudocode; `inspect` maps `analysis/1st_read.bin` at `0x8C010100` |
| 3 GD-ROM unpack | **written + exercised** | `scripts/03-unpack-gdrom.py` on a synthetic GDI (`scripts/make_fake_disc.py`) |
| 4 header parse | **written + exercised** | `scripts/04-extract-1stread.py` recovers `1ST_READ.BIN`, parses Katana header |

## Blocked

Steps 2–4 against the real disc need the image on this machine. Nothing ROM-like
is in `~/Downloads`; `work/raw/` is empty. I am not downloading a copyrighted
commercial GD-ROM from a ROM mirror.

## Corrections discovered during execution

1. **`SuperH:LE:32:SH-4` does not exist in Ghidra 12.x.** Use `SuperH4:LE:32:default`
   with cspec `default`. Ghidra's `SuperH` module is now big-endian SH-1/SH-2/SH-2A only.
2. **Raw import seeds the entry at the mapping base.** rea maps file offset 0 at
   `entry_point` and creates the entry function there. A Katana `1ST_READ.BIN` has a
   0x100-byte header at offset 0, so the entry function would be header bytes —
   map at `0x8C010000` and separately seed `0x8C010100`, or (preferred) strip the
   header and map the code at `0x8C010100`. Script 04 needs a `1st_read.code.bin`
   output for that; added once the real header offset is confirmed on the real file.
3. **Raw import finds zero functions by itself.** Even after a full linear sweep and
   auto-analysis, only caller-named entries exist. The real target needs seeding at
   the Katana entry and at call targets (helper TSV → `REA_GHIDRA_SEED_FILE`).
4. **`.gdi` descriptors end with a trailing offset field** (`track lba type sector
   filename 0`); that 0 is not part of the filename. Handled in script 03.
5. **The first real doubt about "source code":** Katana binaries ship with no
   symbols, no linker map and no debug info, so every name is recovered evidence,
   never a recovered identifier.

## Not started

Step 11 (systematic RE) and step 12 (whole-game C via DecompPipeline or a
matching-decomp effort) — both need the real binary.

## How to use what exists now

```bash
# toolchain self-test, no disc needed
scripts/06-headless-smoke.sh

# once the image exists
scripts/02-freeze-image.sh /path/to/lemans.gdi
scripts/03-unpack-gdrom.py  work/raw/lemans.gdi
scripts/04-extract-1stread.py work/unpacked/lemans-track3.iso
# then, through the rea MCP server (restart pi or /reload first)
#   open_binary {path: analysis/1st_read.bin, format: raw-binary,
#                architecture: sh4, entry_point: "0x8C010100", provider_id: ghidra}
```

Any new pi session picks the MCP server up automatically; the current session
needed a restart because servers are connected at startup.
