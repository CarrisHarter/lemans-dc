//! 0x8c059054 — `return *(u32 *)(*(u32 *)(param_1 + 0x44) + param_2 * 4);`
//!
//! `add #68,r4 ; mov.l @r4,r1 ; shll2 r5 ; mov r5,r0 ; rts ; mov.l @(r0,r1),r0`
//! The C hides: r4 is permanently advanced by 0x44, r1 keeps the loaded base
//! pointer, r5 is destroyed by `shll2` (which leaves T alone), and the final
//! load rides in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(68); // add #68,r4
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r1
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5 (T unchanged)
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap(); // delay slot of `rts`
}
