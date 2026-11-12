//! 0x8c19c61c - search with r6/r7 and fr6, store fr4, fr5 and a flag byte.
//!
//! +24 is entry fr4, +28 is entry fr5, and the byte at +32 is the low byte
//! of entry r5. Vtable becomes 0x8c1f14d4.

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
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[5] = cpu.r[6];
    cpu.r[6] = cpu.r[7];
    cpu.fr[4] = cpu.fr[6];
    cpu.pr = 0x8c19c638;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c19c660).unwrap_or(0);
    cpu.r[1] = cpu.r[13];
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[13], cpu.r[3]);
    cpu.r[1] = cpu.r[1].wrapping_add(24);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[14]);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    cpu.r[0] = 32;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), (cpu.r[14] & 0xff) as u8);
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
