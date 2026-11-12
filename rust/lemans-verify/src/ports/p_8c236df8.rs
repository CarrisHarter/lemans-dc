//! 0x8c236df8 — look `r4` up with 0x8c237780 and, on a hit, copy the object's
//! field +0x3c through `r5`.
//!
//! The prologue spills both arguments, and the `jsr` delay slot reloads `r4`
//! from the frame. A hit (callee returns 1) does `*r5 = *(u32 *)(r4 + 0x3c)`
//! and returns 0; a miss returns the sentinel 0x0000eeee and writes nothing.
//! The `tst` that selects between the two paths is also the last flag-setting
//! instruction, so T exits as 1 on the miss and 0 on the hit.

use sh4_oracle::{Cpu, Memory};

/// Entry of 0x8c237780, also the word in the literal pool at 0x8c236e24.
const CALLEE: u32 = 0x8c237780;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    cpu.r[3] = mem.read_u32(0x8c236e24).unwrap_or(CALLEE); // mov.l 0x8c236e24,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r15)
    cpu.pr = 0x8c236e06; // jsr @r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot: mov.l @r15,r4
    crate::registry::p_8c237780::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.r[0] = 0x0000_eeee; // mov.l 0x8c236e28,r0
    } else {
        // bf 0x8c236e40
        cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
        cpu.r[0] = 0; // mov #0,r0
        cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r3
        cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
        cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0); // mov.l @(36,r4),r2
        let _ = mem.write_u32(cpu.r[3], cpu.r[2]); // mov.l r2,@r3
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts; delay slot 0x8c236e52: nop
}
