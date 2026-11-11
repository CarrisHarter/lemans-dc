//! 0x8c059b44 — `*out = **in` (double indirection) with `out` from the stack.
//!
//! Original: `mov.l @r15,r1 ; mov.l @r4,r0 ; mov.l @r0,r0 ; rts ; mov.l r0,@r1`.
//! Side effects the C does not show: r1 = `[r15]` stays live, r0 ends as the
//! twice-dereferenced value, and the store happens in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // 0x8c059b44 mov.l @r15,r1
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // 0x8c059b46 mov.l @r4,r0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // 0x8c059b48 mov.l @r0,r0
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // 0x8c059b4c rts delay: mov.l r0,@r1
}
