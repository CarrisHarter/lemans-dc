//! 0x8c010080 — `rts ; nop`
//!
//! The delay slot is a `nop`, so the whole routine has no observable effect.
//! The empty body is the correct port, not a placeholder.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
