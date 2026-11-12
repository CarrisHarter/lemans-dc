//! 0x8c026e84 - set a byte flag, then tail into 0x8c03ed5c through a field.
//!
//! `[r4+0x15c] = 1`, `r4 += 116`, then `jmp 0x8c03ed5c` with `r4 = [r4]` loaded in
//! the delay slot. pr is untouched, so the callee returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c026e94).unwrap_or(0x15c); // mov.l 0x8c026e94,r0
    cpu.r[1] = 1; // mov #1,r1
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // mov.b r1,@(r0,r4)
    cpu.r[4] = cpu.r[4].wrapping_add(116); // add #116,r4
    cpu.r[0] = mem.read_u32(0x8c026e98).unwrap_or(0x8c03ed5c); // mov.l 0x8c026e98,r0
    cpu.r[4] = mem.read_u32(cpu.r[4]).unwrap_or(0); // jmp delay: mov.l @r4,r4
    crate::registry::p_8c03ed5c::run(cpu, mem);
}
