//! 0x8c10fe08 — byte[r5+0x50] = byte[r6+0x15c]; r5 += 32; copy 4 halfwords r6+0x94..0x9a → stack, then stack → r5+0..6.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let b = mem.read_u8(cpu.r[6] + 0x15c).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[5] + 0x32, b);
    cpu.r[5] = cpu.r[5].wrapping_add(32);
    let mut stack = [0u16; 4];
    for i in 0..4u32 {
        stack[i as usize] = mem.read_u16(cpu.r[6] + 0x94 + i * 2).unwrap_or(0);
    }
    for i in 0..4u32 {
        let _ = mem.write_u16(cpu.r[5] + i * 2, stack[i as usize]);
    }
    cpu.r[0] = stack[3] as i16 as u32; // mov.w: sign-extend halfword
    cpu.r[1] = cpu.r[15].wrapping_sub(8); // r15 after add #-8
    cpu.r[3] = b as u32;
}
