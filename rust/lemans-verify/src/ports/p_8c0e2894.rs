//! 0x8c0e2894 — copy_three_floats_2.
//!
//! Copies the three floats at `r5`, `r5+4`, `r5+8` to `r4`, `r4+4`, `r4+8`.
//! `r0` ends as `r4`, `r1` as `r4+8`, `r3` as `r4+4`, `r5` as `src+8`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.fr[0] = mem.read_f32(src).unwrap_or(0.0);
    cpu.r[0] = src;
    cpu.r[3] = dst;
    let _ = mem.write_f32(dst, cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[1] = dst;
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    cpu.r[0] = dst;
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // delay slot of `rts`
}
