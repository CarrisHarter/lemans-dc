//! 0x8c0f1508 — indexed_load_offset_434.
//!
//! `r1=[r4] ; r0=0x434 ; rts ; r0=[r0+r1]` — a table load at `0x434 + *r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8C0F_1510).unwrap_or(0x434);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0); // delay slot
}
