//! 0x8c07d9ac — substring search: find the p4-byte pattern at r5 inside the
//! buffer described by *r4, starting at offset r6; return the offset from the
//! buffer base, or -1.
//!
//! CFG sketch (struct = *r4: [0]=size u32, [12]=base u32):
//!   if (r6 + r7) > size (unsigned) → return -1        // bt 0x8c07d9c4
//!   if r7 == 0 → return -1                            // tst/bf, fall into d9c4
//!   start = base + r6, end = base + size - r7 + 1
//!   scan p from start while p != end for pattern[0]:
//!     on match: if r7 <= 1 → return p - base
//!               else compare pattern[1..r7) against p[1..):
//!                 mismatch → p += 1, rescan
//!                 full match → return p - base
//!   exhausted → return -1
//!
//! Side effects the decompiled C does not show: r4 is clobbered with the buffer
//! base, r3 ends as the end pointer, r1 as the final scan pointer, r6 as the
//! last candidate byte compared (inner loop only — seed on the len-1 path),
//! r0 as the result, and T follows the last cmp: true on every -1 exit, false
//! on a match. r11..r14 are pushed/popped (finals = seeds), r15 nets to zero.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r3 = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r3
    cpu.r[3] = r3;
    let sum = cpu.r[6].wrapping_add(cpu.r[7]); // mov r6,r0; add r7,r0
    let size = mem.read_u32(r3).unwrap_or(0); // mov.l @r3,r1
    cpu.r[1] = size;
    cpu.set_t(sum > size); // cmp/hi r1,r0 → T = r0 > r1
    if sum > size {
        cpu.r[0] = 0xffff_ffff; // mov #-1,r0 (delay of bra 0x8c07da20)
        return;
    }
    cpu.set_t(cpu.r[7] == 0); // tst r7,r7 → T = (r7 == 0)
    if cpu.r[7] == 0 {
        cpu.r[0] = 0xffff_ffff; // fall through 0x8c07d9c4
        return;
    }
    let base = mem.read_u32(r3.wrapping_add(12)).unwrap_or(0); // mov.l @(12,r3),r4
    cpu.r[4] = base;
    let end = base
        .wrapping_add(size)
        .wrapping_sub(cpu.r[7])
        .wrapping_add(1); // r3
    cpu.r[3] = end;
    let mut p = base.wrapping_add(cpu.r[6]); // r1 = start
    let p0 = sext8(mem, cpu.r[5]); // 0x8c07d9da, delay slot of bt.s, always runs
    cpu.r[0] = p0;
    cpu.set_t(p == end); // 0x8c07d9d6 cmp/eq r3,r1
    if p == end {
        cpu.r[1] = p; // r1 still holds the start pointer here
        cpu.r[0] = 0xffff_ffff; // bt.s 0x8c07da1e
        return;
    }
    let pat0 = p0; // exts.b r0,r12
    let r13 = 1u32;
    // 0x8c07d9e0 outer scan loop
    loop {
        cpu.set_t(p == end); // cmp/eq r3,r1
        if p == end {
            cpu.r[1] = p;
            cpu.r[0] = 0xffff_ffff; // bt 0x8c07da1e
            return;
        }
        // 0x8c07d9e4 inner scan for pattern[0]
        loop {
            let c = sext8(mem, p); // mov.b @r1,r0; exts.b r0,r0
            cpu.r[0] = c;
            cpu.set_t(c == pat0); // cmp/eq r12,r0 → T = r0 == r12
            if c == pat0 {
                // 0x8c07d9f6 match
                let mut q = p; // mov r1,r11
                cpu.set_t(cpu.r[7] > r13); // cmp/hi r13,r7 → T = r7 > r13
                cpu.r[0] = r13; // delay slot of bf.s at 0x8c07d9fa, always runs
                if !(cpu.r[7] > r13) {
                    // bf.s 0x8c07da12: len-1 match
                    let off = p.wrapping_sub(base); // sub r4,r1
                    cpu.r[1] = off;
                    cpu.r[0] = off; // delay of bra 0x8c07da20: mov r1,r0
                    return;
                }
                // 0x8c07d9fe inner compare loop
                let mut i = r13; // r0 = 1
                loop {
                    q = q.wrapping_add(1); // add #1,r11
                    let pc = sext8(mem, cpu.r[5].wrapping_add(i)); // mov.b @(r0,r5),r14
                    let cc = sext8(mem, q); // mov.b @r11,r6
                    cpu.r[6] = cc;
                    cpu.set_t(cc == pc); // cmp/eq r6,r14 → T = r14 == r6
                    if cc != pc {
                        // bf 0x8c07da18: mismatch, advance and rescan
                        p = p.wrapping_add(1);
                        cpu.set_t(p == end); // 0x8c07da1a
                        if p != end {
                            break; // bf 0x8c07d9e0 → outer loop
                        }
                        cpu.r[1] = p;
                        cpu.r[0] = 0xffff_ffff; // fall to 0x8c07da1e
                        return;
                    }
                    i = i.wrapping_add(1); // add #1,r0
                    cpu.set_t(cpu.r[7] > i); // cmp/hi r0,r7 → T = r7 > r0
                    if cpu.r[7] > i {
                        continue; // bt 0x8c07d9fe
                    }
                    // full match → 0x8c07da12
                    let off = p.wrapping_sub(base);
                    cpu.r[1] = off;
                    cpu.r[0] = off; // delay of bra 0x8c07da20
                    return;
                }
                continue; // rescan from the advanced p
            }
            p = p.wrapping_add(1); // add #1,r1
            cpu.set_t(p == end); // cmp/eq r3,r1
            if p == end {
                cpu.r[1] = p;
                cpu.r[0] = 0xffff_ffff; // 0x8c07d9f2 bra 0x8c07da1e
                return;
            }
            // bf 0x8c07d9e4
        }
    }
}
