//! 0x8c19c7a4 - same stores as 0x8c19c764, but the search gets fr4 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[15] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[5] = cpu.r[6];
    cpu.r[6] = cpu.r[7];
    cpu.fr[4] = 0.0;
    cpu.pr = 0x8c19c7bc;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c19c7e0).unwrap_or(0);
    cpu.r[1] = cpu.r[13].wrapping_add(24);
    let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
    cpu.fr[0] = 0.0;
    let _ = mem.write_u32(cpu.r[13].wrapping_add(28), cpu.r[14]);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[15]);
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
