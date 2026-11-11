//! 0x8c18629c — copy_three_words_and_clear_byte.
//!
//! Copies three words from `r5` to `r4`, then clears `byte[r4+0x80]` in the
//! `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.r[0] = mem.read_u32(0x8c1862b0).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[6]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[3]);
    cpu.r[3] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u8); // delay slot of `rts`
}
