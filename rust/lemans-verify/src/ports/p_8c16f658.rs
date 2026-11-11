//! 0x8c16f658 — lerp_ratio.
//!
//! Linear interpolation of `r4` and `r5` by `fr4` between `fr5` and `fr6`:
//! `(fr4-fr5)/(fr6-fr5) * (r5-r4) + r4`, truncated to an integer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[6] = cpu.fr[6] - cpu.fr[5];
    cpu.fr[1] = 1.0;
    cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[4]);
    cpu.fr[1] = cpu.fr[1] / cpu.fr[6];
    cpu.fr[4] = cpu.fr[4] - cpu.fr[5];
    cpu.fr[1] = cpu.fr[1] * cpu.fr[4];
    cpu.fpul = cpu.r[5];
    cpu.fr[0] = cpu.fpul as i32 as f32;
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    cpu.fpul = cpu.r[4];
    cpu.fr[0] = cpu.fpul as i32 as f32;
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    cpu.fpul = cpu.fr[1] as i32 as u32;
    cpu.r[0] = cpu.fpul; // delay slot of `rts`
}
