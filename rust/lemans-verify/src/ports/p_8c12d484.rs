//! 0x8c12d484 — set two flag bytes, return `PTR_s_Rookie_GT_8c12d4a8 + 0x6a`.
//!
//! `mov.l @r4,r1 ; mov #32,r0 ; mov #1,r3 ; mov.l @(4,r1),r4 ; mov.l 0x8c12d4a0,r1 ;
//!  mov.l @r4,r4 ; mov.b r3,@(r0,r4) ; mov.l @r1,r1 ; mov.l 0x8c12d4a4,r0 ;
//!  mov.b r3,@(r0,r1) ; mov.l 0x8c12d4a8,r0 ; rts ; add #106,r0`
//! The C shows the two byte stores but not the register clobbers (r0, r1, r3,
//! r4 all change) nor that the `+ 0x6a` runs in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r1 = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[3] = 1;
    cpu.r[4] = mem.read_u32(r1.wrapping_add(4)).unwrap();
    r1 = mem.read_u32(0x8C12D4A0).unwrap(); // 0x8c26c378
    cpu.r[4] = mem.read_u32(cpu.r[4]).unwrap();
    mem.write_u8(cpu.r[4].wrapping_add(0x20), cpu.r[3] as u8).unwrap(); // mov.b r3,@(r0,r4)
    r1 = mem.read_u32(r1).unwrap();
    let base = mem.read_u32(0x8C12D4A4).unwrap(); // 0x83
    mem.write_u8(base.wrapping_add(r1), cpu.r[3] as u8).unwrap(); // mov.b r3,@(r0,r1)
    cpu.r[1] = r1;
    cpu.r[0] = mem.read_u32(0x8C12D4A8).unwrap(); // 0x8c207af8
    cpu.r[0] = cpu.r[0].wrapping_add(0x6a); // rts delay slot: add #106,r0
}
