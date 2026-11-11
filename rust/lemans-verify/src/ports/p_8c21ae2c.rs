//! 0x8c21ae2c — set_enable_and_clear_flag.
//!
//! Stores 1 at `[[0x8c2ba634]]+0xfc0`, then clears the byte behind the
//! pointer at `0x8c21c440` and returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c21aed0).unwrap_or(0);
    cpu.r[1] = 1;
    cpu.r[0] = mem.read_u16(0x8c21aec6).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[2] = 0;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1]);
    cpu.r[1] = mem.read_u32(0x8c21aecc).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[3], cpu.r[2] as u8);
    cpu.r[0] = cpu.r[2]; // delay slot of `rts`
}
