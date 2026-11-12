# Handover — Le Mans 24 Hours (Dreamcast) → verified Rust port

Copy everything below the line into the new agent's first message.

---

## What this project is

Reverse-engineering *Test Drive Le Mans* (Sega Dreamcast, 2001, **USA** release,
product `T15123N`) and porting it to Rust **locally only**, function by function,
each port proven equivalent to the original SH-4 machine code by an interpreter.

Working directory: `~/Projects/lemans-dc` (git repo).

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
- **Stop the run and report only when one of these is true:**
  1. the oracle hits an instruction you cannot implement without guessing;
  2. a function's behaviour cannot be pinned by any spec (hardware registers, DMA);
  3. an action would touch the legal constraints below;
  4. ten consecutive functions fail.
- **Always leave the repo green:** `cargo test` in `rust/sh4-oracle` passes,
  `git status` is clean, every claimed port has a passing verification file.
- **Never wait for a "go".** If the user is silent, keep porting. The bottleneck
  is 5,211 functions, not permission.

## Legal constraints — do not violate these

- Permission was granted by Business Affairs, Atari SA (2026-11-11) for personal,
  non-commercial static analysis and publication of notes/pseudocode only.
  Archive in `notes/permission/`; approved notice in `NOTICE.md` and generated
  into every export by `scripts/08-export.sh`.
- Never publish the port. No repo, demo, video or WASM build of the Rust game.
  `CarrisHarter/lemans-dc` is a **private backup** of this repository and must
  stay private: no collaborators, no fork, no Pages. `scripts/git-hooks/pre-push`
  checks `gh api repos/CarrisHarter/lemans-dc --jq .private` before every push
  and refuses any commit that adds `analysis/*.bin`, `analysis/data/`,
  `analysis/export/`, `work/` or a ROM fixture. A user timer
  (`lemans-backup.timer`, every 30 min) pushes committed work only.
- Never redistribute the ROM, disc image, or any extracted asset (art, audio,
  models, textures). A padded copy of the game image was once committed as a test
  fixture; it has been purged from all history and `.gitignore` now refuses
  `fixtures/lemans24*.bin`. Never copy the image into the repository.
- Third-party rights (car manufacturers, sponsors, liveries, Le Mans marks) are
  not covered by the Atari grant.
- `work/`, `analysis/*.bin`, `analysis/data/` and `analysis/export/` are
  gitignored on purpose. Keep it that way.

## Environment (already installed, do not rebuild)

| thing | path |
| --- | --- |
| Ghidra 12.1.4 | `~/opt/ghidra_12.1.4_PUBLIC` (`GHIDRA_INSTALL_DIR`) |
| JDK 21 (Temurin) | `~/opt/jdk-21.0.13+9` (`JAVA_HOME`) |
| SH-4 cross compiler | `~/opt/sh-elf/bin/sh-elf-gcc` (freestanding, no newlib) |
| cross tool symlinks | `~/opt/sh-elf/toolbin` (needed with `-B`) |
| rea (patched fork) | `~/Projects/rea-sh4` + MCP server `rea` |
| SH-4 binutils | `sh4-elf-objdump` |
| Rust stable (rustup, installed 2026-11-12) | `~/.cargo/bin`, symlinked into `~/.local/bin` |

Facts learned the hard way:

- The Ghidra language for Dreamcast is `SuperH4:LE:32:default`.
- `sh-elf` defaults to big-endian; always pass `-ml`.
- Float code needs `-m4-single-only`; without it GCC emits soft-float calls.
- The stage-1 compiler has no target prefix: pass `-B$HOME/opt/sh-elf/toolbin`.
- `sh-elf-as` is the arbiter for an encoding question: assemble the instruction
  and read the bytes.

## What exists on disk

- `work/raw/extracted/` — frozen disc: `.cue` + 5 track `.bin` (read-only).
- `analysis/lemans24.bin` — the game, 2,283,504 bytes, mapped at `0x8C010000`
  (image ends at `0x8C23D7F0`). The analysis target.
