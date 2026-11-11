//! 0x8c159e70 — store_fr4_to_global.
//!
//! `r0=[0x8c159e78] ; rts ; [r0]=fr4` — a float store through the pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c159e78).unwrap_or(0);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]); // delay slot of `rts`
}
