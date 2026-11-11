//! 0x8c082210 — copy_four_words.
//!
//! Four word copies from `r5` to `r4`; the last store is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src).unwrap_or(0);
    cpu.r[0] = mem.read_u32(src + 4).unwrap_or(0);
    let _ = mem.write_u32(dst, cpu.r[1]);
    let _ = mem.write_u32(dst + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(src + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(src + 12).unwrap_or(0);
    let _ = mem.write_u32(dst + 8, cpu.r[1]);
    let _ = mem.write_u32(dst + 12, cpu.r[0]); // delay slot of `rts`
}
