//! 0x8c0d1360 — offset_table_entry.
//!
//! `r5<<=2 ; r5<<=2 ; r0=0xa4 ; r4+=r5 ; rts ; r0+=r4` — `r5` is scaled by 16
//! and `r0` ends as `0xa4 + r4 + r5*16`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[0] = mem.read_u32(0x8C0D_136C).unwrap_or(0xa4);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
