//! 0x8c1bcc04 — sum_two_fields.
//!
//! `r0=r4+68 ; r4+=80 ; r0=[r0] ; r1=[r4] ; rts ; r0+=r1` — adds the words at
//! `+68` and `+80`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(68);
    cpu.r[4] = cpu.r[4].wrapping_add(80);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // delay slot of `rts`
}
