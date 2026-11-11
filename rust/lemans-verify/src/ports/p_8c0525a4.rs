//! 0x8c0525a4 — `return *(u32*)(*(u32*)(r4 + 0x190) + 0x24)`.
//!
//! Side effects the decompiled C does not show: the offset 0x190 is a PC-
//! relative literal at 0x8c0525ac and the +0x24 load happens in the `rts`
//! delay slot, chained off the first load's result. Only r0 changes; T is
//! untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let off = mem.read_u32(0x8c0525ac).unwrap_or(0x190);
    let p = mem.read_u32(cpu.r[4].wrapping_add(off)).unwrap();
    cpu.r[0] = mem.read_u32(p.wrapping_add(0x24)).unwrap(); // rts delay slot
}
