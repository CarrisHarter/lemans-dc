//! 0x8c04347c — `*param_2 = *(*param_1);` (double indirection).
//!
//! `mov.l @r15,r1 ; mov.l @r4,r0 ; mov.l @r0,r0 ; rts ; mov.l r0,@r1`
//! Side effects the C does not show: r15 is not incremented, r1 is left
//! holding the stack-loaded destination pointer, and the store happens in
//! the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r1 (no post-increment)
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // mov.l @r0,r0
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `rts`: mov.l r0,@r1
}
