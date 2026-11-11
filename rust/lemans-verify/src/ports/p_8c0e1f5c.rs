//! 0x8c0e1f5c — store_byte_at_96.
//!
//! `mov #96,r0 ; rts ; mov.b r5,@(r0,r4)` — writes the low byte of `r5` at
//! `r4+96`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 96;
    let _ = mem.write_u8(cpu.r[4] + 96, cpu.r[5] as u8); // delay slot of `rts`
}
