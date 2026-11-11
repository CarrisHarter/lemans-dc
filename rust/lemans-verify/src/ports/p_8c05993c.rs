//! 0x8c05993c — `*(char *)(*(int *)(param_1 + 4) + 5) = 1;`
//!
//! `mov.l @(4,r4),r1 ; mov #1,r0 ; rts ; mov.b r0,@(5,r1)` — same shape as
//! 0x8c059934 with the immediate 1: r1 keeps the inner pointer, r0 = 1, and
//! the byte store rides in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // mov.l @(4,r4),r1
    cpu.r[0] = 1; // mov #1,r0
    let _ = mem.write_u8(cpu.r[1].wrapping_add(5), cpu.r[0] as u8); // delay slot of `rts`
}
