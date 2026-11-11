//! 0x8c07e03c — copy_six_words.
//!
//! `r1=[r5] ; r0=r4 ; [r4]=r1 ; ... ; r1=[r5+20] ; rts ; [r4+20]=r1`
//! Six word copies; the last store is the delay slot and `r1` keeps the last word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src).unwrap_or(0);
    cpu.r[0] = dst;
    let _ = mem.write_u32(dst, cpu.r[1]);
    for off in [4u32, 8, 12, 16, 20] {
        cpu.r[1] = mem.read_u32(src + off).unwrap_or(0);
        let _ = mem.write_u32(dst + off, cpu.r[1]);
    }
}
