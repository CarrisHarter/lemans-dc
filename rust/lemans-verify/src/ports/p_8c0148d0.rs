//! 0x8c0148d0 — call_and_scale.
//!
//! Calls 0x8c013e94, then returns `fr0 * [0x8c1cd288]` (the pool word is the
//! float address, not a pointer).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c0148e4).unwrap_or(0);
    crate::registry::p_8c013e94::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c0148e8).unwrap_or(0);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[3] = cpu.fr[0];
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3]; // delay slot of `rts`
}
