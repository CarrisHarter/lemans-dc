//! 0x8c0f87b0 - dispatch to 0x8c14fe18 or 0x8c14fe3c on the low byte of r6.
//!
//! Both targets take `r4+44` from their `jsr` delay slot with r5 unchanged,
//! and the callee's r0 is the result on both paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = cpu.r[6] & 0xff; // extu.b r6,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c0f87d0).unwrap_or(0x8c14fe18); // mov.l 0x8c0f87d0,r0
        cpu.r[4] = cpu.r[4].wrapping_add(44); // jsr delay: add #44,r4
        cpu.pr = 0x8c0f87be;
        crate::registry::p_8c14fe18::run(cpu, mem);
    } else {
        cpu.r[0] = mem.read_u32(0x8c0f87d4).unwrap_or(0x8c14fe3c); // mov.l 0x8c0f87d4,r0
        cpu.r[4] = cpu.r[4].wrapping_add(44); // jsr delay: add #44,r4
        cpu.pr = 0x8c0f87c8;
        crate::registry::p_8c14fe3c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
