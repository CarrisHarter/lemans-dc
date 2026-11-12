//! 0x8c059a84 — `thunk_FUN_8c059a92`.
//!
//! ```text
//! 0x8c059a84: bra 0x8c059a92
//! 0x8c059a86: nop          ; delay slot
//! ```
//!
//! An unconditional tail jump into the byte string compare at 0x8c059a92.
//! The delay slot does nothing, so every register, memory byte and flag at
//! the callee entry is exactly the state at this entry: the whole routine is
//! the callee. `pr` is untouched (`bra` does not link).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // `nop` delay slot runs first and changes nothing; the branch target's
    // port then produces the entire residue.
    crate::registry::p_8c059a92::run(cpu, mem);
}
