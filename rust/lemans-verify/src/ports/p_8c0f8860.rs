//! 0x8c0f8860 — copy_four_words_at_20.
//!
//! `r1=[r5] ; r4+=20 ; r0=[r5+4] ; [r4]=r1 ; [r4+4]=r0 ; r1=[r5+8] ; r0=[r5+12] ;
//! [r4+8]=r1 ; rts ; [r4+12]=r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    cpu.r[0] = mem.read_u32(src + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(src + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(src + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[0]); // delay slot of `rts`
}
