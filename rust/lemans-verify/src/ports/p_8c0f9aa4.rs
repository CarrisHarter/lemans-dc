//! 0x8c0f9aa4 — `thunk_FUN_8c0f9ab2`.
//!
//! `bra 0x8c0f9ab2 ; nop`. A delayed branch, not a call: pr is untouched, so
//! the callee's `rts` returns straight to our caller, and the delay slot is a
//! nop. The target is 0x8c0f9ab2 — the strcmp loop's body entry, not the
//! 0x8c0f9aa8 fall-through block — so r3 and T are loaded/computed inside the
//! loop and never read from the entry state.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c0f9ab2::run(cpu, mem); // bra target
}
