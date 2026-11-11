//! 0x8c010134 — `rts ; mov #0,r0`
//!
//! The C shows `return 0`, and the return value lands in r0 — but only in the
//! delay slot of `rts`, after the branch has already been taken.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0; // delay slot of `rts`
}
