//! 0x8c059adc — `return *DAT_8c059ae4;` i.e. dereference the global pointer.
//!
//! `mov.l 0x8c059ae4,r0 ; rts ; mov.l @r0,r0` — double indirection through
//! the literal pool (literal holds 0x8c23de00); the second load is the `rts`
//! delay slot, so r0 ends as the global's value, not the pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C059AE4).unwrap(); // mov.l 0x8c059ae4,r0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // delay slot of `rts`: mov.l @r0,r0
}
