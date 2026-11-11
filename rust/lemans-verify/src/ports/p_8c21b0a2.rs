//! 0x8c21b0a2 — store_field_10cc.
//!
//! `[0x8c2ba634]+0x10cc = r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c21b170).unwrap_or(0);
    cpu.r[0] = mem.read_u16(0x8c21b15a).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[4]); // delay slot of `rts`
}
