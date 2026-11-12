//! 0x8c216bee - fold three bytes into one value through 0x8c216bd4.
//!
//! Reads `[r14]`, `[r14+1]` and `[r14+2]` (the first two with post-increment) and
//! converts each with 0x8c216bd4. The result is
//! `((b0*60 + b1) * 75) + b2` in wrapping 32-bit arithmetic; the two `mul.l`
//! steps write MACH:MACL and only MACL is read back.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14

    cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32; // bsr delay: mov.b @r14+,r4
    cpu.r[14] = cpu.r[14].wrapping_add(1);
    cpu.pr = 0x8c216bfa;
    crate::registry::p_8c216bd4::run(cpu, mem);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r15)

    cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32; // bsr delay: mov.b @r14+,r4
    cpu.r[14] = cpu.r[14].wrapping_add(1);
    cpu.pr = 0x8c216c00;
    crate::registry::p_8c216bd4::run(cpu, mem);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@r15

    cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32; // bsr delay: mov.b @r14,r4
    cpu.pr = 0x8c216c06;
    crate::registry::p_8c216bd4::run(cpu, mem);

    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r2
    cpu.r[3] = 60; // mov #60,r3
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    let product = (cpu.r[3] as i32 as i64) * (cpu.r[2] as i32 as i64); // mul.l r3,r2
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[3] = 75; // mov #75,r3
    cpu.r[0] = cpu.macl; // sts macl,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // add r1,r0
    let product = (cpu.r[3] as i32 as i64) * (cpu.r[0] as i32 as i64); // mul.l r3,r0
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[0] = cpu.macl; // sts macl,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // add r4,r0

    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
