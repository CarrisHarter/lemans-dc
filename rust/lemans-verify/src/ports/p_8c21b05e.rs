//! 0x8c21b05e - store 1 at byte 13 of slot `r4`.
//!
//! 0x8c21afd0(r4) returns the slot. The store is the `rts` delay slot, so r0
//! is already 1 when it runs.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.pr = 0x8c21b064; // bsr delay is nop
    crate::registry::p_8c21afd0::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[0] = 1; // mov #1,r0
    let _ = mem.write_u8(cpu.r[4].wrapping_add(13), cpu.r[0] as u8); // rts delay: mov.b r0,@(13,r4)
}
