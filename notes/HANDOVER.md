# Handover — Le Mans 24 Hours (Dreamcast) → verified Rust port

Copy everything below the line into the new agent's first message.

---

## What this project is

Reverse-engineering *Test Drive Le Mans* (Sega Dreamcast, 2001, **USA** release,
product `T15123N`) and porting it to Rust **locally only**, function by function,
each port proven equivalent to the original SH-4 machine code by an interpreter.

Working directory: `~/Projects/lemans-dc` (git repo, 89 files tracked).

## Work autonomously

Do not ask the user before each function. The loop is mechanical and already
proven; run it unattended and report results, not questions.

- **Default action:** open `analysis/export/pseudocode/functions/`, take the next
  unported address in ascending order that has no outgoing calls, and run the
  full loop (spec → port → verify → evidence → ledger → commit).
- **Batch size:** 10 functions, then a one-paragraph report: ported, failed,
  new instruction gaps, new side-effect patterns. Then continue with the next 10
  without waiting for approval.
- **On failure:** if a port fails verification twice for the same reason, mark
  the function `blocked` in `notes/address-map.md` with the diff output, move to
  the next function, and keep going. Never stall the batch on one function.
- **On a missing oracle instruction:** add it, add a unit test, re-run
  `cargo test`, update `tools/opcode-census.py`'s implemented set, then resume
  the batch. This is expected work, not a question for the user.
- **On an ambiguous name or struct:** record it as unknown in
  `notes/address-map.md` and continue. Do not invent evidence and do not stop.
- **Stop the run and report** only when one of these is true:
  1. the oracle hits an instruction you cannot implement without guessing
     semantics;
  2. a function's behaviour cannot be pinned by any spec (e.g. it depends on
     hardware registers or DMA);
  3. an action would touch the legal constraints above;
  4. ten consecutive functions fail.
- **Always leave the repo green:** `cargo test` in `rust/sh4-oracle` passes,
  `git status` is clean, every claimed port has a verification file.
- **Never wait for a "go".** If the user is silent, keep porting. The project's
  bottleneck is 5,211 functions, not permission.

## Legal constraints — do not violate these

- Permission was granted by Business Affairs, Atari SA (2026-11-11) for personal,
  non-commercial **static analysis** and publication of notes/pseudocode only.
  Archive in `notes/permission/`; approved notice in `NOTICE.md` and generated
  into every export by `scripts/08-export.sh`.
- **Never publish the port.** No repo, demo, video or WASM build of the Rust
  game. The grant covers analysis, not a derivative playable work.
- Never redistribute the ROM, disc image, or any extracted asset (art, audio,
  models, textures).
- Third-party rights (car manufacturers, sponsors, liveries, Le Mans marks) are
  **not** covered by the Atari grant.
- `work/`, `analysis/*.bin`, `analysis/data/` and `analysis/export/` are
  gitignored on purpose. Keep it that way.

## Environment (already installed, do not rebuild)

| thing | path |
| --- | --- |
| Ghidra 12.1.4 | `~/opt/ghidra_12.1.4_PUBLIC` (`GHIDRA_INSTALL_DIR`) |
| JDK 21 (Temurin) | `~/opt/jdk-21.0.13+9` (`JAVA_HOME`) |
| SH-4 cross compiler | `~/opt/sh-elf/bin/sh-elf-gcc` (freestanding, no newlib) |
| cross tool symlinks | `~/opt/sh-elf/toolbin` (needed with `-B`) |
| rea (patched fork) | `~/Projects/rea-sh4` + MCP server `rea` in `~/.pi/agent/mcp.json` |
| SH-4 binutils | `sh4-elf-objdump` (package `sh4-elf-binutils`) |

Facts learned the hard way — do not rediscover them:

- The Ghidra language for Dreamcast is **`SuperH4:LE:32:default`**. The id
  `SuperH:LE:32:SH-4` does not exist in Ghidra 12.x.
- `sh-elf` defaults to **big-endian**; always pass `-ml`.
- Float code needs **`-m4-single-only`**; without it GCC emits soft-float calls
  (`__addsf3`), which is a different program.
- The stage-1 compiler has no target program prefix, so pass
  `-B$HOME/opt/sh-elf/toolbin` or it calls the host x86 `as`.
- rea's raw-binary lane was patched to accept `architecture: "sh4"`; diff at
  `notes/rea-sh4-sh4-lane.patch`.

