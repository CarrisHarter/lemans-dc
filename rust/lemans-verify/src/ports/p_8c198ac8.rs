//! 0x8c198ac8 — copy_four_words_at_64.
//!
//! Copies four words from `r5` to `r4+64`; the last store is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(64);
    cpu.r[0] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[5] + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[0]); // delay slot of `rts`
}
