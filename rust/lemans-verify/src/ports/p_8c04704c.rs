//! 0x8c04704c — empty function.
//!
//! ```text
//! rts
//! nop
//! ```
//! Neither the instruction nor the delay slot touches any register, memory,
//! or the T bit.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
