//! 0x8c03ed84 — `return DAT_8c03ed8c + param_1;`
//!
//! `mov.l 0x8c03ed8c,r0 ; rts ; add r4,r0`
//! The C shows the sum, but the addition is the delay slot of `rts`: r0
//! is first loaded with the literal-pool constant (0x3c4) and only then
//! gets param_1 added.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C03ED8C).unwrap(); // mov.l 0x8c03ed8c,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`: add r4,r0
}
