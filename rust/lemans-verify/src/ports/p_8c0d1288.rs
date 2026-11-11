//! 0x8c0d1288 — indexed_pool_load.
//!
//! `r0=[0x8C0D_1290] ; r4<<=2 ; rts ; r0=[r0+r4]` — a table lookup with the
//! index shifted in place.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0D_1290).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0); // delay slot
}
