//! 0x8c01017c — `return;` as `rts ; nop`. No architectural effect at all.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
