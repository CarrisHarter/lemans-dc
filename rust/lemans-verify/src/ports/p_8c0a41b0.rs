//! 0x8c0a41b0 — set_flag_byte_one.
//!
//! `mov #1,r0 ; rts ; mov.b r0,@(4,r4)` — writes 1 to the byte at `r4+4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 1;
    let _ = mem.write_u8(cpu.r[4] + 4, 1); // delay slot of `rts`
}
