//! 0x8c0fca08 — pick_smaller_pointer.
//!
//! `r1=[r4] ; r0=[r5] ; cmp/hi r1,r0 ; bf keep ; r5=r4 ; rts ; r0=r5` — chooses
//! `r4` when its word is not above `r5`'s, and returns the chosen pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    let above = cpu.r[0] > cpu.r[1];
    cpu.set_t(above); // cmp/hi r1,r0
    if !above {
        cpu.r[5] = cpu.r[4];
    }
    cpu.r[0] = cpu.r[5]; // delay slot of `rts`
}
