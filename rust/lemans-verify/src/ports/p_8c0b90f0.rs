//! 0x8c0b90f0 — store_byte_at_0.
//!
//! `rts ; mov.b r5,@r4` — writes the low byte of `r5` at `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u8(cpu.r[4], cpu.r[5] as u8); // delay slot of `rts`
}
