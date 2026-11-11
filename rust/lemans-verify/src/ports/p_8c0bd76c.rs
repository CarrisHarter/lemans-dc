//! 0x8c0bd76c — zero_twelve_words.
//!
//! Zeroes twelve words at `r4` (`0..44`) and returns `r4`; the last store is
//! the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = 0;
    for off in (0u32..44).step_by(4) {
        let _ = mem.write_u32(base + off, 0);
    }
    cpu.r[0] = base;
    let _ = mem.write_u32(base + 44, 0); // delay slot of `rts`
}
