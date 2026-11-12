//! 0x8c10b6fc — index a word table at r4+0x84 and tail-call 0x8c10ef64.
//!
//! `r4 = [r4 + 0x84 + (r6 << 2)]`, then `jmp` to the literal at 0x8c10b710.
//! `jmp` leaves pr alone, so the callee returns to our caller. `shll2` does
//! not set T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c10b70c).unwrap_or(0x84);
    cpu.r[6] = cpu.r[6].wrapping_shl(2);
    cpu.r[0] = cpu.r[6];
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c10b710).unwrap_or(0x8c10ef64);
    crate::registry::p_8c10ef64::run(cpu, mem);
}
