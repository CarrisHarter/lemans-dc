//! 0x8c21c3e0 — set_high_bit_on_field.
//!
//! ORs `0x80000000` into `[[[0x8c2ba634]]+0xfcc]` and returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21c430).unwrap_or(0);
    cpu.r[0] = 0xfcc;
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] = mem.read_u32(0x8c21c434).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[1] = cpu.r[1] | cpu.r[2];
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    cpu.r[0] = 0; // delay slot of `rts`
}
