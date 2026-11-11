//! 0x8c21ef74 — interrupt-masked zero-fill of two RAM tables: words at
//! 0x8c2bc9a8+12 step 16 up to +0x1000, then 0x8c2bd9a8 step 4 up to +0x300.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[3] = mem.read_u16(0x8c21f074).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    let _ = mem.write_u16(cpu.r[15], cpu.r[0] as u16); // mov.w r0,@r15
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[6] = mem.read_u16(0x8c21f076).unwrap_or(0) as i16 as i32 as u32; // 0x1000
    cpu.r[5] = 0;
    cpu.r[7] = mem.read_u32(0x8c21f084).unwrap_or(0); // 0x8c2bc9a8
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[7]);
    cpu.r[4] = cpu.r[7];
    loop {
        let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[5]);
        cpu.r[4] = cpu.r[4].wrapping_add(16);
        cpu.set_t(cpu.r[4] >= cpu.r[6]); // cmp/hs r6,r4
        if !cpu.t() {
            continue;
        }
        break;
    }
    cpu.r[6] = mem.read_u16(0x8c21f078).unwrap_or(0) as i16 as i32 as u32; // 0x300
    cpu.r[7] = mem.read_u32(0x8c21f088).unwrap_or(0); // 0x8c2bd9a8
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[7]);
    cpu.r[4] = cpu.r[7];
    loop {
        let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        cpu.set_t(cpu.r[4] >= cpu.r[6]); // cmp/hs r6,r4
        if !cpu.t() {
            continue;
        }
        break;
    }
    cpu.r[0] = mem.read_u16(cpu.r[15]).unwrap_or(0) as u32; // mov.w @r15,r0
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[0] &= 0xffff; // extu.w r0,r0
    cpu.r[2] = mem.read_u16(0x8c21f074).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot
}
