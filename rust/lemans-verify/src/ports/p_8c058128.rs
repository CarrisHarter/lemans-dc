//! 0x8c058128 — `return *(u32 *)(param_1 + 0x38);`
//!
//! `rts ; mov.l @(56,r4),r0` — the load is the `rts` delay slot, so r0 is
//! the field and nothing else changes (T untouched).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap(); // delay slot of `rts`
}
