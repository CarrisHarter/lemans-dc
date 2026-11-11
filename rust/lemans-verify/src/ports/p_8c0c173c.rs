//! 0x8c0c173c — store_field_4_via_36.
//!
//! `r0=[r4+36] ; rts ; [r0+4]=r5` — the store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0] + 4, cpu.r[5]); // delay slot of `rts`
}
