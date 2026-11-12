//! 0x8c15217c - initialise a compound at r4: sub-init, quaternion, two resets.
//!
//! The argument lives in a 4-byte frame and is reloaded for every call:
//! 0x8c151524(arg), `[arg] = 0x8c1ef498` plus 0x8c0b40b4(arg+44),
//! 0x8c0b4ff0(arg+60), `[arg+72] = +0.0` plus 0x8c0b4ff0(arg+76). Returns arg.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15

    cpu.r[0] = mem.read_u32(0x8c1521b8).unwrap_or(0x8c151524); // mov.l 0x8c1521b8,r0
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay: mov.l @r15,r4
    cpu.pr = 0x8c152188;
    crate::registry::p_8c151524::run(cpu, mem);

    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[1] = mem.read_u32(0x8c1521bc).unwrap_or(0x8c1ef498); // mov.l 0x8c1521bc,r1
    cpu.r[0] = mem.read_u32(0x8c1521c0).unwrap_or(0x8c0b40b4); // mov.l 0x8c1521c0,r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[4] = cpu.r[4].wrapping_add(44); // jsr delay: add #44,r4
    cpu.pr = 0x8c152196;
    crate::registry::p_8c0b40b4::run(cpu, mem);

    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = mem.read_u32(0x8c1521c4).unwrap_or(0x8c0b4ff0); // mov.l 0x8c1521c4,r0
    cpu.r[4] = cpu.r[4].wrapping_add(60); // jsr delay: add #60,r4
    cpu.pr = 0x8c15219e;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[0] = mem.read_u32(0x8c1521c4).unwrap_or(0x8c0b4ff0); // mov.l 0x8c1521c4,r0
    cpu.r[1] = cpu.r[1].wrapping_add(72); // add #72,r1
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits()); // fmov fr0,@r1
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[4] = cpu.r[4].wrapping_add(76); // jsr delay: add #76,r4
    cpu.pr = 0x8c1521ae;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
