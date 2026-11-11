//! 0x8c010084 — release and clear a block pointer.
//!
//! Saves `r14`/`pr`, calls `free([r4])` (0x8c159fc0), then zeroes `[r4]`.
//! This is the first port that crosses a call: the port calls the ported
//! callee, and the oracle runs the callee's real bytes, so the whole tree must
//! agree.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c01009c).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[14]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c159fc0::run(cpu, mem);
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
