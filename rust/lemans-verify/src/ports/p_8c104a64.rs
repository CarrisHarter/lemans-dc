//! 0x8c104a64 — copy four floats from `r5` to `r4`.
//!
//! `r5` is advanced by 12 before the fourth load, so it reads `r5+12`; `r0`
//! ends as `r4` from the `rts` delay slot and `r1` as `r4+4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    let dst = cpu.r[4];

    cpu.fr[0] = mem.read_f32(src).unwrap_or(0.0);
    cpu.r[1] = src;
    cpu.r[0] = src;
    let _ = mem.write_f32(dst, cpu.fr[0]);

    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(12);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.r[1] = dst.wrapping_add(4);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);

    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = dst.wrapping_add(8);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);

    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[0] = dst.wrapping_add(12);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);

    cpu.r[0] = dst; // delay slot of `rts`
}
