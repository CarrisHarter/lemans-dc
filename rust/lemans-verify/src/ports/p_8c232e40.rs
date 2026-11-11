//! 0x8c232e40 — interrupt-masked slot init: writes r5, r6, 0, r4 at offsets
//! 0/12/4/8 of slot `r4` (stride 16) in the table at 0x8c2c8b08+48. Returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[3] = mem.read_u16(0x8c232ea0).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[7] = mem.read_u32(0x8c232ea4).unwrap_or(0); // 0x8c2c8b08
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[0] = cpu.r[4];
    cpu.r[2] = cpu.r[7];
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] = cpu.r[7];
    cpu.r[2] = cpu.r[2].wrapping_add(48);
    cpu.r[0] <<= 2; // shll2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(48);
    let _ = mem.write_u32(cpu.r[2], cpu.r[5]);
    cpu.r[2] = cpu.r[7];
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[2] = cpu.r[2].wrapping_add(48);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[6]);
    cpu.r[3] = 0;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[0]);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(4), cpu.r[3]);
    cpu.r[2] = cpu.r[7];
    cpu.r[2] = cpu.r[2].wrapping_add(48);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u16(0x8c232ea0).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    let _ = mem.write_u32(cpu.r[0].wrapping_add(8), cpu.r[4]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[0] = 0;
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot
}
