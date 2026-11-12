//! 0x8c19be80 - search, then clear the object's header.
//!
//! Calls 0x8c19c9b8 with `fr4 = 0`, then overwrites the vtable with
//! 0x8c1f1464, clears the byte at +24 and the float at +28, and returns
//! the object.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.fr[4] = 0.0; // jsr delay
    cpu.pr = 0x8c19be8c;
    crate::registry::p_8c19c9b8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c19beac).unwrap_or(0);
    cpu.r[1] = cpu.r[14].wrapping_add(28);
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    cpu.r[0] = 24;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), 0);
    cpu.fr[0] = 0.0;
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
