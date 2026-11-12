//! 0x8c01016c — `thunk_FUN_8c15a1d8`.
//!
//! `mov.l 0x8c010174,r0 ; jmp @r0 ; nop` — load the code pointer from the
//! literal pool and tail-call it; the delay slot does nothing. r0 is left
//! holding the loaded pointer (the callee does not return it anywhere else),
//! and 0x8c15a1d8 is an empty `rts ; nop` stub, so the pointer is the only
//! residue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C010174).unwrap();
    crate::registry::p_8c15a1d8::run(cpu, mem); // jmp @r0
}
