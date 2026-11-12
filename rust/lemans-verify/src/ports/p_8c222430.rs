//! 0x8c222430 - 1 when a table entry's byte is nonzero and its slot has bit 1.
//!
//! `r14 = [0x8c23cc70 + 4*r4]`. 0x8c2199b8(r14) yields a byte pointer; 0x8c2199ca(r14)
//! yields the slot. A zero byte, or a slot word without bit 1, returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[0] = mem.read_u32(0x8c2224c0).unwrap_or(0x8c23cc70); // mov.l 0x8c2224c0,r0
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[14] <<= 2; // shll2 r14
    cpu.r[3] = mem.read_u32(0x8c2224c4).unwrap_or(0x8c2199b8); // mov.l 0x8c2224c4,r3
    cpu.r[14] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0); // mov.l @(r0,r14),r14
    cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
    cpu.pr = 0x8c222444;
    crate::registry::p_8c2199b8::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c2224c8).unwrap_or(0x8c2199ca); // mov.l 0x8c2224c8,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@r15
    cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
    cpu.pr = 0x8c22244c;
    crate::registry::p_8c2199ca::run(cpu, mem);
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[3] = mem.read_u8(cpu.r[2]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r2,r3
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    let mut hit = false;
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
        cpu.set_t((cpu.r[0] & 2) == 0); // tst #2,r0
        hit = !cpu.t();
    }
    cpu.r[0] = u32::from(hit); // mov #0/#1,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
