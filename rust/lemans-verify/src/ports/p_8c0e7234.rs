//! 0x8c0e7234 — sr_imask.
//!
//! `stc sr,r0 ; shlr2 r0 ; shlr2 r0 ; rts ; and #15,r0` — the interrupt mask
//! field of SR.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.sr;
    cpu.r[0] = cpu.r[0] >> 2;
    cpu.r[0] = cpu.r[0] >> 2;
    cpu.r[0] &= 15; // delay slot of `rts`
}
