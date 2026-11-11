//! 0x8c10e3dc — `return *(undefined4 *)(*param_1 + 0x4c);`
//!
//! ```text
//! mov.l @r4,r0
//! add   #76,r0
//! rts
//! mov.l @r0,r0     ; delay slot: the actual second load happens here
//! ```
//! Hidden side effect: the final load is the `rts` delay slot, so r0 is
//! overwritten with *(u32*)(*r4 + 76) — the decompiled C shows the same
//! expression, but only the delay-slot timing explains r0's final value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = mem.read_u32(cpu.r[4]).unwrap();
    let addr = base.wrapping_add(76);
    cpu.r[0] = mem.read_u32(addr).unwrap(); // delay slot of `rts`
}
