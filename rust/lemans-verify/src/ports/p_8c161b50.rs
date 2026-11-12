//! 0x8c161b50 — compare two words reached through trailing-link chains.
//!
//! Saves `r14`/`pr`, calls 0x8c1104c4 with `r5 = 0` to fetch
//! `*(*(r4 + 0x48))`, takes its `+4` word, then walks
//! `*(*(*(*(r4 + 0x58) + 8)) + 8)` and returns their equality in `r0`/`T`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c161b74).unwrap_or(0);
    cpu.r[5] = 0; // delay slot of `jsr`
    crate::registry::p_8c1104c4::run(cpu, mem);
    cpu.r[14] = cpu.r[14].wrapping_add(88);
    cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[3] == cpu.r[0]); // cmp/eq r0,r3
    cpu.r[0] = u32::from(cpu.t()); // movt r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
