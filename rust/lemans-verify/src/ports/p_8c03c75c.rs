//! 0x8c03c75c — store fr7/fr4/fr5/fr6 as four consecutive floats at param_1,
//! return param_1.
//!
//! ```text
//! mov r4,r3 ; fmov fr7,@r4 ; add #4,r3 ; mov r4,r1 ; mov r4,r0
//! fmov fr4,@r3 ; add #8,r1 ; fmov fr5,@r1 ; add #12,r0 ; fmov fr6,@r0
//! rts ; mov r4,r0          ; delay slot: r0 = r4
//! ```
//! Side effects the C does not show: r3 is left at r4+4, r1 at r4+8 (the
//! C only shows the four stores and the return); the return value itself is
//! produced by the `mov r4,r0` in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[3] = base; // mov r4,r3
    let _ = mem.write_u32(base, cpu.fr[7].to_bits()); // fmov fr7,@r4
    cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
    cpu.r[1] = base; // mov r4,r1
    cpu.r[0] = base; // mov r4,r0
    let _ = mem.write_u32(cpu.r[3], cpu.fr[4].to_bits()); // fmov fr4,@r3
    cpu.r[1] = cpu.r[1].wrapping_add(8); // add #8,r1
    let _ = mem.write_u32(cpu.r[1], cpu.fr[5].to_bits()); // fmov fr5,@r1
    cpu.r[0] = cpu.r[0].wrapping_add(12); // add #12,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[6].to_bits()); // fmov fr6,@r0
    cpu.r[0] = base; // delay slot of `rts`: mov r4,r0
}
