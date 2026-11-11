//! 0x8c059b30 — unlink-style pointer shuffle: `p2 = *p1; *p1 = **p1;` plus
//! a delay-slot store of the old `*p1` through `*sp`.
//!
//! `mov.l @r4,r1 ; mov.l @r1,r0 ; mov.l r0,@r4 ; mov.l @r15,r0 ; rts ; mov.l r1,@r0`
//! The C hides: r1 keeps the original `*r4`, r0 is first `*(*r4)` then
//! overwritten with `*sp`, and the `rts` delay slot stores r1 (the original
//! `*r4`) to `*(sp)` — a real store outside the stack window when `*sp`
//! points elsewhere.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r1
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap(); // mov.l @r1,r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // mov.l r0,@r4
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[1]); // delay slot of `rts`: mov.l r1,@r0
}
