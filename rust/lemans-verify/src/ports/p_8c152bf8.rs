//! 0x8c152bf8 — pool_plus_arg.
//!
//! `r0=[0x8c152c00] ; rts ; r0+=r4` — returns `0xcc + r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c152c00).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
