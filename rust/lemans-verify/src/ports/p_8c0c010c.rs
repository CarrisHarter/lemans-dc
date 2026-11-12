//! 0x8c0c010c — thunk that advances a pointer through its second word.
//!
//! ```text
//! mov.l 0x8c0c0114,r0   ; r0 = 0x8c0f8874 (advance_8)
//! jmp   @r0             ; tail call; pr is NOT touched
//! mov.l @(4,r4),r4      ; delay slot: r4 = [r4+4]
//! ```
//!
//! The callee adds 8 to the loaded word and leaves it in `r4` and `r0`; every
//! other register, T and all stores are the callee's. The thunk itself changes
//! only `r0` (immediately overwritten), `r4` and `pc`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0c0114).unwrap_or(0x8c0f8874);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // delay slot of `jmp`
    crate::registry::p_8c0f8874::run(cpu, mem);
}
