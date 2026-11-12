//! 0x8c1bbe3c — clear a flag byte on an object, then jmp to 0x8c0b5e94(0, 0x1388).
//!
//! `r0 = 24`, `r1 = 0`, `*(r4 + 24) = 0` (byte store), `r5 = 0x1388`, and the
//! `jmp` delay slot sets `r4 = 0`. `pr` is untouched, so the callee's `rts`
//! returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 24; // mov #24,r0
    cpu.r[1] = 0; // mov #0,r1
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // mov.b r1,@(r0,r4)
    cpu.r[5] = mem.read_u32(0x8c1bbe4c).unwrap_or(0x1388); // mov.l 0x8c1bbe4c,r5
    cpu.r[0] = mem.read_u32(0x8c1bbe50).unwrap_or(0x8c0b5e94); // mov.l 0x8c1bbe50,r0
    cpu.r[4] = cpu.r[1]; // delay slot: mov r1,r4
    crate::registry::p_8c0b5e94::run(cpu, mem); // jmp @r0
}
