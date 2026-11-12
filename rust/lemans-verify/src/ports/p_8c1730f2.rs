//! 0x8c1730f2 - branch thunk into the countdown loop at 0x8c1730fe.
//!
//! `bra 0x8c1730fe ; nop`. Delayed branch, not a call: pr is untouched and
//! the target's `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c1730fe::run(cpu, mem); // bra target
}
