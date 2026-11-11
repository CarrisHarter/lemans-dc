//! 0x8c12d718 — `return PTR_DAT_8c12d720;`
//!
//! `mov.l 0x8c12d720,r0 ; rts ; nop`
//! The value comes from the ROM literal pool, and the delay slot is a nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C12D720).unwrap();
}
