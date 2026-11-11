//! 0x8c21b100 — load_field_10d8.
//!
//! Returns `[[0x8c2ba634]]+0x10d8`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b170).unwrap_or(0);
    cpu.r[1] = 0x10d8;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0); // delay slot of `rts`

}
