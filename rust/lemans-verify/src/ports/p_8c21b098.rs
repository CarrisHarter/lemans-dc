//! 0x8c21b098 — load_field_10c4.
//!
//! Returns `[[0x8c2ba634]+0x10c4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b170).unwrap_or(0);
    cpu.r[1] = mem.read_u16(0x8c21b158).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0); // delay slot of `rts`
}
