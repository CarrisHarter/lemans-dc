//! 0x8c010144 — `rts ; nop`
//!
//! The delay slot is a `nop`, so the whole routine has no observable effect
//! even though the decompiler types it as `void`.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
