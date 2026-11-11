//! 0x8c0b4e68 — scale_two_floats.
//!
//! Multiplies the two floats at `r4` by `fr4` into a stack-passed destination;
//! the second store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = cpu.r[4].wrapping_add(4);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.fr[1] = cpu.fr[1] * cpu.fr[4];
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // delay slot of `rts`
}
