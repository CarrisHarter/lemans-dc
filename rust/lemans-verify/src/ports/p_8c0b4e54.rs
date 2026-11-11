//! 0x8c0b4e54 — dot2.
//!
//! Two-element dot product: `fr0 = a0*b0 + a1*b1`, with both pointers advanced
//! by 4. `fr1`/`fr2`/`fr3` keep b1/a1/b0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[3] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3];
    cpu.fr[2] = cpu.fr[2] * cpu.fr[1];
    cpu.fr[0] = cpu.fr[0] + cpu.fr[2]; // delay slot of `rts`
}
