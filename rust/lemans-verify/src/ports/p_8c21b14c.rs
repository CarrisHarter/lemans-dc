//! 0x8c21b14c — load_field_fb0.
//!
//! Returns `[[0x8c2ba634]]+0xfb0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b170).unwrap_or(0);
    cpu.r[1] = 0xfb0;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0); // delay slot of `rts`

}
