//! 0x8c050c54 — `return *(undefined4 *)(param_1 + 0x20);`
//!
//! ```text
//! rts
//! mov.l @(32,r4),r0   ; delay slot — NOT an empty stub
//! ```
//! Hidden side effect: the 2-instruction body is a load in the rts delay slot;
//! r0 ends as `[r4+0x20]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(0x20)).unwrap(); // delay slot of `rts`
}
