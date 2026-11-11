//! 0x8c07e8e4 — store_arg_return_self.
//!
//! `[r4]=r5 ; rts ; r0=r4` — the store is the body and the return is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let _ = mem.write_u32(base, cpu.r[5]);
    cpu.r[0] = base; // delay slot of `rts`
}
