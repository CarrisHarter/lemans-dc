//! 0x8c0560f8 — clamp each of the four floats at [r4+0..r4+12] into [0,1],
//! in place, processing +4, +8, +12, then +0.
//!
//! Per element the original does: `fr0 = f; fr1 = 0; fcmp/gt fr0,fr1` (T = 0
//! > f) — if f < 0 keep fr1 = 0; else `fr1 = 1; fcmp/gt fr1,fr0` (T = f > 1) —
//! if f > 1 keep fr1 = 1, else fr1 = f; then `fmov fr1,@(r4+off)`.
//!
//! Side effects the C does not show: r0 ends at r4+12, r1 at r4+8 (scratch
//! pointers from the `mov r4,rN` before each store), fr0 ends as the raw
//! [r4] value, fr1 as its clamp, the final store at [r4] is the `rts` delay
//! slot, and T ends as the *last* element's comparison: T = (f < 0) ? 1 :
//! (f > 1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    for off in [4u32, 8, 12] {
        let f = f32::from_bits(mem.read_u32(r4.wrapping_add(off)).unwrap_or(0));
        cpu.fr[0] = f; // fmov @r0,fr0
        cpu.fr[1] = 0.0; // fldi0 fr1
        if !(0.0f32 > f) {
            // fcmp/gt fr0,fr1 was false -> second check
            cpu.fr[1] = 1.0; // fldi1 fr1
            if !(f > 1.0) {
                cpu.fr[1] = f; // fmov fr0,fr1
            }
        }
        let _ = mem.write_u32(r4.wrapping_add(off), cpu.fr[1].to_bits()); // fmov fr1,@rN
    }
    // last element at [r4]: load, clamp, store in the rts delay slot
    let f = f32::from_bits(mem.read_u32(r4).unwrap_or(0));
    cpu.fr[0] = f; // fmov @r4,fr0
    cpu.fr[1] = 0.0;
    let mut t = 0.0f32 > f; // fcmp/gt fr0,fr1
    if !t {
        cpu.fr[1] = 1.0;
        t = f > 1.0; // fcmp/gt fr1,fr0
        if !t {
            cpu.fr[1] = f;
        }
    }
    let _ = mem.write_u32(r4, cpu.fr[1].to_bits()); // rts delay slot: fmov fr1,@r4
    cpu.set_t(t);
    cpu.r[0] = r4.wrapping_add(12); // mov r4,r0 ; add #12,r0 (last block)
    cpu.r[1] = r4.wrapping_add(8); // mov r4,r1 ; add #8,r1 (second block)
}
