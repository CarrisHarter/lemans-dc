//! 0x8c0b5e84 — subtract_from_global.
//!
//! `r1=[0x8c0b5e90] ; r0=[r1] ; r0-=r4 ; rts ; [r1]=r0` — subtracts the
//! argument from a global and stores the result back.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8C0B_5E90).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[4]);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `rts`
}
