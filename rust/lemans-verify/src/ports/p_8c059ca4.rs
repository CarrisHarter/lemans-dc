//! 0x8c059ca4 — empty function: `rts ; nop`.
//!
//! No register, flag, FPU or memory effect at all; the delay slot is a nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
