# Step 11 — systematic RE, bottom-up

Status: **blocked on the binary**. Mechanism now proven; the analysis itself has
nothing to run on.

## What is proven

| mechanism | evidence |
| --- | --- |
| seed TSV generation | `scripts/10-seed-tsv.py` validates range, SH-4 2-byte alignment, label names |
| rea applies seeds before auto-analysis | same fixture: **1 procedure → 4**; rea reports `3 functions created, 1 already present, 0 failed; 1 code seeds decoded; 1 label applied` |
| seed file must be absolute | relative path is refused: `REA_GHIDRA_SEED_FILE must be absolute` |
| independent decoder cross-check | `sh4-elf-objdump -m sh4 -EL` agrees with Ghidra instruction-for-instruction |

## Order of work (when the binary exists)

1. **Strings first.** `search_strings` for `LE MANS`, `1ST_READ`, `TRACK`, `.PVR`,
   `.ADX`, `.NJ`, `BIN`, `SOUND`, `CONFIG`, plus debug leftovers (`ERROR`, `WARN`,
   `ASSERT`, `.c`, `.cpp`). Every hit is an address→purpose candidate with
   evidence, and file-name strings usually expose the resource loader's table.
2. **Resource/file loader.** Follow the strings that name extensions. Recover the
   file table (name, offset, size, flags) and type it as a struct. This is the
   loader for every asset, so it names the rest of the binary almost for free.
3. **Track/scene system.** Track files, sector/scene lists, spawn points.
4. **Vehicle physics.** Fixed-step integrator: look for the constant stepping
   value, the tyre/suspension term structure, and the car state struct.
5. **Renderer.** PVR state setup, sort/draw list submission, track mesh.

At each step: seed computed jump targets, rename functions as evidence lands,
and append to `notes/address-map.md` with the evidence that justified the name.

## Address map

`notes/address-map.md` is the ledger. Table columns:

| address | name | kind | evidence |
| --- | --- | --- | --- |

A name is only admitted when the evidence column names the thing that produced
it (string xref, struct offset, call site, test result). Guessed names are
marked `?` and never promoted without evidence.

## Assets in parallel

PVR textures, Ninja (`.NJ`) models and ADX audio are parsed from the unpacked
ISO, not from the binary. Their names feed the loader analysis in the opposite
direction: a file table entry that matches a real asset's header confirms the
table layout without reading a single instruction.