- `analysis/export/` — 5,211 per-function pseudocode files, `symbols.tsv`, and
  `disassembly.asm` (41 MB, 969,383 instructions).
- `rust/sh4-oracle/` — SH-4 interpreter, 55 tests, 99.70% static coverage.
  Models MACH/MACL/GBR/VBR/SSR/SPC/SGR/DBR/bank registers and maps zeroed RAM
  from the image end to the stack, so globals past the image are addressable.
- `rust/lemans-verify/` — one port per file in `src/ports/p_<address>.rs`, with
  `src/registry.rs` generated by `scripts/gen-registry.py`.
- `notes/` — `05-progress.md` (metrics), `address-map.md` (ledger),
  `specs/` (schema + files), `verifications/` (evidence per verified function).

## The verification loop (this is the whole method)

1. Pick a function address from `analysis/export/pseudocode/functions/<addr>.c`.
2. Write `notes/specs/<addr>.json` — seed registers, FR, memory, steps. One spec
   per code path; branches get `<addr>.invalid.json` and so on. Always leave a
   default `<addr>.json`, or the standard verify command fails with "no spec".
   `memory[].hex` is raw little-endian bytes; FR seeds use keys like `f4`, not
   `fr4` (a wrong key is silently ignored).
3. Write `rust/lemans-verify/src/ports/p_<addr>.rs` exporting
   `pub fn run(cpu: &mut Cpu, mem: &mut Memory)`.
4. `python3 scripts/gen-registry.py`.
5. `./scripts/verify-function.sh <addr>` must print `ok`, and
   `notes/verifications/<addr>.json` must say `"result": "pass"`.
6. Append a row to `notes/address-map.md` (`scripts/record-batch.py` does this
   from a workspace ledger), then commit.

## The single most important lesson

Every port failed first, and every failure was a register side effect the
decompiled C did not show: an argument copied into r4, a loaded field left in
r3, a base pointer left in r1, an inner pointer left in r0, a different T bit
per branch, and delay-slot instructions that run after `rts`/`bra`/`bf.s`.
Diff registers and T, always. A port that "looks right" and compiles is not
evidence.

## Instruction semantics that cost ports a retry

- **A `bt.s` delay slot runs even on the FINAL, non-taken iteration.**
  In `0x8c011d1c`, `fmov fr3,fr4` in the delay slot overwrites the
  accumulator with a half-computed value on exit; a `loop { if !t break }`
  transcription that `break`s before the write-back produces a wrong `fr4`
  (and wrong `fr0`) on every input. Model: unconditional write-back, then
  `if t { continue } else { break }`.

- Plain `bt`/`bf` have **no** delay slot. `bt.s`/`bf.s`, `bra` and `rts` do, and
  the delay slot runs whether or not the branch is taken.
- `fcmp/gt frA,frB` sets T = `frB > frA` — the encoding's operand order is the
  reverse of the listing text.
- `shll2`/`shlr2` do not touch T; `shll`/`shlr`/`shar`/`shal` do.
- `flds`/`ftrc` take their register from the `n` field (`sh-elf-as`:
  `ftrc fr4,fpul` → `0xF43D`). The oracle used to read `m`.
- `mov.b` sign-extends; `mov.w` sign-extends; `extu.b`/`extu.w` zero-extend.
- A `bra` whose target is the loop test makes a while loop; a body before the
  test is a do-while. Read which address the branch targets.
- **The `rts` delay slot is the return value more often than not.** A loop that
  computes a count usually ends `rts ; mov r3,r0`, so the register the C calls
  the loop index is overwritten by the result. Always read the word after `rts`.
- `subc r4,r4` then `add #1,r4` is the standard "materialise a boolean" idiom:
  `r4 = 1 - T`. `movt rN` (any register, `op & 0xF0FF == 0x0029`) is the other.
