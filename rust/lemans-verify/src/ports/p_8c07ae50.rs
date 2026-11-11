//! 0x8c07ae50 — copy_three_words.
//!
//! `r1=[r5] ; r0=r4 ; [r4]=r1 ; r1=[r5+4] ; [r4+4]=r1 ; r1=[r5+8] ; rts ; [r4+8]=r1`
//! The last store is the delay slot and r1 keeps the last word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src).unwrap_or(0);
    cpu.r[0] = dst;
    let _ = mem.write_u32(dst, cpu.r[1]);
    cpu.r[1] = mem.read_u32(src + 4).unwrap_or(0);
    let _ = mem.write_u32(dst + 4, cpu.r[1]);
    cpu.r[1] = mem.read_u32(src + 8).unwrap_or(0);
    let _ = mem.write_u32(dst + 8, cpu.r[1]); // delay slot of `rts`
}
