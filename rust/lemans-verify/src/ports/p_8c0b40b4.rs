//! 0x8c0b40b4 — init_quaternion.
//!
//! Zeroes the three floats at `r4+4`, `+8`, `+12`, sets `fr0` to 1.0 and stores
//! it at `[r4]`. `r0`/`r1`/`r3` end as `r4+4`/`r4+8`/`r4+12`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[3] = base.wrapping_add(12);
    cpu.fr[0] = 0.0;
    cpu.r[1] = base;
    cpu.r[0] = base;
    let _ = mem.write_f32(cpu.r[3], 0.0);
    cpu.r[1] = base.wrapping_add(8);
    let _ = mem.write_f32(cpu.r[1], 0.0);
    cpu.r[0] = base.wrapping_add(4);
    let _ = mem.write_f32(cpu.r[0], 0.0);
    cpu.fr[0] = 1.0;
    let _ = mem.write_f32(base, cpu.fr[0]); // delay slot of `rts`
}
