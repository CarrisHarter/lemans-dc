//! 0x8c0f1ad8 — clear_byte_13.
//!
//! `mov #0,r0 ; rts ; mov.b r0,@(13,r4)` — clears the byte at `r4+13`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4] + 13, 0); // delay slot of `rts`
}
