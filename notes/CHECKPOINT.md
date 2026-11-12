# Checkpoint — 2026-11-12

Stopping point for an unattended port run. Everything below was verified at the
moment of the checkpoint; nothing is claimed from memory.

## State

| metric | value |
| --- | --- |
| verified ports | **1,832 of 5,211 (35.2%)** |
| spec files | 4,051 |
| evidence files | 3,561 |
| full-spec sweep | `specs 4042 failures 0` |
| integrity | `ok: ports 1832, ledger 1832, passing evidence 1832, blocked 90` |
| oracle tests | 21 pass (`cargo test` in `rust/sh4-oracle`) |
| tree | clean, everything pushed |
| last commit | `87c53ecb` |

Ready-to-port queue: 225 leaf functions under 400 instructions, median 66.
Ports in this run: 101 (batches 44–57), all proven by `verify-function.sh` and
recorded with step counts in `notes/address-map.md`.

## What unblocked the throughput

Specs cannot cross calls, so a function is only eligible once every callee is
already ported. That makes the queue self-feeding: porting a leaf unlocks its
callers. The whole run was driven by `python3 scripts/next-targets.py`, which
prints exactly that eligible set.

## Wave 14 — dispatched, then stopped (nothing landed)

Killed at checkpoint so nothing could land half-written. All five are fresh
targets with verified callees:

| address | shape |
| --- | --- |
| `0x8c10e3e4` | record teardown, 0x8c018bd8 releases + free |
| `0x8c111794` | guarded insertion sort (`cmp/pl` then lexcmp) |
| `0x8c14b070` | 5-callee free fan-out |
| `0x8c14fe94` | 5-callee teardown |
| `0x8c16d690` | memset + 0x8c017f48, 78 insns |

## Next up, already decoded

`0x8c176512` (30 insns) was disassembled past its inline literal pool
(`0x8c17651e..0x8c17654e` is data). Real code resumes at `0x8c176550`:

```
r4==0                 -> r5 = str@0x8c1e18cc          -> emit(101), r0 = 0
r4!=0, r5==0          -> r5 = [0x8c176634]            -> emit(101), r0 = 0
otherwise             -> r0 = [r4 + mov.w@0x8c17662e]
                         cmp/eq #3 -> r3 = 1, bt.s (delay r6 = r0)
                         r6 == 0   -> r3 = 1
                         else      -> r3 = 0
                         [r5] = r3, r0 = 1
```

So it is a three-exit flag reporter: `[r5] = ([r4+field] == 3 || field == 0)`.
The emit callee is `0x8c174564` (verified).

## Lessons worth keeping

1. **Hand-written branch summaries invert directions.** Eight dispatches had
   swapped taken/not-taken sides. Every port brief must say "reconfirm from
   `disassembly.asm`"; agents that did caught all eight.
2. **Inline literal pools break naive sweeps.** `0x8c172ee2` and `0x8c176512`
   both have data between `bra` and the resume point. Cross-check any window
   that contains `.word` runs against `objdump` before trusting it.
3. **Kill stalled agents early.** Two agents sat ≥4 min at "writing the port"
   with no file. Replacing them (or porting inline) finished the same functions
   in about a minute each.
4. **Killed agents can still write.** A cancelled agent overwrote an already
   committed port and specs; `git status` + `git diff HEAD` caught it and the
   verified commit was restored. Always re-check the tree after a kill, and
   re-run the verifier on any file that changed.
5. **Spec seeds are little-endian hex bytes.** Two transposed byte pairs
   (`0001408c` for `0x8c240100`) silently produced different addresses and both
   sides still matched — a vacuous pass. Prefer generating seeds with
   `struct.pack("<I", value).hex()`.
6. **`mov.w` sign-extends.** The `0xff0f` SR mask becomes `0xffffff0f`, which
   makes the SR update OR-only. Applies to every SR-merge port.
7. **Dead arms exist.** In `0x8c1825d8` the 2³² fixup is unreachable because
   `extu.w` clamps the converted value to `0..0xFFFF`, so `fr1 >= 0 = fr0`
   always. The port mirrors the arm; no spec can take it.
8. **Unmapped reads are unverifiable.** `0x8c1739c0` reads `0x8c004020`, below
   `LOAD_BASE`; the oracle faults before a diff can mean anything, and spec
   patches to unmapped addresses are dropped. Recorded in the Blocked ledger,
   not retried.

## Resume

```bash
cd ~/Projects/lemans-dc
git status --short                 # expect clean
python3 scripts/check-integrity.py # expect ok: ports 1832 ...
python3 scripts/next-targets.py --limit 200 --max-insns 60
```

Then run the loop: spec -> port -> `./scripts/verify-function.sh <addr>` ->
ledger row -> `fuzz-audit.py --only <addr>` -> commit. Dispatch agents in
batches of five on disjoint addresses; never let two agents own one address.
