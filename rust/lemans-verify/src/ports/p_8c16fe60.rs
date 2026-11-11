//! 0x8c16fe60 — scale_and_truncate.
//!
//! Stores `fr4` at `[r4+8]`, then scales it by the constant at `0x8c16fe7c`
//! and by `[r4+28]`, truncates, and stores the integer at `[r4+16]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_f32(cpu.r[4] + 8, cpu.fr[4]);
    cpu.r[1] = cpu.r[4];
    cpu.r[0] = 0x8c16fe7c;
    cpu.r[1] = cpu.r[1].wrapping_add(28);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[4] = cpu.fr[1] * cpu.fr[4];
    cpu.fr[4] = cpu.fr[0] * cpu.fr[4];
    cpu.fpul = cpu.fr[4] as i32 as u32;
    cpu.r[0] = cpu.fpul;
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[0]); // delay slot of `rts`
}
