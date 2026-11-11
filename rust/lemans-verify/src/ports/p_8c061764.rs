//! 0x8c061764 — bare `rts; nop`: no register, flag or memory effects.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
