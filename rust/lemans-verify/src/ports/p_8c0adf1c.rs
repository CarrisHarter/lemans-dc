//! 0x8c0adf1c — copy_three_words_at_28.
//!
//! `[r4+28]=[r5] ; [r4+32]=[r5+4] ; rts ; [r4+36]=[r5+8]` — `r4` is advanced by 28
//! first and the last store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    cpu.r[0] = mem.read_u32(src + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[0] = mem.read_u32(src + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // delay slot of `rts`
}
