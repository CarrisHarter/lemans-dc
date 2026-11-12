//! 0x8c222964 - bitmask of indices 0..7 for which 0x8c222430 returns nonzero.
//!
//! `r14` advances in the `bt.s` delay slot, so the compare sees the next index.
//! The last `shll` of `0x80` sets T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = 1; // mov #1,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[11] = 8; // mov #8,r11
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[10] = mem.read_u32(0x8c222a94).unwrap_or(0x8c222430);
    loop {
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c222980;
        crate::registry::p_8c222430::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        cpu.r[14] = cpu.r[14].wrapping_add(1); // bt.s delay: add #1,r14
        if !cpu.t() {
            cpu.r[12] |= cpu.r[13]; // or r13,r12
        }
        cpu.set_t(cpu.r[13] >> 31 != 0); // shll r13
        cpu.r[13] <<= 1;
        cpu.set_t((cpu.r[14] as i32) >= (cpu.r[11] as i32)); // cmp/ge r11,r14
        if cpu.t() {
            break;
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.r[12]; // mov r12,r0
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
