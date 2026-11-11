//! 0x8c0c8ec4 — store_arg_to_global_2.
//!
//! `r0=[0x8c0c8ecc] ; rts ; [r0]=r4` — the store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0C_8ECC).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
