//! 0x8c0f1138 — advance_to_limit.
//!
//! `r3 = [r4+8] + ([r4+4]<<2)` then `r1` steps by 4 until it is not below
//! `r3`; `[r4+4]` is then cleared and `r0` is 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = mem.read_u32(base + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(base + 4).unwrap_or(0);
    cpu.r[3] = cpu.r[1];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    let mut above = cpu.r[3] > cpu.r[1];
    cpu.set_t(above); // cmp/hi r1,r3
    while above {
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        above = cpu.r[3] > cpu.r[1];
        cpu.set_t(above); // cmp/hi r1,r3
    }
    cpu.r[0] = 0;
    let _ = mem.write_u32(base + 4, 0); // delay slot of `rts`
}
