//! 0x8c0c1720 — store_field_24_via_36.
//!
//! `r0=[r4+36] ; rts ; [r0+24]=r5` — the store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0] + 24, cpu.r[5]); // delay slot of `rts`
}
