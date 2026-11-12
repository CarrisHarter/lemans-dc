//! 0x8c19c664 - search with fr4 taken from fr6, then store fr4 and fr5.
//!
//! Entry `fr5` is saved in `fr15` and entry `fr4` in `fr14`. The search
//! receives `fr4 = entry fr6`. Afterwards +24 gets the saved fr4, +28 the
//! saved fr5, and the byte at +32 is cleared.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.fr[15] = cpu.fr[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[14] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.fr[4] = cpu.fr[6]; // jsr delay
    cpu.pr = 0x8c19c678;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c19c6a0).unwrap_or(0);
    cpu.r[1] = cpu.r[14].wrapping_add(24);
    cpu.r[0] = cpu.r[14].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[14]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    cpu.r[1] = 0;
    cpu.r[0] = 32;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), 0);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
