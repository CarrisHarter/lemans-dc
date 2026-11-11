//! 0x8c146728 — indexed_load_shll2.
//!
//! `shll2 r5 ; r0=r5 ; rts ; r0=[r4+r0]` — `r5` is scaled in place and the
//! load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // delay slot
}
