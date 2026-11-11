//! 0x8c0b766c — init_three_vtables.
//!
//! Stores three pool words at `[r4]` in order (the last wins), then `r5` at
//! `[r4+4]`, and returns `r4` from the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    for pool in [0x8C0B_7680u32, 0x8C0B_7684, 0x8C0B_7688] {
        cpu.r[0] = mem.read_u32(pool).unwrap_or(0);
        let _ = mem.write_u32(base, cpu.r[0]);
    }
    let _ = mem.write_u32(base + 4, cpu.r[5]);
    cpu.r[0] = base; // delay slot of `rts`
}
