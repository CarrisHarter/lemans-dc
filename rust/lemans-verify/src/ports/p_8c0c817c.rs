//! 0x8c0c817c — rolling table hash over `r4[0..r5]`, seeded in r6, result in r0.
//!
//! The C shows the math but not the residue: the word loop leaves r3 = last
//! word loaded, r4 past the words, r7 = word count; the byte tail runs its
//! `bf.s` delay slot (`add #-1,r5`) one extra time, so r5 ends at 0xFFFFFFFF
//! (not 0) whenever a remainder was processed, and r1 ends as the shifted
//! table index, not the table entry. r14 is restored by the epilogue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let table = mem.read_u32(0x8c0c81e8).unwrap_or(0); // mov.l literal -> 0x8c27a2a8
    let r14_saved = cpu.r[14];
    let mut r4 = cpu.r[4];
    let mut r5 = cpu.r[5];
    let mut r6 = cpu.r[6];
    let r7 = r5 >> 2; // mov r5,r7 ; shlr2 r7
    let r0 = r7 << 2; // mov r7,r0 ; shll2 r0
    r5 = r5.wrapping_sub(r0); // bf.s delay slot: sub r0,r5 (always runs)
    let mut r1 = cpu.r[1];
    let mut r3 = cpu.r[3];
    if r7 > 0 {
        // cmp/hi r14,r7 (r14 = 0): word loop runs exactly r7 times
        for _ in 0..r7 {
            r1 = r6 >> 24; // mov r6,r1 ; shlr8 r1 ; shlr16 r1
            r3 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r3
            r1 = ((r1 ^ r3) & 0xff) << 2; // exts.b ; xor r3,r1 ; extu.b ; shll2 r1
            r1 = mem.read_u32(table.wrapping_add(r1)).unwrap_or(0); // mov.l @(r0,r1),r1
            r6 = (r6 << 8) ^ r1; // shll8 r6 ; bt.s delay slot xor r1,r6 (runs every iteration)
            r4 = r4.wrapping_add(4); // add #4,r4
        }
    }
    if r5 != 0 {
        // byte tail over the remainder
        r5 = r5.wrapping_sub(1); // add #-1,r5
        r1 = 0; // bt.s delay slot: mov #0,r1 (always runs)
        loop {
            let b = mem.read_u8(r4).unwrap_or(0) as u32; // mov.b @r4,r0
            let t = r5 == 0; // tst r5,r5
            r1 = (r1 << 8) | (b & 0xff); // shll8 r1 ; extu.b r0,r0 ; or r0,r1
            r4 = r4.wrapping_add(1); // add #1,r4
            r5 = r5.wrapping_sub(1); // bf.s delay slot: add #-1,r5 (runs every iteration)
            if t {
                break;
            }
        }
        let idx = (((r6 >> 24) ^ r1) & 0xff) << 2; // shlr8+shlr16+exts.b ; xor r1,r0 ; extu.b ; shll2
        r1 = idx;
        let entry = mem.read_u32(table.wrapping_add(idx)).unwrap_or(0); // mov.l @(r0,r1),r0
        r6 = (r6 << 8) ^ entry; // shll8 r6 ; xor r0,r6
    }
    cpu.r[0] = r6; // rts delay slot: mov r6,r0
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[14] = r14_saved; // mov.l @r15+,r14
    cpu.set_t(true); // last tst on either exit path leaves T = 1
}
