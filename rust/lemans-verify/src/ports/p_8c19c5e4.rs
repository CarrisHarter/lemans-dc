//! 0x8c19c5e4 - search with fr4 forced to 0, then store the saved fr4 twice.
//!
//! Vtable becomes 0x8c1f14d4. The entry `fr4` is written at +24 and +28,
//! and the byte at +32 is cleared.

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
    cpu.fr[4] = 0.0;
    cpu.pr = 0x8c19c5f4;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c19c618).unwrap_or(0);
    cpu.r[1] = cpu.r[14].wrapping_add(24);
    cpu.r[0] = cpu.r[14].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[15]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    cpu.r[1] = 0;
    cpu.r[0] = 32;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), 0);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
