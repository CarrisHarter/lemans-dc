//! 0x8c0fdae8 — copy_four_words_at_48.
//!
//! `r1=[r6] ; r5+=48 ; r0=[r6+4] ; [r5]=r1 ; [r5+4]=r0 ; r1=[r6+8] ; r0=[r6+12] ;
//! [r5+8]=r1 ; rts ; [r5+12]=r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[6];
    cpu.r[1] = mem.read_u32(src).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(48);
    cpu.r[0] = mem.read_u32(src + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(src + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(src + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[5] + 12, cpu.r[0]); // delay slot of `rts`
}
