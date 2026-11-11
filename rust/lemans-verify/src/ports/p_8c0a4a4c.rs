//! 0x8c0a4a4c — zero_struct.
//!
//! Zeroes `[r4]`, `+4`, `+8`, `+12`, `+16`, the byte at `+20` and `+24`, then
//! returns `r4` in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = 0;
    let _ = mem.write_u32(base, 0);
    cpu.r[0] = 20;
    let _ = mem.write_u32(base + 4, 0);
    let _ = mem.write_u32(base + 8, 0);
    let _ = mem.write_u32(base + 12, 0);
    let _ = mem.write_u32(base + 16, 0);
    let _ = mem.write_u8(base + 20, 0);
    let _ = mem.write_u32(base + 24, 0);
    cpu.r[0] = base; // delay slot of `rts`
}
