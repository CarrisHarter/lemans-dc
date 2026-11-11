//! 0x8c043474 — `*param_2 = *param_1;` with param_2 popped off the stack.
//!
//! `mov.l @r15,r1 ; mov.l @r4,r0 ; rts ; mov.l r0,@r1`
//! Side effects the C does not show: r15 is NOT incremented (this is a raw
//! `@r15` load, not `@r15+`), r1 is left holding the destination pointer
//! read from the stack, and the store `*r1 = r0` happens in the `rts`
//! delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r1 (no post-increment)
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `rts`: mov.l r0,@r1
}
