//! 0x8c0adec0 — clamp_float_to_range.
//!
//! Clamps `fr4` into [1.0, 180.0] and stores it at `r4+44`. The bounds come from
//! the `mova` literals at 0x8c0adee0/e4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let lo = mem.read_f32(0x8C0A_DEE0).unwrap_or(1.0);
    let hi = mem.read_f32(0x8C0A_DEE4).unwrap_or(180.0);

    cpu.r[0] = 0x8C0A_DEE0; // mova
    cpu.fr[0] = lo;
    let below = cpu.fr[0] > cpu.fr[4]; // fcmp/gt fr4,fr0
    cpu.set_t(below);
    if !below {
        cpu.r[0] = 0x8C0A_DEE4; // mova
        cpu.fr[0] = hi;
        let above = cpu.fr[4] > cpu.fr[0]; // fcmp/gt fr0,fr4
        cpu.set_t(above);
        if !above {
            cpu.fr[0] = cpu.fr[4];
        }
    }

    cpu.r[4] = base.wrapping_add(44);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // delay slot of `rts`
}
