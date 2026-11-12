//! 0x8c21b04e - zero-extended byte 12 of slot `r4`.
//!
//! 0x8c21afd0(r4) returns the slot. `mov.b` sign-extends, then `extu.b` clears
//! the high bits. That `extu.b` is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.pr = 0x8c21b054; // bsr delay is nop
    crate::registry::p_8c21afd0::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(12)).unwrap_or(0) as i8 as i32 as u32; // mov.b
    cpu.r[0] &= 0xff; // rts delay: extu.b r0,r0
}
