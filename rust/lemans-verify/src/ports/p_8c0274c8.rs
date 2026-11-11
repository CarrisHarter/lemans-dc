//! 0x8c0274c8 — an empty function: `rts ; nop`.
//! No registers, no T change, no stores.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
