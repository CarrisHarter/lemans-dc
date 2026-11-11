//! 0x8c17eb38 — reset_flag_0.
//!
//! Zeroes `byte[r4+61]`, sets `[r4+56]=0`, advances `r4` by 61, then
//! returns `movt` of `exts.b(byte[r4])` — i.e. 1 when the byte is zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 61;
    cpu.r[1] = 0;
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4] + 56, cpu.r[0]);
    cpu.r[4] = cpu.r[4].wrapping_add(61);
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
    cpu.set_t(cpu.r[0] == 0);
    cpu.r[0] = cpu.t() as u32; // delay slot of `rts`
}
