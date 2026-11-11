//! 0x8c0b9f78 — indexed_table_then_field.
//!
//! `r1=[r4] ; r1<<=2 ; r0=[0x8c27a028 + r1] ; rts ; r0=[r0+36]` — two levels of
//! indirection through a table at 0x8c27a028.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8C0B_9F84).unwrap_or(0);
    cpu.r[1] = cpu.r[1] << 2;
    cpu.r[0] = mem
        .read_u32(cpu.r[0].wrapping_add(cpu.r[1]))
        .unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 36).unwrap_or(0); // delay slot of `rts`
}
