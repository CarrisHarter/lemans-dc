//! 0x8c151f60 — store_to_global.
//!
//! `r0=[0x8c151f68] ; rts ; [r0]=r4` — writes `r4` through the pool pointer
//! `0x8c2abd18`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c151f68).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
