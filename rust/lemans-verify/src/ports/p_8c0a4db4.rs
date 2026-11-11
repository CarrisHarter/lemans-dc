//! 0x8c0a4db4 — store_byte_at_20.
//!
//! `mov #20,r0 ; rts ; mov.b r5,@(r0,r4)` — writes the low byte of `r5` at
//! `r4+20`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 20;
    let _ = mem.write_u8(cpu.r[4] + 20, cpu.r[5] as u8); // delay slot of `rts`
}
