//! 0x8c12b4e0 — `return *(*(param_1 + 0x28) + param_2 * 4);`
//!
//! `mov.l @r4,r1 ; shll2 r5 ; mov r5,r0 ; add #32,r1 ; mov.l @(8,r1),r1 ; rts ; mov.l @(r0,r1),r0`
//! The C hides three things: `shll2 r5` clobbers r5 itself (T is untouched),
//! r1 is left holding the intermediate base pointer, and the `rts` delay slot
//! is the load that produces the return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r1 = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[5] <<= 2; // shll2 r5 — shifts r5 itself, leaves T alone
    let r0 = cpu.r[5];
    r1 = r1.wrapping_add(0x20);
    r1 = mem.read_u32(r1.wrapping_add(8)).unwrap();
    cpu.r[1] = r1;
    cpu.r[0] = mem.read_u32(r0.wrapping_add(r1)).unwrap(); // rts delay slot
}
