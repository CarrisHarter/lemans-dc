//! 0x8c01e6f8 — thunk to the `free` thunk at 0x8c01032c, taking the pointer in `r5`.
//!
//! ```text
//! mov.l 0x8c01e700,r0   ; r0 = 0x8c01032c
//! jmp   @r0             ; tail call; pr is NOT touched, pr belongs to the caller
//! mov   r5,r4           ; delay slot: free()'s argument comes from r5, not r4
//! ```
//!
//! The thunk contributes only `r0` (the loaded pointer) and `r4 = r5`. Every
//! other register, T and all stores belong to the callee, so the body delegates
//! to the ported 0x8c01032c thunk exactly as the original `jmp` does. `r0` is
//! unobservable from this level anyway: the callee's first instruction loads
//! its own literal into `r0` before any branch, and that value survives the
//! `free(NULL)` early exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c01e700).unwrap_or(0x8c01032c);
    cpu.r[4] = cpu.r[5]; // delay slot of `jmp`
    crate::registry::p_8c01032c::run(cpu, mem);
}