## What exists on disk

- `work/raw/extracted/` — frozen disc: `.cue` + 5 track `.bin` (read-only, hashed
  in `SHA256SUMS`). Track 3 and track 5 are data; audio tracks sit between them.
- `analysis/lemans24.bin` — **the game**: 2,283,504 bytes, sha256 `08016236…`,
  mapped at `0x8C010000`. This is the analysis target.
- `analysis/1st_read.bin` — 108 KB boot stub (not the game). `analysis/ip.bin` —
  disc header (title, product number, boot filename).
- `analysis/export/` — 5,211 per-function Ghidra pseudocode files, `symbols.tsv`,
  and `disassembly.asm` (41 MB, 969,383 instructions).
- `analysis/labels.tsv` — 310 auto-proposed names from Ghidra string labels.
- `rust/sh4-oracle/` — SH-4 interpreter, 15 tests, covers **96.67%** of static
  instructions. CLI + `--spec` JSON mode.
- `rust/lemans-verify/` — registry of ported functions + differ.
- `notes/` — `04-first-contact.md` (disc findings), `05-progress.md` (metrics),
  `address-map.md` (ledger), `specs/` (spec schema + files), `verifications/`
  (evidence JSON per verified function).

## The verification loop (this is the whole method)

1. Pick a function address from `analysis/export/pseudocode/functions/<addr>.c`.
2. Write `notes/specs/<addr>.json` — seed registers and memory (schema in
   `notes/specs/README.md`). One spec per code path; a function with branches
   gets several specs (`<addr>.json`, `<addr>.invalid.json`, …).
3. Implement it in `rust/lemans-verify/src/registry.rs` as
   `fn(&mut Cpu, &mut Memory)` and register the address in `lookup()`.
4. Run `./scripts/verify-function.sh <addr> [binary] [spec]`.
5. Only when it prints `ok` does the port count. The evidence lands in
   `notes/verifications/`. Record it in `notes/address-map.md`.
6. Commit.

Verified so far: `0x8c175c72` (`am_sound_is_playing`), `0x8c18b9c8`
(`array_lookup`, both branches), `0x8c1552e4` (`store_through_two_levels`), plus
the fixture `0x8c010000` as the harness regression test.

## The single most important lesson

**Every ported function so far failed first, and every failure was a register
side effect that the decompiled C did not show**: an argument copied into `r4`,
a loaded field left in `r3`, `base[1]` left in `r1`, an inner pointer left in
`r0`, and the T flag differing per branch. Diff registers and T, always. A port
that "looks right" and compiles is not evidence of anything.

## Known gaps / next work

- Oracle coverage 96.67% static; the remaining mnemonics are a long tail under
  0.08% each (`pref`, `tst.b`, `mac.w`, `cmp/str`, `shal`, `mac.l`, `subc`).
  Run `python3 tools/opcode-census.py analysis/export/disassembly.asm` to see the
  current list. Anything unimplemented fails loudly as `Unimplemented`.
- Specs cannot cross calls: a function that calls into another subsystem cannot
  be verified yet. That blocks most of the binary (only 1,616 of 5,211 functions
  have no outgoing calls at all).
- No modelling of store queues, DMA, hardware registers, or interrupts.
- The synthetic stack window is `0x8C300000`–`0x8C310000`; stack writes are
  excluded from the diff on purpose.
- `DATA.ZIP` is a reordered zip (central directory first) — 3,552 entries parsed
  by directory walk, but extraction is not implemented yet.
- Only the USA disc is analysed; the Japan SKU (named in the permission) has
  different offsets.

## Useful commands

```bash
cd ~/Projects/lemans-dc
python3 scripts/discfs.py "work/raw/extracted/Test Drive Le Mans (USA) (En,Fr,Es).cue" --list
./scripts/verify-function.sh 0x8c175c72
python3 tools/opcode-census.py analysis/export/disassembly.asm
cd rust/sh4-oracle && cargo test
```

## Rules of engagement

- Never invent evidence. If a name, struct layout or branch is ambiguous, log it
  in `notes/address-map.md` as unknown and move on.
- Never mark a function verified without `scripts/verify-function.sh` printing
  `ok` and a file in `notes/verifications/`.
- Do not touch the frozen disc in `work/raw/`.
- Commit after each verified port, with the side effect you had to add in the
  message — that is the knowledge that would otherwise be lost.
