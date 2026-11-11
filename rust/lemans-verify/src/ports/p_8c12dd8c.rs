//! 0x8c12dd8c — `return PTR_DAT_8c12dd94;`
//!
//! `mov.l 0x8c12dd94,r0 ; rts ; nop`
//! The value comes from the ROM literal pool, and the delay slot is a nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C12DD94).unwrap();
}
