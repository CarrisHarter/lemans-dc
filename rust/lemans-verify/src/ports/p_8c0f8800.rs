//! 0x8c0f8800 - dispatch to 0x8c14fdf4 or 0x8c14fe3c on the low byte of r6.
//!
//! Twin of 0x8c0f87b0 with a different nonzero target. Both paths hand `r4+44`
//! to the callee; r5 is untouched either way.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = cpu.r[6] & 0xff; // extu.b r6,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c0f8820).unwrap_or(0x8c14fdf4); // mov.l 0x8c0f8820,r0
        cpu.r[4] = cpu.r[4].wrapping_add(44); // jsr delay: add #44,r4
        cpu.pr = 0x8c0f880e;
        crate::registry::p_8c14fdf4::run(cpu, mem);
    } else {
        cpu.r[0] = mem.read_u32(0x8c0f8824).unwrap_or(0x8c14fe3c); // mov.l 0x8c0f8824,r0
        cpu.r[4] = cpu.r[4].wrapping_add(44); // jsr delay: add #44,r4
        cpu.pr = 0x8c0f8818;
        crate::registry::p_8c14fe3c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
