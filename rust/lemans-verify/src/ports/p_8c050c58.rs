//! 0x8c050c58 — `*(undefined4 *)(param_1 + 0x20) = param_2;`
//!
//! ```text
//! rts
//! mov.l r5,@(32,r4)   ; delay slot — NOT an empty stub
//! ```
//! Hidden side effect: the 2-instruction body is a store in the rts delay slot;
//! `[r4+0x20]` is written with r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let addr = cpu.r[4].wrapping_add(0x20);
    let _ = mem.write_u32(addr, cpu.r[5]); // delay slot of `rts`
}
