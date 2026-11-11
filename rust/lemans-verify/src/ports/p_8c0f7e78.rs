//! 0x8c0f7e78 — store_advanced_arg_to_stack_pointer_2.
//!
//! `r0=[r15] ; r4+=4 ; rts ; [r0]=r4` — the destination is the caller's stack
//! slot and `r4` is advanced first.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`
}
