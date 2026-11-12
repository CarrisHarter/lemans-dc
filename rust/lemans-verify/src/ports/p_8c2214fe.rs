//! 0x8c2214fe — index into a table after the 0x8c221da8 check.
//!
//! `jsr` 0x8c221da8 with the caller's r4 (spilled to the scratch word in the
//! delay slot). Non-zero → return -1. Zero → `r0 = scratch` (the preserved r4),
//! `r1 = [0x8c221600]` (0x8c2bde78), `r0 <<= 2`, `r0 = [r1 + r0]`; the stack is
//! popped and `rts` returns that value. T is the `tst` result on the -1 arm and
//! is untouched by the table load otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[3] = mem.read_u32(0x8c2215e8).unwrap_or(0x8c221da8); // mov.l 0x8c2215e8,r3
    let saved = cpu.r[4];
    let _ = mem.write_u32(cpu.r[15], saved); // delay slot: mov.l r4,@r15
    cpu.pr = 0x8c221508;
    crate::registry::p_8c221da8::run(cpu, mem); // jsr @r3 ; nop
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = u32::MAX; // mov #-1,r0
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    } else {
        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(saved); // mov.l @r15,r0
        cpu.r[1] = mem.read_u32(0x8c221600).unwrap_or(0x8c2bde78); // mov.l 0x8c221600,r1
        cpu.r[0] <<= 2; // shll2 r0
        cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r1),r0
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
