//! 0x8c01e70c — empty function.
//!
//! `rts ; nop` — no register, T, or memory effect at all.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
