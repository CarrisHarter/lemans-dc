//! 0x8c086b00 — store fr4/fr5/fr6 as three floats at r4, return r4.
//!
//! Side effects the C does not show: r1 ends as r4+4 (`mov r4,r1 ; add #4,r1`)
//! and r0 ends as r4, not r4+8 — the `rts` delay slot `mov r4,r0` overwrites
//! the `add #8,r0` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = base; // mov r4,r1
    let _ = mem.write_f32(base, cpu.fr[4]); // fmov fr4,@r4
    cpu.r[1] = base.wrapping_add(4); // add #4,r1
    cpu.r[0] = base; // mov r4,r0
    let _ = mem.write_f32(cpu.r[1], cpu.fr[5]); // fmov fr5,@r1
    cpu.r[0] = base.wrapping_add(8); // add #8,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[6]); // fmov fr6,@r0
    cpu.r[0] = base; // rts delay slot: mov r4,r0
}
