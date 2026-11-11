//! 0x8c06cab4 — advance the argument by 4 and store `fr4` there.
//!
//! `add #4,r4 ; rts ; fmov fr4,@r4`
//! The store uses the advanced `r4`, and `r4` keeps the advanced value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of `rts`
}
