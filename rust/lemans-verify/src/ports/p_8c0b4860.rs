//! 0x8c0b4860 — copy_four_words_at_4.
//!
//! Copies four words from `r5` to `r4+4..r4+16`, clears the byte at `[r4]` from
//! the delay slot, and leaves `r6 = r4+4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[3] = mem.read_u32(src).unwrap_or(0);
    cpu.r[6] = dst;
    cpu.r[1] = mem.read_u32(src + 4).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(4);
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[6], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[1]);
    cpu.r[3] = mem.read_u32(src + 8).unwrap_or(0);
    cpu.r[1] = mem.read_u32(src + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6] + 8, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[1]);
    let _ = mem.write_u8(dst, 0); // delay slot of `rts`
}
