//! 0x8c19c47c - search with the caller's r6/r7 and fr7.
//!
//! Saved fr4/fr5/fr6 land at +24/+28/+32. The byte at +48 is the low byte
//! of entry r5. Vtable becomes 0x8c1f14b8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.fr[14] = cpu.fr[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[13] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.fr[15] = cpu.fr[6];
    cpu.r[5] = cpu.r[6];
    cpu.r[6] = cpu.r[7];
    cpu.fr[4] = cpu.fr[7];
    cpu.pr = 0x8c19c49c;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[4] = mem.read_u32(0x8c19c4cc).unwrap_or(0);
    cpu.r[3] = cpu.r[13];
    cpu.r[1] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[13], cpu.r[4]);
    cpu.r[3] = cpu.r[3].wrapping_add(24);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[13]);
    cpu.r[1] = cpu.r[1].wrapping_add(28);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[14]);
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    cpu.r[0] = 48;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), (cpu.r[14] & 0xff) as u8);
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
