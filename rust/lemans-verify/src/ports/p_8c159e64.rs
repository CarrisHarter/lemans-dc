//! 0x8c159e64 — store_byte_to_global.
//!
//! `r0=[0x8c159e6c] ; rts ; [r0]=r4` — a byte store through the pool pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c159e6c).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[0], cpu.r[4] as u8); // delay slot of `rts`
}
