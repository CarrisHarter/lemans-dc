//! 0x8c16d338 — store_field8_through_stack_pointer.
//!
//! `r1=[r15] ; r0=[r4+8] ; rts ; [r1]=r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `rts`
}
