//! 0x8c10cb7c — fixpoint search over a pointer chain with a key-match exit.
//!
//! `r13 = r4` (object), `r14 = r5` (key). The frame carries two out-slots:
//! scratch `(8,r15)` and committed `(12,r15)`. Let `P = [r13+16]`.
//!
//! ```text
//! committed = **[r13+16]        (via 0x8c10c884 into scratch, then copied)
//! [r15]     = *[r13+16]         (via 0x8c10c558)
//! if committed == [r15] { return 0 }                      // done
//! loop {
//!     if **[committed+8] == r14 { return [committed+8] }  // FOUND (bra delay)
//!     committed = [committed]   (via 0x8c10c878; old value spills to (4,r15))
//!     [r15] = *[r13+16]         (via 0x8c10c558)
//!     if committed == [r15] { return 0 }                  // done
//! }
//! ```
//!
//! Byte-identical in shape to the twin at 0x8c10cae8 except the probe field
//! (`@(16,r13)` here vs `@(12,r13)` there) and the four pool words. All five
//! `jsr` callees are already ported and called directly; the stack traffic
//! (pushes, out-slots, pops) is emulated for real so `r3`/`r15` residue
//! matches. `bt`/`bf` here are plain (no delay slot); the `bra` out and every
//! `jsr`/`rts` run their delay slot. Final `T` is set: both exits leave it
//! from an equal `cmp/eq`.

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

    // Scratch = **[r13+16] via 0x8c10c884; delay slot loads r4.
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c10cc00).unwrap_or(0);
    cpu.pr = 0x8c10cb98; // jsr @r0
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(16)).unwrap_or(0);
    crate::registry::p_8c10c884::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Committed = scratch.
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[15].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);

    // [r15] = *[r13+16] via 0x8c10c558.
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c10cc04).unwrap_or(0);
    cpu.pr = 0x8c10cbb0; // jsr @r0
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(16)).unwrap_or(0);
    crate::registry::p_8c10c558::run(cpu, mem);
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
            // key = **[committed+8] via 0x8c10e374; delay slot loads r4.
            cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.r[0] = mem.read_u32(0x8c10cc08).unwrap_or(0);
            cpu.r[1] = cpu.r[1].wrapping_add(8);
            cpu.pr = 0x8c10cbc4; // jsr @r0
            cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            crate::registry::p_8c10e374::run(cpu, mem);
            cpu.set_t(cpu.r[0] == cpu.r[14]);
            if cpu.t() {
                // FOUND: r0 = [committed+8] via the bra delay slot.
                cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
                break;
            }
            // advance: committed = [committed] via 0x8c10c878 (r4 = &committed).
            cpu.r[0] = cpu.r[15].wrapping_add(4);
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[0] = mem.read_u32(0x8c10cc0c).unwrap_or(0);
            cpu.r[4] = cpu.r[15].wrapping_add(16);
            cpu.pr = 0x8c10cbde; // jsr @r0
            cpu.r[5] = 0;
            crate::registry::p_8c10c878::run(cpu, mem);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            // [r15] = *[r13+16] via 0x8c10c558.
            cpu.r[0] = cpu.r[15];
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[0] = mem.read_u32(0x8c10cc04).unwrap_or(0);
            cpu.pr = 0x8c10cbea; // jsr @r0
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(16)).unwrap_or(0);
            crate::registry::p_8c10c558::run(cpu, mem);
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