- `mov.w @(disp,PC),Rn` and `mov.l @(disp,PC),Rn` read the *code* stream, so a
  port must read the pool word from memory rather than hardcode it if the spec
  might patch it.

## Spec-writing traps

- Little-endian hex, always. `0x8c210000` is `"0000218c"`, not `"00218c00"`.
  Getting this wrong produces `data access 0x008c2100 outside mapped memory`,
  which looks like a port bug and is not one.
- JSON has no hex literals: `"r4": 64`, never `"r4": 0x40`. The verifier reports
  `bad spec JSON: bad object at byte N`.
- Seed every pointer you dereference, including the literal-pool words the code
  loads (`mov.l @(disp,PC),Rn`), and every table the function indexes.
- A function whose `rts` returns to `pr = 0` halts at `HALT_PC`; that is the
  normal stop, not a failure.

## Decoder bugs found by ports (keep these fixed)

1. `load`/`store` decoded width only for nibbles 0/1/2, so `mov.b @Rm+,Rn` read
   four bytes. Widths are 0/4/C byte, 1/5/D word, 2/6/E long.
2. `bra`/`bsr` left bit 11 of the displacement positive, so backward branches
   jumped forward.
3. The `0x8x` group used the `n` field and the whole low byte: Rn/Rm is bits
   7-4 and the displacement is bits 3-0.
4. `fmov.s` was missing its post-increment and indexed forms; FR values are now
   emitted as bit patterns so infinities survive JSON.
5. `flds`/`ftrc` read the `m` field; the register is in `n`.

`tools/decode-check.py` re-proves 1–5 against objdump across the whole listing:
153,248 width nibbles, 4,379 displacements, 20,509 branch targets, 5,181
indexed FPU moves, 926 FPU/FPUL transfers, zero mismatches. Run it after any
decoder change.

## Parallel porting

One port per file means agents never edit a shared file.
`scripts/agent-workspace.sh <name>` creates an isolated copy at
`${LEMANS_WORKSPACES:-$HOME/Work/lemans-agents}/lm-<name>` (on disk, not tmpfs)
(analysis symlinked, own `rust/`, own specs and target dir).
`scripts/collect-batch.py <workspace>` copies ports and specs back and refuses to
overwrite a differing file; `scripts/verify-ports.sh` then re-runs every address
canonically. `scripts/record-batch.py` adds ledger rows only for addresses whose
evidence says `"result": "pass"`. `scripts/audit-ledger.py` checks that every
ledger row has passing evidence — a row with a *failing* evidence file is a lie.

**Do not run the full verification while collecting a batch** — it races the
collector and reports phantom failures.

## Concurrent writers

More than one worker can commit to this repo at once. Symptoms seen: a port
file deleted from the working tree while its ledger row survives, and ports
appearing with no ledger row or evidence. Before any commit run

```bash
python3 scripts/check-integrity.py
```

It proves the three-way agreement between `src/ports/p_*.rs`, the ledger in
`notes/address-map.md`, and `notes/verifications/*.json`. A port with no
passing evidence is not a claim; either finish it or remove it.

Stage explicit paths: never `git add -A`, `git add .` or `git commit -a`, which
sweep another worker's half-finished files into your commit. Port results
(`notes/verifications/`, `notes/specs/`, `notes/address-map.md`,
`rust/lemans-verify/src/ports/`, `registry.rs`) and tooling (`scripts/`,
`tools/`) go in separate commits; `scripts/git-hooks/pre-commit` rejects a commit
that mixes them (installed with `git config core.hooksPath scripts/git-hooks`).

## Crossing calls

A caller **can** be verified. The oracle runs the real bytes of the whole call
tree; the port calls the ported callee. Both sides then have to agree on every
register and store, which is exactly the check we want.

```rust
crate::registry::p_<callee>::run(cpu, mem);
```

The module path is `crate::registry::…`, not `crate::ports::…` — the ports are
declared inside `registry.rs`.

Find candidates with

```bash
python3 tools/caller-worklist.py --count 40
```

