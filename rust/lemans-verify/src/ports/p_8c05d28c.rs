//! 0x8c05d28c — empty function.
//!
//! ```text
//! rts
//! nop
//! ```
//! No architectural state changes at all: no register, no T bit, no store.
//! The decompiled `return;` is the whole story here.

use sh4_oracle::{Cpu, Memory};

pub fn run(_cpu: &mut Cpu, _mem: &mut Memory) {}
