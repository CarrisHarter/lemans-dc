//! 0x8c0fdc8c — store_arg_to_global_3.
//!
//! `r0=[0x8c0fdc94] ; [r0]=r4 ; rts ; r0=r4` — stores the argument in a global
//! and returns it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0F_DC94).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
