//! 0x8c19c1e0 - search with fr5, then publish a vtable and a saved word.
//!
//! The callee sees `r5 = entry r6`, `r6 = entry r7`, `fr4 = entry fr5`.
//! Afterwards +0 is 0x8c1f149c, +32 is entry fr4, +48 is entry r5, +28 is
//! 0.0 and the byte at +24 is 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[15] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4];
    cpu.r[13] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[5] = cpu.r[6];
    cpu.r[6] = cpu.r[7];
    cpu.fr[4] = cpu.fr[5];
    cpu.pr = 0x8c19c1f8;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c19c224).unwrap_or(0);
    cpu.r[1] = cpu.r[14];
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    cpu.r[1] = cpu.r[1].wrapping_add(32);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[15]);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(48), cpu.r[13]);
    cpu.fr[0] = 0.0;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[1] = 1;
    cpu.r[0] = 24;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), 1);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
