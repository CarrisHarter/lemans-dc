//! 0x8c0b9edc — clear_byte_return_self.
//!
//! `r0=0 ; [r4]=r0 ; rts ; r0=r4` — clears the byte at `[r4]` and returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = 0;
    let _ = mem.write_u8(base, 0);
    cpu.r[0] = base; // delay slot of `rts`
}
