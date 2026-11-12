//! 0x8c146640 - hash through a cached field at `[r4+0x7d4]` and store the result.
//!
//! The incoming r4 is kept in r14; the callee gets `r4 = arg2`, `r5 = arg3`,
//! `r6 = [arg1+0x7d4]`. Its result goes back into that same slot and is the
//! return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[4] = cpu.r[5]; // mov r5,r4
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[5] = cpu.r[6]; // mov r6,r5
    cpu.r[0] = mem.read_u32(0x8c146664).unwrap_or(0x7d4); // mov.l 0x8c146664,r0
    cpu.r[6] = mem.read_u32(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r14),r6
    cpu.r[0] = mem.read_u32(0x8c146668).unwrap_or(0x8c0c814c); // mov.l 0x8c146668,r0
    cpu.pr = 0x8c146654;
    crate::registry::p_8c0c814c::run(cpu, mem);
    cpu.r[1] = cpu.r[0]; // mov r0,r1
    cpu.r[0] = mem.read_u32(0x8c146664).unwrap_or(0x7d4); // mov.l 0x8c146664,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]); // mov.l r1,@(r0,r14)
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r14),r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
