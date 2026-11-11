//! 0x8c21a04a — set_indexed_word_if_in_range.
//!
//! Stores `r5` at `0x8c2b8878 + 4*r4` only when `r4 <= 31`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 31;
    let t = cpu.r[4] > cpu.r[3];
    cpu.set_t(t);
    if !t {
        cpu.r[0] = mem.read_u32(0x8c21a0c0).unwrap_or(0);
        cpu.r[4] = cpu.r[4] << 2;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[5]);
    }
}
