//! 0x8c1a40b8 - run 0x8c19c4d0, then overwrite the vtable.
//!
//! The callee installs 0x8c1f14b8. This wrapper replaces that word with
//! 0x8c1f1788 and returns the object. The callee's other stores stay.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.pr = 0x8c1a40c4;
    crate::registry::p_8c19c4d0::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c1a40d4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
