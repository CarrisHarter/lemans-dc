//! 0x8c0b4db4 — negate_three_floats.
//!
//! Negates the three floats at `r4`, `r4+4`, `r4+8` into a stack-passed
//! destination; the third store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[5] = dst;
    cpu.r[3] = cpu.r[4].wrapping_add(8);
    cpu.r[1] = cpu.r[4].wrapping_add(4);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[5];
    cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[0] = -cpu.fr[0];
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.fr[2] = -cpu.fr[2];
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]);
    cpu.fr[1] = -cpu.fr[1];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[5], cpu.fr[2]); // delay slot of `rts`
}
