//! 0x8c21d0f2 - copy the first two bytes of a 0x100-stride record into `[r5]`.
//!
//! 0x8c2199ca(r4) yields the slot. 0x8c219ed6(stack+8, slot) splits the record's
//! first two bytes into words at `sp+8` and `sp+12`, or returns -1. A negative
//! result returns -2. Otherwise the low bytes of those words are stored at
//! `[r5]` and `[r5+1]` and the function returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(16); // add #-16,r15
    cpu.r[3] = mem.read_u32(0x8c21d138).unwrap_or(0x8c2199ca); // mov.l 0x8c21d138,r3
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[4]); // mov.l r4,@(4,r15)
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]); // mov.l r5,@r15
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // jsr delay
    cpu.pr = 0x8c21d100;
    crate::registry::p_8c2199ca::run(cpu, mem);
    cpu.r[2] = mem.read_u32(0x8c21d13c).unwrap_or(0x8c219ed6); // mov.l 0x8c21d13c,r2
    cpu.r[4] = cpu.r[15]; // mov r15,r4
    cpu.r[5] = cpu.r[0]; // mov r0,r5
    cpu.r[4] = cpu.r[4].wrapping_add(8); // jsr delay: add #8,r4
    cpu.pr = 0x8c21d10a;
    crate::registry::p_8c219ed6::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
    if !cpu.t() {
        cpu.r[15] = cpu.r[15].wrapping_add(16); // add #16,r15
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 0xffff_fffe; // rts delay: mov #-2,r0
        return;
    }
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r15),r3
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
    let _ = mem.write_u8(cpu.r[2], cpu.r[3] as u8); // mov.b r3,@r2
    cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(12)).unwrap_or(0) as i8 as i32 as u32; // mov.b @(12,r15),r0
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
    let _ = mem.write_u8(cpu.r[2].wrapping_add(1), cpu.r[0] as u8); // mov.b r0,@(1,r2)
    cpu.r[0] = 0; // mov #0,r0
    cpu.r[15] = cpu.r[15].wrapping_add(16); // add #16,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
