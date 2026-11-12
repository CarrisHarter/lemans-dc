//! 0x8c1bb480 — initialise the object's two 16-byte nodes and publish it.
//!
//! `r4` is the object. 0x8c1bb6a8 initialises `[r4]` with the byte the frame
//! holds at `sp+8`, and 0x8c1bb8d0 initialises `[r4+16]` with the byte at
//! `sp+6`; the compiler shuffles those two (uninitialised) frame bytes down to
//! `sp+5`/`sp+4` before each `jsr`, which is why this routine reads the stack
//! window at all. The finished pointer is then stored to the global at
//! 0x8c2b82b0. Writes to the stack window are not observed; the two node
//! initialisations and the global store are.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[5] = cpu.r[15];
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r0,r0
    cpu.r[6] = cpu.r[15];
    cpu.r[1] = mem.read_u32(0x8c1bb4c4).unwrap_or(0); // literal 0x8c1bb6a8
    cpu.r[5] = cpu.r[5].wrapping_add(5);
    cpu.r[6] = cpu.r[6].wrapping_add(9);
    cpu.pr = 0x8c1bb49c; // return address after the first `jsr`
    let _ = mem.write_u8(cpu.r[5], cpu.r[0] as u8); // delay slot of `jsr @r1`
    crate::registry::p_8c1bb6a8::run(cpu, mem);

    cpu.r[0] = cpu.r[15].wrapping_add(6);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[5] = cpu.r[15];
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r0,r0
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.r[6] = cpu.r[15];
    let _ = mem.write_u8(cpu.r[5], cpu.r[0] as u8);
    cpu.r[4] = cpu.r[4].wrapping_add(16);
    cpu.r[0] = mem.read_u32(0x8c1bb4c8).unwrap_or(0); // literal 0x8c1bb8d0
    cpu.pr = 0x8c1bb4b4; // return address after the second `jsr`
    cpu.r[6] = cpu.r[6].wrapping_add(7); // delay slot of `jsr @r0`
    crate::registry::p_8c1bb8d0::run(cpu, mem);

    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(0x8c1bb4cc).unwrap_or(0); // literal 0x8c2b82b0
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
