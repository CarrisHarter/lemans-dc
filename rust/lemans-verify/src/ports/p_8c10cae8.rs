//! 0x8c10cae8 — fixpoint search over a pointer chain with a key-match exit.
//!
//! `r13 = r4` (object), `r14 = r5` (key). The frame carries two out-slots:
//! scratch `(8,r15)` and committed `(12,r15)`. Let `P = [r13+12]`.
//!
//! ```text
//! committed = **[r13+12]        (via 0x8c10c7bc into scratch, then copied)
//! [r15]     = *[r13+12]         (via 0x8c10c484)
//! if committed == [r15] { return 0 }                      // done
//! loop {
//!     if **[committed+8] == r14 { return [committed+8] }  // FOUND (bra delay)
//!     committed = [committed]   (via 0x8c10c7b0; old value spills to (4,r15))
//!     [r15] = *[r13+12]         (via 0x8c10c484)
//!     if committed == [r15] { return 0 }                  // done
//! }
//! ```
//!
//! All five `jsr` callees are already ported and called directly; the stack
//! traffic (pushes, out-slots, pops) is emulated for real so `r3`/`r15`
//! residue matches. `bt`/`bf` here are plain (no delay slot); the `bra` out
//! and every `jsr`/`rts` run their delay slot. Final `T` is set: both exits
//! leave it from an equal `cmp/eq`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: push r14, r13, pr; open a 16-byte frame.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[13] = cpu.r[4];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[0]);

    // Scratch = **[r13+12] via 0x8c10c7bc; delay slot loads r4.
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c10cb6c).unwrap_or(0);
    cpu.pr = 0x8c10cb04; // jsr @r0
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
    crate::registry::p_8c10c7bc::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Committed = scratch.
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[15].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);

    // [r15] = *[r13+12] via 0x8c10c484.
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c10cb70).unwrap_or(0);
    cpu.pr = 0x8c10cb1c; // jsr @r0
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
    crate::registry::p_8c10c484::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Fixpoint? committed == [r15].
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == cpu.r[1]);
    if cpu.t() {
        // done
        cpu.r[0] = 0;
    } else {
        loop {
            // key = **[committed+8] via 0x8c10610c; delay slot loads r4.
            cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.r[0] = mem.read_u32(0x8c10cb74).unwrap_or(0);
            cpu.r[1] = cpu.r[1].wrapping_add(8);
            cpu.pr = 0x8c10cb30; // jsr @r0
            cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            crate::registry::p_8c10610c::run(cpu, mem);
            cpu.set_t(cpu.r[0] == cpu.r[14]);
            if cpu.t() {
                // FOUND: r0 = [committed+8] via the bra delay slot.
                cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
                break;
            }
            // advance: committed = [committed] via 0x8c10c7b0 (r4 = &committed).
            cpu.r[0] = cpu.r[15].wrapping_add(4);
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[0] = mem.read_u32(0x8c10cb78).unwrap_or(0);
            cpu.r[4] = cpu.r[15].wrapping_add(16);
            cpu.pr = 0x8c10cb4a; // jsr @r0
            cpu.r[5] = 0;
            crate::registry::p_8c10c7b0::run(cpu, mem);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            // [r15] = *[r13+12] via 0x8c10c484.
            cpu.r[0] = cpu.r[15];
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[0] = mem.read_u32(0x8c10cb70).unwrap_or(0);
            cpu.pr = 0x8c10cb56; // jsr @r0
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
            crate::registry::p_8c10c484::run(cpu, mem);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            // Fixpoint? committed == [r15].
            cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == cpu.r[1]);
            if cpu.t() {
                // done
                cpu.r[0] = 0;
                break;
            }
        }
    }

    // Epilogue: drop the frame, pop pr, r13, r14 (rts delay slot).
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
