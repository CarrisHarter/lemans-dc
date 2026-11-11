//! 0x8c08bf38 — `return *DAT_8c08bf40;` i.e. return *(0x8c23ddc0).
//!
//! `mov.l 0x8c08bf40,r0 ; rts ; mov.l @r0,r0`
//! The load of the global happens in the `rts` delay slot, so r0 is the
//! dereferenced value, not the pointer 0x8c23ddc0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c08bf40).unwrap_or(0); // mov.l literal
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // rts delay slot: mov.l @r0,r0
}
