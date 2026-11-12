//! 0x8c21b034 - compare `r5` against word 32 of slot `r4`.
//!
//! 0x8c21afd0(r4) returns the slot. Returns 1 when `[slot+32] == r5`, else 0.
//! T is the `cmp/eq`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]); // bsr delay: mov.l r5,@r15
    cpu.pr = 0x8c21b03c;
    crate::registry::p_8c21afd0::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0); // mov.l @(32,r4),r3
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
    cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r2,r3
    cpu.r[0] = u32::from(cpu.t()); // movt r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
