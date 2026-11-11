//! 0x8c155e4c — init_from_rom_block.
//!
//! Publishes `r4` to `0x8c2abd50`, zeroes `byte[r4+16]`, copies four words from
//! `0x8c1d0f18`, then overwrites `[r4]` with 0.0 in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c155e70).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.fr[0] = 0.0;
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
    cpu.r[0] = 16;
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
    cpu.r[5] = mem.read_u32(0x8c155e74).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // delay slot of `rts`
}
