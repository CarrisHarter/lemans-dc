//! 0x8c0b4d4c — lerp_two_pairs.
//!
//! Blends two float pairs with `fr4` and `1-fr4` into a stack-passed
//! destination: `out0 = a0*fr4 + b0*(1-fr4)`, `out1 = a1*(1-fr4) + b1*fr4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = dst;
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[4];
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[5] = 1.0;
    cpu.fr[2] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.fr[5] = cpu.fr[5] - cpu.fr[4];
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    cpu.fr[1] = cpu.fr[1] * cpu.fr[5];
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.fr[3] = cpu.fr[3] * cpu.fr[5];
    cpu.fr[2] = cpu.fr[2] * cpu.fr[4];
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2];
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[3]); // delay slot of `rts`
}
