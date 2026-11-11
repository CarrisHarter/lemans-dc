//! 0x8c21c40a — mask_28_bits.
//!
//! `r0=[0x8c21c438] ; rts ; r0 &= r4` — masks `r4` with the pool constant.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c21c438).unwrap_or(0);
    cpu.r[0] = cpu.r[0] & cpu.r[4]; // delay slot of `rts`
}
