//! 0x8c05ac38 — `return *(undefined4 *)(*(int *)(param_1 + 4) + 0x24);`
//!
//! ```text
//! mov.l @(4,r4),r0
//! rts
//! mov.l @(36,r0),r0   ; delay slot — the second load really happens
//! ```
//! Hidden side effect: r0 ends as the *double* dereference `[[r4+4]+0x24]`,
//! not the intermediate `[r4+4]`; no instruction touches T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let inner = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = mem.read_u32(inner.wrapping_add(0x24)).unwrap_or(0); // rts delay slot
}
