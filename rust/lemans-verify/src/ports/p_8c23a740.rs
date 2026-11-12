//! 0x8c23a740 — write a flags word into the record for a table entry.
//!
//! `0x8c237780(r4)` locates the entry; a miss (or an index past `[entry+12]`)
//! returns 0x0000eeee. Otherwise the word at `[r15+4]` gets 0x10000000 ORed in,
//! or 0xefffffff ANDed instead when the index is the last one, and is stored at
//! `entry + 72 + index*4` (computed with `shll2` + `shll`, so T follows the
//! shifted-out bit). r14 is restored on every exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14]; // mov.l r14,@-r15 ; sts.l pr,@-r15 ; add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let arg0 = cpu.r[4];
    let arg2 = cpu.r[6];
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    let _ = mem.write_u32(cpu.r[15], arg0); // mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), arg2); // mov.l r6,@(4,r15)
    cpu.r[3] = mem.read_u32(0x8c23a7a0).unwrap_or(0x8c237780); // mov.l 0x8c23a7a0,r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay: mov.l @r15,r4
    cpu.pr = 0x8c23a752;
    crate::registry::p_8c237780::run(cpu, mem); // jsr @r3
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        return exit(cpu, saved14, true); // bt 0x8c23a782
    }
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r4),r3
    cpu.set_t(cpu.r[14] >= cpu.r[3]); // cmp/hs r3,r14
    if cpu.t() {
        return exit(cpu, saved14, true); // bt 0x8c23a782
    }
    // 0x8c23a75e
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r4),r2
    cpu.r[3] = cpu.r[14].wrapping_add(1); // mov r14,r3 ; add #1,r3
    cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r2,r3
    cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // bt.s delay: mov.l @(4,r15),r5
    if cpu.r[2] != cpu.r[3] {
        cpu.r[1] = mem.read_u32(0x8c23a7a4).unwrap_or(0x10000000); // mov.l 0x8c23a7a4,r1
        cpu.r[5] |= cpu.r[1]; // bra delay: or r1,r5
    } else {
        cpu.r[2] = mem.read_u32(0x8c23a7a8).unwrap_or(0xefffffff); // mov.l 0x8c23a7a8,r2
        cpu.r[5] &= cpu.r[2]; // and r2,r5
    }
    // 0x8c23a774
    cpu.r[3] = cpu.r[4]; // mov r4,r3
    cpu.r[14] = cpu.r[14].wrapping_shl(2); // shll2 r14
    cpu.r[3] = cpu.r[3].wrapping_add(72); // add #72,r3
    cpu.set_t(cpu.r[14] >> 31 != 0); // shll r14 sets T
    cpu.r[14] = cpu.r[14].wrapping_shl(1); // shll r14
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[3]); // add r3,r14
    let _ = mem.write_u32(cpu.r[14], cpu.r[5]); // bra delay: mov.l r5,@r14
    cpu.r[0] = 0; // 0x8c23a78c: mov #0,r0
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15 ; lds.l @r15+,pr ; rts delay: @r15+,r14
    cpu.r[14] = saved14;
}

fn exit(cpu: &mut Cpu, saved14: u32, miss: bool) {
    if miss {
        cpu.r[0] = 0x0000eeee; // mov.l 0x8c23a7ac,r0
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15 ; lds.l @r15+,pr ; rts delay: @r15+,r14
    cpu.r[14] = saved14;
}
