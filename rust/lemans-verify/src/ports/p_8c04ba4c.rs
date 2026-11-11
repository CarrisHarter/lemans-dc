//! 0x8c04ba4c — `return *(undefined4 *)(param_2 * 4 + *(int *)(param_1 + 4));`
//!
//! ```text
//! mov.l @(4,r4),r1
//! shll2 r5
//! mov   r5,r0
//! rts
//! mov.l @(r0,r1),r0    ; delay slot
//! ```
//! Hidden side effects: r5 is shifted in place (C shows `param_2 * 4`), the
//! table base stays in r1, and the element load is in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
    cpu.r[5] <<= 2; // shll2 r5
    cpu.r[0] = cpu.r[5];
    let addr = cpu.r[0].wrapping_add(cpu.r[1]);
    cpu.r[0] = mem.read_u32(addr).unwrap(); // delay slot of `rts`
}
