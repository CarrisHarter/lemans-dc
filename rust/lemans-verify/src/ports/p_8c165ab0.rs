//! 0x8c165ab0 — add_through_pointer.
//!
//! `r1=[r4] ; r0=[r5] ; r1+=r0 ; rts ; [r4]=r1` — accumulate `[r5]` into `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // delay slot of `rts`
}
