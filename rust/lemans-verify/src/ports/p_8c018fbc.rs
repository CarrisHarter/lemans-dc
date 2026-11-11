//! 0x8c018fbc — `rts ; nop`
//!
//! The delay slot is a `nop`; the empty body is the exact port.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
