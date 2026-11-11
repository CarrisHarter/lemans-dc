//! 0x8c100330 — store_float_indexed.
//!
//! `r0=[0x8c10033c] ; r4<<=2 ; r4+=r0 ; rts ; fmov fr4,@r4` — `r4` becomes the
//! table address and `fr4` is stored there.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C10_033C).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of `rts`
}
