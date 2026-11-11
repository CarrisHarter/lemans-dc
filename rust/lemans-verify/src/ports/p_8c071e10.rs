//! 0x8c071e10 — store_global_8c2648f0.
//!
//! `mov.l 0x8c071e18,r0 ! 8c2648f0 ; rts ; mov.l r4,@r0` — the store is the delay slot and r0 keeps the address.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C07_1E18).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
