//! 0x8c0599f0 — `return *(char *)(*(int *)(param_1 + 4) + 4);`
//!
//! `mov.l @(4,r4),r0 ; rts ; mov.b @(4,r0),r0` — r0 is first the inner
//! pointer, then the delay-slot byte load sign-extends over it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // mov.l @(4,r4),r0
    let byte = mem.read_u8(cpu.r[0].wrapping_add(4)).unwrap();
    cpu.r[0] = byte as i8 as i32 as u32; // delay slot of `rts`: mov.b @(4,r0),r0 sign-extends
}
