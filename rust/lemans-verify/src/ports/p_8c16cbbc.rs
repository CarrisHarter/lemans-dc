//! 0x8c16cbbc — store_r5_return_self.
//!
//! `[r4]=r5 ; rts ; r0=r4` — a store that returns the pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
