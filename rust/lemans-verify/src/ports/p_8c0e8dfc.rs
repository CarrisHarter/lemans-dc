//! 0x8c0e8dfc — swap_float_global.
//!
//! `r0=[0x8c0e8e04] ; fr0=[r0] ; rts ; [r0]=fr4` — reads the old float into
//! `fr0` and writes `fr4` over it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0E_8E04).unwrap_or(0);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]); // delay slot of `rts`
}
