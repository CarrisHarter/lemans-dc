# lemans-dc — SH-4 RE workspace

Target: *Le Mans 24 Hours* (Sega Dreamcast, Japan release), from the user's own
disc dump. Work product is Ghidra pseudocode + typed structures + an address
index, not linkable C.

## Layout

```
opt/                 (outside this tree) Ghidra 12.1.4 + Temurin JDK 21
work/raw/            frozen read-only disc image + SHA256SUMS
work/unpacked/       ISO extracted from the data track + file listing
analysis/            1ST_READ.BIN, header.json, later: projects and exports
fixtures/            synthetic SH-4 blob and synthetic GDI to test the scripts
scripts/             02..06 pipeline, DumpDecomp.java, patch notes
notes/               evidence, decisions, corrections
```

## Pipeline

| script | step | needs |
| --- | --- | --- |
| `02-freeze-image.sh IMAGE` | freeze + hash a disc image | the image |
| `03-unpack-gdrom.py IMAGE` | GDI/CDI/ISO → ISO + file listing | python3 |
| `04-extract-1stread.py ISO` | pull `1ST_READ.BIN`, parse Katana header | python3 |
| `05-open-in-rea.md` | MCP `open_binary` call for the SH-4 lane | patched rea |
| `06-headless-smoke.sh` | prove the SH-4 lane with no disc | Ghidra |
| `07-build-sh4-cross-gcc.sh` | freestanding SH-4 cross compiler (linkable C) | ~20 min CPU |
| `08-export.sh` | per-function pseudocode + symbols.tsv + objdump listing | rea, binutils |
| `10-seed-tsv.py` | hotlist JSON -> rea seed TSV (computed jumps, vtables) | python3 |
| `01-preflight.sh` | whole-toolchain check before disc work | - |

## Toolchain

- Ghidra 12.1.4 PUBLIC + Temurin JDK 21 under `~/opt` (hash-verified).
- `rea` 6.8.0 patched with an SH-4 raw-binary lane (`sh4` →
  `SuperH4:LE:32:default`). Fork: `~/Projects/rea-sh4`, diff:
  `notes/rea-sh4-sh4-lane.patch`.
- Registered with pi as MCP server `rea` (254 tools, 900 s timeout).

## Non-negotiables

- Only a dump the author owns. Nothing from this tree is redistributable.
- Permission is conditional and revocable; see `NOTICE.md`. Publication is limited to
  technical notes, pseudocode and format docs — never ROMs, disc images or extracted assets.
- Third-party rights (car manufacturers, sponsors, liveries, marks) are not covered by it.
- Every address→meaning claim in `notes/` cites the evidence that produced it.
