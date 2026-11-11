//! 0x8c07a27c — init_record.
//!
//! Stores the global `[0x8c1f8580]` at `[r4]`, then zeroes `r4+4..r4+16`,
//! `r4+20` and `r4+24`. `r4` ends as that global and `r3` as `r4+4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = base;
    cpu.r[3] = mem.read_u32(0x8C07_A2A4).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[3] = cpu.r[0];
    let _ = mem.write_u32(base, cpu.r[4]);
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    let _ = mem.write_u32(base + 4, 0);
    let _ = mem.write_u32(base + 8, 0);
    let _ = mem.write_u32(base + 12, 0);
    let _ = mem.write_u32(base + 16, 0);
    let _ = mem.write_u32(base + 20, 0);
    let _ = mem.write_u32(base + 24, 0);
}
