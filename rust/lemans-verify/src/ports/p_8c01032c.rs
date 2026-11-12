//! 0x8c01032c — thunk to the free() implementation at 0x8c159fc0.
//!
//! ```text
//! mov.l 0x8c010334,r0   ; r0 = 0x8c159fc0
//! jmp   @r0             ; tail call; pr is NOT touched, the delay slot is a nop
//! nop
//! ```
//!
//! The thunk itself changes only `pc` and `r0`. The observable r0 matters on
//! the callee's `r4 == 0` early exit, which never writes r0: the original leaves
//! the loaded function pointer there. Every other register, T and all stores are
//! the callee's, so the body is delegated to the ported callee.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c010334).unwrap_or(0x8c159fc0);
    crate::registry::p_8c159fc0::run(cpu, mem);
}
