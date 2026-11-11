//! 0x8c0b2780 — abs_copy.
//!
//! `fmov fr4,fr0 ; rts ; fabs fr0` — the absolute value of `fr4` in `fr0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = cpu.fr[4];
    cpu.fr[0] = cpu.fr[0].abs(); // delay slot of `rts`
}
