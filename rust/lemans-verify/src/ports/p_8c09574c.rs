//! 0x8c09574c — follow_pointer_and_store_back.
//!
//! `r1=[r4] ; r0=[r1] ; [r4]=r0 ; r0=[r15] ; rts ; [r0]=r1` — replaces the
//! pointer at `[r4]` with its target and stores the old pointer at the
//! stack-passed destination.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[1]); // delay slot of `rts`
}
