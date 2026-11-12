//! 0x8c19c228 - search with the caller's r6/r7, keep fr4 and r5.
//!
//! The search sees `r5 = entry r6`, `r6 = entry r7`, `fr4 = 0`. Afterwards
//! the saved entry `fr4` is stored at +28, entry `r5` at +48, and +24 is cleared.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[15] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[5] = cpu.r[6];
    cpu.r[6] = cpu.r[7];
    cpu.fr[4] = 0.0; // jsr delay
    cpu.pr = 0x8c19c240;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[1] = mem.read_u32(0x8c19c264).unwrap_or(0);
    cpu.r[0] = cpu.r[13].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[13].wrapping_add(48), cpu.r[14]);
    cpu.r[0] = 24;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), 0);
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
