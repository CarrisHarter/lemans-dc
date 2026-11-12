//! 0x8c221660 - fill one 128-byte slot, or return -1.
//!
//! 0x8c221da8(r4) nonzero returns -1. Otherwise 0x8c221db8 stores 12, then
//! 0x8c221dc8 fills the record from r5/r6 and three stack words, 0x8c221e18
//! copies two strings, and 0x8c221e8e copies 8 bytes from the word at the
//! entry stack pointer. Success returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let entry_sp = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = mem.read_u32(0x8c221880).unwrap_or(0x8c221da8);
    cpu.r[14] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[15], cpu.r[6]);
    cpu.r[13] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[7]);
    cpu.r[4] = cpu.r[14]; // jsr delay
    cpu.pr = 0x8c221676;
    crate::registry::p_8c221da8::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[0] = (-1i32) as u32; // bra delay
    } else {
        cpu.r[3] = mem.read_u32(0x8c221884).unwrap_or(0x8c221db8);
        cpu.r[5] = 12;
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c221686;
        crate::registry::p_8c221db8::run(cpu, mem);
        cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(24)).unwrap_or(0);
        cpu.r[5] = cpu.r[14];
        cpu.r[6] = cpu.r[13];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[2]);
        cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(24)).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[3]);
        cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c221888).unwrap_or(0x8c221dc8);
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[2]);
        cpu.r[7] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c22169e;
        crate::registry::p_8c221dc8::run(cpu, mem);
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        cpu.r[2] = mem.read_u32(0x8c22188c).unwrap_or(0x8c221e18);
        cpu.r[5] = cpu.r[13];
        cpu.r[6] = cpu.r[13];
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c2216aa;
        crate::registry::p_8c221e18::run(cpu, mem);
        cpu.r[3] = mem.read_u32(0x8c221890).unwrap_or(0x8c221e8e);
        cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c2216b2;
        crate::registry::p_8c221e8e::run(cpu, mem);
        cpu.r[0] = 0;
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    let _ = entry_sp;
}
