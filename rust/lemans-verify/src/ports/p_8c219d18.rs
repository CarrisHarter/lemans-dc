//! 0x8c219d18 - unpack a 4-byte record into five halfword fields.
//!
//! 0x8c219c9e(r5, 2) returns a record pointer; null returns -1. Otherwise the
//! caller's halfwords are filled as `[0] = b0+1`, `[2] = (b1+1)*32`,
//! `[4] = (b2>>4)&15`, `[6] = b2&15`, `[8] = (b3>>7)&1`, and the function
//! returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]); // mov.l r5,@r15
    cpu.r[5] = 2; // mov #2,r5
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bsr delay: mov.l @r15,r4
    cpu.pr = 0x8c219d28;
    crate::registry::p_8c219c9e::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut result;
    if !cpu.t() {
        cpu.r[5] = cpu.r[4].wrapping_add(2); // mov r4,r5 ; add #2,r5
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32; // mov.b @r4,r3
        cpu.r[3] &= 0xff; // extu.b r3,r3
        cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
        let _ = mem.write_u16(cpu.r[14], cpu.r[3] as u16); // mov.w r3,@r14
        cpu.r[3] = 0xffff_fff9; // mov #-7,r3
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(1)).unwrap_or(0) as u32; // mov.b @(1,r4),r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
        cpu.r[0] = cpu.r[0].wrapping_shl(1);
        let _ = mem.write_u16(cpu.r[14].wrapping_add(2), cpu.r[0] as u16); // mov.w r0,@(2,r14)
        cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; // mov.b @r5,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] &= 15; // and #15,r0
        let _ = mem.write_u16(cpu.r[14].wrapping_add(4), cpu.r[0] as u16); // mov.w r0,@(4,r14)
        cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; // mov.b @r5,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] &= 15; // and #15,r0
        let _ = mem.write_u16(cpu.r[14].wrapping_add(6), cpu.r[0] as u16); // mov.w r0,@(6,r14)
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(3)).unwrap_or(0) as u32; // mov.b @(3,r4),r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = (cpu.r[0] as i32 >> ((-(cpu.r[3] as i32)) & 31)) as u32; // shld r3,r0 (r3 = -7)
        cpu.r[0] &= 1; // and #1,r0
        let _ = mem.write_u16(cpu.r[14].wrapping_add(8), cpu.r[0] as u16); // mov.w r0,@(8,r14)
        result = 0; // mov #0,r0
    } else {
        result = 0xffff_ffff; // mov #-1,r0
    }
    cpu.r[0] = result;
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
