//! 0x8c0d8b74 — store_float_to_global.
//!
//! `r0=[0x8c0d8b7c] ; rts ; fmov fr4,@r0` — stores `fr4` at the pool address.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0D_8B7C).unwrap_or(0);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]); // delay slot of `rts`
}
