//! 0x8c10eadc — thunk into the strcmp body at 0x8c10eaea.
//!
//! `bra 0x8c10eaea ; nop`. Delayed branch, not a call: pr is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c10eaea::run(cpu, mem);
}