It resolves `jsr @Rn` through the most recent `mov.l <pool>,Rn` in the same
function and lists callers whose every callee is already ported. The first one
ported this way was `0x8c010084`, which calls `free` and then zeroes the
pointer.

A caller whose callee touches memory the spec does not seed fails loudly with
an oracle error, not a silent mismatch, so this is safe to attempt.

Three traps that cost ports:

- **Put `r15` well inside the window** (e.g. `0x8c308000`). A callee that
  spills below `0x8C300000` makes those writes count, and the port that tracks
  them locally then disagrees.
- **Seed everything the callee dereferences**, including the pointers it loads
  from its own literal pool and the objects it writes through (`[r4+44]` etc.).
  A null pointer there shows up as `data access 0x20`.
- **A callee that jumps through a pointer** (`jmp @Rn` fed by a table) cannot be
  pinned unless the table is seeded. `0x8c027260` is blocked for this.

## Known gaps / next work

- Oracle coverage 99.70%. The gaps are `div1` (1,001 instructions) and the vector
  FPU set (`ftrv`, `fipr`, `frchg`, `fschg`, `fsca`, `fsrra`, `fcnvsd`; 303).
  Both are left `Unimplemented` on purpose: `div1`'s non-restoring step could not
  be validated against a trusted division sequence, and the vector instructions
  need the matrix-register model. A wrong oracle is worse than a loud gap.
- Specs cannot cross calls *by themselves*, but ports can: a port calls the
  already-ported callee through `registry::p_<addr>::run` with the right `pr`
  and delay slot, so a function is portable once every callee is ported.
  `scripts/next-targets.py` resolves literal-pool and immediate call targets
  and lists the functions whose callees are all done. 1454 of
  5,211 are ported; 89 are blocked.
- No modelling of store queues, DMA, hardware registers, or interrupts. Nineteen
  leaves touching `0xa05f****`, `0xff00****` or BIOS vectors are recorded as
  blocked, as is `0x8c018244`, which branches to itself.
- `DATA.ZIP` is a reordered zip (central directory first) — 3,552 entries parsed
  by directory walk, but extraction is not implemented yet.
- Only the USA disc is analysed; the Japan SKU has different offsets.

## Useful commands

```bash
cd ~/Projects/lemans-dc
./scripts/verify-function.sh <address>        # spec -> oracle -> port -> diff
./scripts/verify-ports.sh <address>...        # canonical batch re-verification
python3 scripts/gen-registry.py               # regenerate the lookup table
python3 scripts/audit-ledger.py               # every ledger row must pass
python3 scripts/next-targets.py --limit 12    # next functions whose callees are ported
python3 scripts/fuzz-audit.py --cases 16      # randomized differential audit, all ports
./scripts/verify-all-specs.sh 12              # re-run every spec, parallel
python3 tools/decode-check.py                 # decoder differential
python3 tools/opcode-census.py analysis/export/disassembly.asm
cd rust/sh4-oracle && cargo test
```

## Rules of engagement

- Never invent evidence. If a name, struct layout or branch is ambiguous, log it
  in `notes/address-map.md` as unknown and move on.
- Never mark a function verified without `scripts/verify-function.sh` printing
  `ok` and a file in `notes/verifications/` whose `result` is `pass`.
- Do not touch the frozen disc in `work/raw/`.
- Commit after each verified port, with the side effect you had to add in the
  message — that is the knowledge that would otherwise be lost.
- A spec must fail loudly when it is wrong: system registers are top-level keys,
  FPU registers are `fr: {{"f0": ...}}`, and `lemans-verify` rejects unknown keys
  and enforces the spec's own `expect` map. Three silent-ignore holes (a
  `"special"` block, `"fr4"` inside `regs`, a typo'd key) once let specs pass
  without seeding anything.
- Hand-written specs are necessary but not sufficient: the randomized audit
  broke 53 of 1,400 ports that had passing specs, and all 53 were real. Run both.
