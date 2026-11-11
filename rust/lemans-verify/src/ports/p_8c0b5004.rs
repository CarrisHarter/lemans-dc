//! 0x8c0b5004 — scale four floats starting at `r4` by `fr4`.
//!
//! `r4` is advanced by 12 before the last load, so the fourth float is at
//! `r4+12`; the final store is the `rts` delay slot. `r1` ends as `r4+4`,
//! `r0` as `r4+8`, and `fr0` as the last product.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.fr[0] = mem.read_f32(base).unwrap_or(0.0);
    cpu.r[1] = base;
    cpu.r[0] = base;
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    let _ = mem.write_f32(base, cpu.fr[0]);

    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);

    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);

    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // delay slot of `rts`
}
