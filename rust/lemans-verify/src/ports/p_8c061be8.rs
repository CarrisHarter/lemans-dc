//! 0x8c061be8 — `rts ; nop`: no architectural effect at all.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
