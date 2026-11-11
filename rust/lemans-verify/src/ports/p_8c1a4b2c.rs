//! 0x8c1a4b2c — store_to_global.
//!
//! `r0=[0x8c1a4b34] ; rts ; [r0]=r4` — writes `r4` through the pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1a4b34).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
