//! 0x8c0b8388 — load_byte_at_4.
//!
//! `rts ; mov.b @(4,r4),r0` — the sign-extended byte at `r4+4` is the
//! whole function.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4] + 4).unwrap_or(0) as i8 as i32 as u32; // delay slot
}
