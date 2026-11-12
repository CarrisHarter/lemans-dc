//! 0x8c03ed5c - set a byte flag, then tail into 0x8c1959e4 on the field.
//!
//! `[r4+0x354] = 1`, then `jmp 0x8c1959e4` with `r4 = r4 + 0x354` (the add is the
//! `jmp` delay slot). pr is untouched, so the callee returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c03ed68).unwrap_or(0x354); // mov.l 0x8c03ed68,r0
    cpu.r[1] = 1; // mov #1,r1
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // mov.b r1,@(r0,r4)
    cpu.r[1] = mem.read_u32(0x8c03ed6c).unwrap_or(0x8c1959e4); // mov.l 0x8c03ed6c,r1
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // jmp delay: add r0,r4
    crate::registry::p_8c1959e4::run(cpu, mem);
}
