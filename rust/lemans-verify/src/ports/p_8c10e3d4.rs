//! 0x8c10e3d4 — `return *param_1 + 4;`
//!
//! ```text
//! mov.l @r4,r0
//! rts
//! add   #4,r0      ; delay slot
//! ```
//! The `add #4` is the `rts` delay slot; no T-bit change.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let v = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[0] = v.wrapping_add(4); // delay slot of `rts`
}
