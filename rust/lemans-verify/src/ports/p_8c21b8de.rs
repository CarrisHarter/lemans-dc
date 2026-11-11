//! 0x8c21b8de — tagged pointer write plus counter bump on the global struct.
//!
//! r5 = 0x8c2ba634; r4 <<= 16 (shll16); r0 = 0x0fcc. r3v = *0x8c2ba634.
//! *(r3v+0xfcc) += 4 * *(r3v+0xfd8) (stored), then that new word is used as a
//! pointer: *(r3v+0xfcc) = (r4e<<16) | 0x300 (or with literal 0x0300), overwriting
//! the bump. Then *(r3v+0xfd8) = 1, and *(r3v+0xfdc) += 4.
//! rts delay slot zeroes r0. T is never touched (shll16/or/mov.w leave it).
//!
//! Final: r0=0, r1=*(r3v+0xfdc)+4, r2=r3v, r3=0x300, r4=(r4e<<16)|0x300,
//! r5=0x8c2ba634, r6=new *(r3v+0xfcc) (the store target).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4e = cpu.r[4];
    cpu.r[5] = 0x8c2ba634; // mov.l literal,r5
    let mut r4 = r4e << 16; // shll16 r4 (T untouched)
    let r0: u32 = 0x0fcc; // mov.w 0x0fcc,r0
    let r3v = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r3
    let w = mem.read_u32(r3v.wrapping_add(r0.wrapping_add(12))).unwrap_or(0); // @r1 = r3+0xfd8
    let mut r2 = mem.read_u32(r3v.wrapping_add(r0)).unwrap_or(0); // @(r0,r3)
    let w = w << 2; // shll2 r1
    r2 = r2.wrapping_add(w);
    mem.write_u32(r3v.wrapping_add(r0), r2).unwrap(); // mov.l r2,@(r0,r3)
    let r1: u32 = 1; // mov #1,r1
    let mut r6 = mem.read_u32(r3v.wrapping_add(r0)).unwrap_or(0); // mov.l @(r0,r6),r6 (r6 was r3v)
    let r0 = r0.wrapping_add(12); // add #12,r0 -> 0xfd8
    let r3: u32 = 0x0300; // mov.w 0x0300,r3
    r4 |= r3; // or r3,r4 (T untouched)
    mem.write_u32(r6, r4).unwrap(); // mov.l r4,@r6
    let r2 = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r2
    mem.write_u32(r2.wrapping_add(r0), r1).unwrap(); // mov.l r1,@(r0,r2)
    let r0 = r0.wrapping_add(4); // add #4,r0 -> 0xfdc
    let r2 = mem.read_u32(0x8c2ba634).unwrap_or(0); // mov.l @r5,r2
    let r1 = mem
        .read_u32(r2.wrapping_add(r0))
        .unwrap_or(0)
        .wrapping_add(4); // mov.l @(r0,r2),r1 ; add #4,r1
    mem.write_u32(r2.wrapping_add(r0), r1).unwrap(); // mov.l r1,@(r0,r2)
    cpu.r[0] = 0; // rts delay slot: mov #0,r0
    cpu.r[1] = r1;
    cpu.r[2] = r2;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[6] = r6;
}
