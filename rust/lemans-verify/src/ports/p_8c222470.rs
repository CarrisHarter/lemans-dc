//! 0x8c222470 - 1 when a table entry's byte is nonzero, slot byte +12 is 0,
//! and the 120-byte slot's first word has bit 1.
//!
//! Same table as 0x8c222430 (`[0x8c23cc70 + 4*r4]`). 0x8c2199b8 yields the byte
//! pointer, 0x8c2199ca the 120-byte slot, 0x8c219c92 the zero-extended byte at
//! slot+12 of the 44-byte table. Any failed test returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    cpu.r[0] = mem.read_u32(0x8c2224c0).unwrap_or(0x8c23cc70);
    cpu.r[14] = cpu.r[4];
    cpu.r[14] <<= 2; // shll2
    cpu.r[3] = mem.read_u32(0x8c2224c4).unwrap_or(0x8c2199b8);
    cpu.r[14] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
    cpu.r[4] = cpu.r[14]; // jsr delay
    cpu.pr = 0x8c222484;
    crate::registry::p_8c2199b8::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c2224c8).unwrap_or(0x8c2199ca);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[0]);
    cpu.r[4] = cpu.r[14]; // jsr delay
    cpu.pr = 0x8c22248c;
    crate::registry::p_8c2199ca::run(cpu, mem);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[2] = mem.read_u8(cpu.r[3]).unwrap_or(0) as i8 as i32 as u32;
    cpu.set_t(cpu.r[2] == 0); // tst r2,r2
    let mut hit = false;
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c2224cc).unwrap_or(0x8c219c92);
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c22249c;
        crate::registry::p_8c219c92::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
            cpu.set_t((cpu.r[0] & 2) == 0); // tst #2,r0
            hit = !cpu.t();
        }
    }
    cpu.r[0] = u32::from(hit);
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
