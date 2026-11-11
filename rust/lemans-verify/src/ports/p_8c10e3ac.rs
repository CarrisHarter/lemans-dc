//! 0x8c10e3ac — load_field_plus_24_2.
//!
//! `r0=[r4] ; rts ; r0+=24` — the pointer plus 24.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(24); // delay slot of `rts`
}
