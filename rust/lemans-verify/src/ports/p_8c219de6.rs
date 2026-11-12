//! 0x8c219de6 - unpack a 4-byte record into two words and two bytes.
//!
//! 0x8c219c9e(r5, 4) returns a record pointer; null returns -1. Otherwise
//! `[0] = b0+1` (word), `[2] = (b1+1)*32` (word), `[4] = (b2>>4)&15` (word),
//! byte `[6] = (b3>>6)&3`, byte `[7] = (b3>>5)&1`, and the function returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]); // mov.l r5,@r15
    cpu.r[5] = 4; // mov #4,r5
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bsr delay: mov.l @r15,r4
    cpu.pr = 0x8c219df6;
    crate::registry::p_8c219c9e::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut result;
    if !cpu.t() {
        cpu.r[5] = cpu.r[4].wrapping_add(3); // mov r4,r5 ; add #3,r5
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32; // mov.b @r4,r3
        cpu.r[3] &= 0xff; // extu.b r3,r3
        cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
        let _ = mem.write_u16(cpu.r[14], cpu.r[3] as u16); // mov.w r3,@r14
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(1)).unwrap_or(0) as u32; // mov.b @(1,r4),r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
        cpu.r[0] = cpu.r[0].wrapping_shl(1);
        let _ = mem.write_u16(cpu.r[14].wrapping_add(2), cpu.r[0] as u16); // mov.w r0,@(2,r14)
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(2)).unwrap_or(0) as u32; // mov.b @(2,r4),r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] &= 15; // and #15,r0
        let _ = mem.write_u16(cpu.r[14].wrapping_add(4), cpu.r[0] as u16); // mov.w r0,@(4,r14)
        cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; // mov.b @r5,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] &= 3; // and #3,r0
        let _ = mem.write_u8(cpu.r[14].wrapping_add(6), cpu.r[0] as u8); // mov.b r0,@(6,r14)
        cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; // mov.b @r5,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] = cpu.r[0].wrapping_shr(2); // shlr2 r0
        cpu.r[0] = cpu.r[0].wrapping_shr(1); // shlr r0
        cpu.r[0] &= 1; // and #1,r0
        let _ = mem.write_u8(cpu.r[14].wrapping_add(7), cpu.r[0] as u8); // mov.b r0,@(7,r14)
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
