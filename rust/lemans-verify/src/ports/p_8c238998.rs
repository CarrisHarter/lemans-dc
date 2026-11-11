//! 0x8c238998 — init_or_reset_global.
//!
//! Stores `r4` at `0x8c2c873c` and returns 0 when `r4` is non-null; otherwise
//! stores `0x8c233800` there and returns `0xeeee`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[2] = mem.read_u32(0x8c2389d8).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[2], cpu.r[4]);
        cpu.r[0] = 0; // delay slot of `rts`
        return;
    }
    cpu.r[3] = mem.read_u32(0x8c2389d8).unwrap_or(0);
    cpu.r[2] = mem.read_u32(0x8c2389dc).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[0] = mem.read_u32(0x8c2389e0).unwrap_or(0);
}
