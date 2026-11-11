//! 0x8c0ab080 — indexed_load.
//!
//! `r1=[r4] ; r5<<=2 ; r0=r5 ; rts ; r0=[r1+r0]` — `r5` is shifted in place and
//! the load is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0); // delay slot
}
