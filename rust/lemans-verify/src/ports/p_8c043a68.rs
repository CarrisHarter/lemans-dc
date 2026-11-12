//! 0x8c043a68 — release the block referenced by a handle.
//!
//! Saves `pr`, reloads `r4 = [r4+8]`, and returns without calling anything when
//! that word is zero. Otherwise loads the free thunk (0x8c01032c) into `r0`
//! and calls it with `r4` as the block pointer. Restores `pr` and returns.
//! The `tst` remains visible on the skip path: T is left set there, while the
//! call path ends with whatever the callee left in T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r4
    let skip = cpu.r[4] == 0;
    cpu.set_t(skip); // tst r4,r4 ; bt 0x8c043a76
    if !skip {
        cpu.r[0] = mem.read_u32(0x8c043a7c).unwrap_or(0x8c01032c); // mov.l @0x8c043a7c,r0
        cpu.pr = 0x8c043a76; // jsr @r0 sets pr to the instruction after the delay slot
        // delay slot at 0x8c043a74 is a nop
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts at 0x8c043a78; its delay slot at 0x8c043a7a is a nop
}
