//! 0x8c16eadc — store_indexed_through_two_fields.
//!
//! `add #92,r4 ; r1=[r4] ; r0=[0x8c16eae8] ; rts ; [r0+r1]=r5` — an indexed
//! store into the table reached through the `+92` field.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c16eae8).unwrap_or(0);
    let addr = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(addr, cpu.r[5]); // delay slot of `rts`
}
