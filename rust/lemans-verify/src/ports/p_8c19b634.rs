//! 0x8c19b634 - cache the 0x8c16ea60 result at `[r4+100]`, then reset with fr4 = 0.
//!
//! `0x8c16ea60(r4)` returns in fr0, which is stored at `r14+100`. Then
//! 0x8c16ee30 runs with `fr4 = +0.0` (loaded by the `fldi0` in its delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c19b654).unwrap_or(0x8c16ea60); // mov.l 0x8c19b654,r0
    cpu.pr = 0x8c19b640; // jsr delay is nop
    crate::registry::p_8c16ea60::run(cpu, mem);
    cpu.r[0] = cpu.r[14].wrapping_add(100); // mov r14,r0 ; add #100,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[0].to_bits()); // fmov fr0,@r0
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    cpu.r[0] = mem.read_u32(0x8c19b658).unwrap_or(0x8c16ee30); // mov.l 0x8c19b658,r0
    cpu.fr[4] = 0.0; // jsr delay: fldi0 fr4
    cpu.pr = 0x8c19b64e;
    crate::registry::p_8c16ee30::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
