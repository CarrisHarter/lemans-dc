# Scope decision: "Linkable C"

Chosen exit criterion (user, 2026-11-11): **linkable C**, not pseudocode-only.

## What that costs on SH-4

"Linkable" here means each recovered function becomes C that compiles and links
into a Dreamcast-targeted ELF, with behaviour verified against the original.
Byte-identical *matching* is a strictly harder target, and it is out of reach
for this title: retail Katana binaries were produced by Hitachi's GCC 2.9x/3.x
with Katana libraries, and modern `sh-elf-gcc` 15 emits different code for the
same source. Without the original compiler, flags and headers, an exact match
cannot be reconstructed — which is exactly why community *matching* decomps of
Dreamcast games take years and are only feasible when the build can be recreated.

So the acceptance test becomes: **same observable behaviour**, measured by a
harness, not identical instruction bytes.

## Toolchain status on this machine

| piece | state |
| --- | --- |
| `sh4-elf-binutils` 2.45.1 (as/ld/objdump/nm) | **installed** from `extra` |
| `sh-elf-gcc` 15.2.0 (freestanding, stage 1) | **building** from the GCC 15.2.0 release tarball via `scripts/07-build-sh4-cross-gcc.sh`, log `/tmp/sh4-cross-build.log` |
| `sh-elf-newlib` | skipped: the AUR packages are circular (`sh-elf-gcc` makedepends `sh-elf-newlib`, which depends on `sh-elf-gcc`), and decompiled functions are freestanding C |
| `objdiff` (encounter/objdiff, Rust) | available via `cargo install`, only needed if matching ever becomes the goal |
| Ghidra 12.1.4 + JDK 21 | installed and verified |
| rea with the `sh4` lane | patched, tested, registered as MCP |

The AUR `sh-elf-*` route looked cheapest but does not resolve: yay refuses the
`sh-elf-gcc` <-> `sh-elf-newlib` makedepends cycle, and the PKGBUILD needs newlib
only because it configures `--with-sysroot=/usr/sh-elf`. Stage-1 GCC with
`--without-headers --with-newlib` produces a compiler that is sufficient to
compile and link recovered functions, so that is what is being built.

`sh4-elf-binutils` is the assembler/linker pair that makes recovered C linkable;
`sh-elf-gcc` is what compiles it. Note the Dreamcast ABI needs `-m4-single-only`
(FPSCR.PR=0, single-precision), which is also what the game's own FPU code
assumes.

## Harness options, evaluated

1. **DecompPipeline (`GeneralAtrox/DecompPipeline`)** — has a real
   `targets/dreamcast_sh4`, an SH-4 translator, a per-function judge, and ships
   its own Ghidra SH-4 language installer (`targets/dreamcast_sh4/ghidra_lang/install_lang.py`),
   which may decompile better than Ghidra's built-in SH-4a module.
   **Blocker:** its judge compiles every candidate with **MSVC / Visual Studio
   2022**, and setup assumes Windows (`Dashboard.bat`, `vcvars`). It also wants
   Python 3.12 and the `claude` CLI. Not usable as-is on this Linux-only box;
   usable in a Windows VM or on the Windows side of a dual-boot.
2. **Own functional harness (recommended here)** — decompile a function, rename
   and type its structs, emit C, compile with `sh-elf-gcc -m4-single-only`,
   then compare against the original routine: pure functions in `qemu-sh4`
   user mode, whole-game behaviour in Flycast. Risks to encode up front, taken
   from DecompPipeline's own lesson list: `fmac`/`fipr`/`ftrv` rounding,
   FPSCR.SZ (64-bit moves) per calling context, results returned in r4..r7,
   hand-written assembly modules, and store-queue writes.
3. **PC-port route** — *Le Mans 24 Hours* also shipped on Windows/PS2/Xbox.
   An x86 build would need **no cross toolchain, no SH-4 harness, and no rea
   patch** (x86 is a first-class rea lane), and host-native testing is far
   cheaper. If a legitimate copy is obtainable, this is the cheapest path to
   linkable C by a wide margin. PS2 would be MIPS (better tooling than SH-4 in
   rea), Xbox would be x86 (best).

## Decision

- Stay on Dreamcast SH-4 for now: the toolchain is nearly ready and the rea
  lane is done.
- Do **not** start DecompPipeline on this machine; its judge is MSVC-bound.
- Build the small functional harness myself, and validate it *before* any real
  binary by round-tripping synthetic SH-4: C → `sh-elf-gcc` → object →
  decompile with rea → recompile → compare.
- Revisit the PC/x86 route with the user, because it removes the entire SH-4
  toolchain and harness risk.

## Still blocked

The disc image. Everything above is preparation; no real function can be
recovered until a dump exists at `work/raw/`.


## Cross compiler: what actually worked

- Built stage-1 freestanding GCC 15.2.0 at `~/opt/sh-elf` (`scripts/07-build-sh4-cross-gcc.sh`).
  The AUR `sh-elf-gcc` route is unusable: `sh-elf-gcc` makedepends `sh-elf-newlib`
  and vice versa, and the PKGBUILD needs newlib only because it configures
  `--with-sysroot`.
- Host GCC 16 makes libcody's `u8"..."` literals `char8_t` under C++20, breaking its
  `S2C()`/`std::string` comparisons in both `.cc` files **and** `cody.hh`. The rewrite
  must cover headers; `.cc`-only leaves `cody.hh:113 S2C(const char8_t [2])`.
- The driver has no target program prefix, so it resolves `as` to the **host x86**
  assembler and fails with `as: unrecognized option '-big'`. Fixed with `-B $HOME/opt/sh-elf/toolbin`,
  a directory of target-named symlinks (`as -> /usr/bin/sh-elf-as`, …). `-print-prog-name=as`
  is the diagnostic.
- **`sh-elf` defaults to big-endian.** Every object must be built with `-ml`; without it
  the bytes are byte-swapped relative to the Dreamcast and every instruction and pointer
  is wrong.
- **`-m4-single-only` is mandatory for float code.** Without it GCC emits *soft-float
  library calls* (`__floatsisf`, `__addsf3`, `__mulsf3`), not double-precision FPU —
  verified by 4 relocations against those helpers. The Dreamcast runs with FPSCR.PR=0.
- Harness (`scripts/12-harness.py`) enforces all of the above and warns on
  `fmac`/`fipr`/`ftrv`/`fschg`/`flds` rounding traps.

## Matching vs equivalence, measured

Original fixture `sum_to` (hand-assembled) and the same algorithm compiled by SH-4 GCC
differ in shape but agree in mechanism:

```
original:  mov r1,r4 ; mov #0,r0 ; add #1,r0 ; dt r1 ; bf .-6 ; rts ; nop
compiled:  cmp/pl r4 ; bf.s +0x14 ; mov #0,r0 ; mov r4,r1 ; dt r1 ; add r4,r0 ; bf.s .-8 ; rts
```

Both are `dt`-driven back-edge loops with the accumulator in r0; neither is byte-comparable.
This is the baseline expectation for the equivalence harness: same mechanism, different code.
