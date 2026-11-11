//! 0x8c02c41c — accumulate a field and scatter a 4-word record.
//!
//! Adds `f32[r4+0x3e0]` into `f32[r4+0x448]`, then calls 0x8c152138 on
//! `[r4+36]` (which advances the pointer by 44 and returns it in r0). Copies
//! the 4 words at the returned pointer onto the stack frame, advances the
//! argument from the stack by 12, and scatters the 4 stack words to
//! `[arg], [arg+4+8]`, `[arg+8+8]`, `[arg+12+12]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[14] = mem.read_u32(cpu.r[15] + 24).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c02c47c).unwrap_or(0);
    cpu.r[3] = cpu.r[4];
    cpu.r[1] = cpu.r[4];
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c02c480).unwrap_or(0);
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[0] = mem.read_u32(0x8c02c484).unwrap_or(0);
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[4] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c152138::run(cpu, mem);
    cpu.r[5] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[14];
    cpu.r[4] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0);
    cpu.r[6] = cpu.r[15];
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]);
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    cpu.r[4] = mem.read_u32(cpu.r[0] + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6] + 8, cpu.r[4]);
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[0]);
    cpu.r[0] = 0;
    cpu.fr[0] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);
    cpu.r[0] = 4;
    cpu.fr[0] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0);
    cpu.r[14] = cpu.r[14].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[0] = 8;
    cpu.fr[0] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = 12;
    cpu.fr[0] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
