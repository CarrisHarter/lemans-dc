//! 0x8c051cfc — `rts ; nop`. A pure stub: no register, T, or memory effect.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
