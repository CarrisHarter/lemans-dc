//! 0x8c106874 — thunk into the strcmp body at 0x8c106882.
//!
//! `bra 0x8c106882 ; nop`. Delayed branch, not a call: pr is untouched and the
//! delay slot is a nop, so the body's `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c106882::run(cpu, mem);
}
