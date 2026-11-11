//! 0x8c10ef38 — identity: `return param_1;`
//!
//! ```text
//! rts
//! mov r4,r0        ; delay slot: r0 = r4
//! ```
//! The whole function is the rts delay slot; r0 = r4, nothing else changes.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
