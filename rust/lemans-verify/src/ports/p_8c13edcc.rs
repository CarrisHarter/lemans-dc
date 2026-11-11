//! 0x8c13edcc — store_arg_return_self_8.
//!
//! `[r4]=r5 ; rts ; r0=r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let _ = mem.write_u32(base, cpu.r[5]);
    cpu.r[0] = base; // delay slot of `rts`
}
