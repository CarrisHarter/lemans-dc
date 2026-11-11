//! 0x8c05f294 — `return r4 + 4`.
//!
//! Original: `add #4,r4 ; rts ; mov r4,r0`. The C shows `param_1 + 4` but
//! the register side effects are r4 itself advanced and r0 = the new r4
//! (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
