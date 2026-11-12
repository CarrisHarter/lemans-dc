//! 0x8c05e034 — `thunk_FUN_8c05e042`: `bra 0x8c05e042 ; nop`.
//!
//! Pure tail branch into the byte-compare loop body. `bra` leaves `pr` alone,
//! so the callee's `rts` returns straight to our caller, and the delay slot is
//! a `nop`. The thunk itself touches no register, flag or memory cell: every
//! residue (`r0`, `r1`, `r3`, `r4`, `r5`, T) and the value returned to the
//! caller come from 0x8c05e042, whose port is invoked here.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    crate::registry::p_8c05e042::run(cpu, mem); // bra 0x8c05e042 ; nop
}
