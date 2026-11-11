//! 0x8c059b3c — `*out = *in` where `out` is fetched from the stack.
//!
//! Original: `mov.l @r15,r1 ; mov.l @r4,r0 ; rts ; mov.l r0,@r1` (delay slot).
//! Side effects the C does not show: r1 is left holding `[r15]` and r0 is
//! left holding `[r4]` — both survive the return, and the store goes through
//! the stack-loaded pointer in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // 0x8c059b3c mov.l @r15,r1
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // 0x8c059b3e mov.l @r4,r0
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // 0x8c059b42 rts delay: mov.l r0,@r1
}
