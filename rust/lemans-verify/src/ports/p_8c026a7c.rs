//! 0x8c026a7c — `thunk_FUN_8c0112f4`: tail-jump to `strlen` (0x8c0112f4).
//!
//! `mov.l 0x8c026a84,r0 ; jmp @r0 ; nop` — the literal pool word in the
//! shipped image is 0x8c0112f4. `jmp` leaves `pr` alone, so the callee's `rts`
//! returns straight to our caller, and the delay-slot `nop` changes nothing.
//! The callee overwrites `r0` with the length, so the loaded pointer is not
//! part of the register residue: after the jump the state is exactly strlen's
//! (`r0`/`r5` = count, `r4` past the NUL, `r3` = sign-extended terminator 0,
//! T set).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C026A84).unwrap_or(0); // mov.l @(0x8c026a84,pc),r0
    crate::registry::p_8c0112f4::run(cpu, mem); // jmp @r0 ; nop
}
