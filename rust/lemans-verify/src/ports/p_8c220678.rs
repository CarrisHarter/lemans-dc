//! 0x8c220678 — interrupt-masked flag set: writes 1 to
//! `[[0x8c23d468]] + (*r4 * 32) + 28`, returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[3] = mem.read_u16(0x8c2206b8).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    cpu.r[5] = cpu.r[0];
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[1] = mem.read_u32(0x8c2206bc).unwrap_or(0); // 0x8c23d468
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[0] <<= 1; // shll
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.r[2] = 1;
    cpu.r[4] = cpu.r[0];
    let _ = mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[2]);
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[0] = cpu.r[5];
    cpu.r[1] = mem.read_u16(0x8c2206b8).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] &= cpu.r[1];
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[0] = 0; // rts delay slot
}
