//! 0x8c0b8340 — store_byte_at_4.
//!
//! `mov r5,r0 ; rts ; mov.b r0,@(4,r4)` — writes the low byte of `r5` at
//! `r4+4` and leaves `r0` as `r5`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5];
    let _ = mem.write_u8(cpu.r[4] + 4, cpu.r[0] as u8); // delay slot of `rts`
}
