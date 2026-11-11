//! 0x8c16d328 — store_advanced_through_stack_pointer.
//!
//! `r0=[r15] ; add #4,r4 ; rts ; [r0]=r4` — stores the advanced pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
