//! 0x8c19c92c - search with fr5, store fr4, clear the word at +60.
//!
//! Vtable becomes 0x8c1f150c. +24 receives entry fr4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[15] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.fr[4] = cpu.fr[5];
    cpu.pr = 0x8c19c93c;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[1] = mem.read_u32(0x8c19c958).unwrap_or(0);
    cpu.r[0] = cpu.r[14].wrapping_add(24);
    let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(60), cpu.r[0]);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
