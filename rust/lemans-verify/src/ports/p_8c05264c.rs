//! 0x8c05264c — `rts ; nop`. The whole function is a bare return: no
//! register, flag or memory effect at all.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
