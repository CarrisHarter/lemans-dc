//! 0x8c18d28c — two-level unsigned key search in a binary tree.
//!
//! Original shape: walk from `*(r4+28)`, at each node comparing the key pair
//! at r5 against the node's +12/+16 pair (`cmp/hi` is unsigned `Rn > Rm`);
//! direction 0 keeps the node in r0 and descends to +4 (right), direction 1
//! descends to +0 (left). After the walk, re-compare the found node against
//! the key; if the key is greater-or-equal (r1 != 0) store `r4+28` through
//! the pointer at `[r15+4]`, else store the found node through it. The `rts`
//! delay slot pops r14, so r14/r15 return to their entry values.
//!
//! Flag details: every `bt.s`/`bra` delay slot runs; the final T is the
//! `tst r1,r1` at 0x8c18d2ea (1 = node stored, 0 = r4+28 stored). On an empty
//! tree the walk is skipped and r1 keeps its entry value (low byte decides
//! the store target), while r6/r7 keep their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r15_entry = cpu.r[15];
    let r15 = r15_entry.wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(r15, cpu.r[14]);
    let mut r14 = mem.read_u32(r4.wrapping_add(28)).unwrap_or(0);
    let mut r0 = 0u32; // bt.s delay slot: mov #0,r0 (always runs)
    let mut r1 = cpu.r[1];
    let mut r3 = cpu.r[3];
    let mut r6 = cpu.r[6];
    let mut r7 = cpu.r[7];
    cpu.set_t(r14 == 0); // tst r14,r14
    if r14 != 0 {
        r6 = r0; // mov r0,r6
        r7 = 1; // mov #1,r7
        loop {
            // 0x8c18d29a
            r3 = mem.read_u32(r5).unwrap_or(0);
            r1 = mem.read_u32(r14.wrapping_add(12)).unwrap_or(0);
            cpu.set_t(r1 > r3); // cmp/hi r3,r1
            if cpu.t() {
                r1 = r7; // bra 0x8c18d2b6 delay: mov r7,r1
            } else {
                cpu.set_t(r3 > r1); // cmp/hi r1,r3
                if cpu.t() {
                    r1 = r6; // bra 0x8c18d2b6 delay: mov r6,r1
                } else {
                    r3 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
                    r1 = mem.read_u32(r14.wrapping_add(16)).unwrap_or(0);
                    cpu.set_t(r1 > r3); // cmp/hi r3,r1
                    r1 = u32::from(cpu.t()); // movt r1
                }
            }
            // 0x8c18d2b6
            r1 &= 0xff; // extu.b r1,r1
            cpu.set_t(r1 == 0); // tst r1,r1
            if r1 == 0 {
                // bt 0x8c18d2c0
                r0 = r14;
                r14 = mem.read_u32(r14.wrapping_add(4)).unwrap_or(0);
            } else {
                // bra 0x8c18d2c4 delay: mov.l @r14,r14
                r14 = mem.read_u32(r14).unwrap_or(0);
            }
            // 0x8c18d2c4
            cpu.set_t(r14 == 0); // tst r14,r14
            if r14 == 0 {
                break; // bf not taken: fall through to 0x8c18d2c8
            }
        }
    }
    // 0x8c18d2c8
    cpu.set_t(r0 == 0); // tst r0,r0
    if r0 == 0 {
        // bt 0x8c18d2ee taken: the whole re-compare block is skipped,
        // r1 keeps its entry value and T stays 1.
        r0 = mem.read_u32(r15.wrapping_add(4)).unwrap_or(0);
        r4 = r4.wrapping_add(28);
        let _ = mem.write_u32(r0, r4); // bra delay slot: mov.l r4,@r0
    } else {
        r3 = mem.read_u32(r0.wrapping_add(12)).unwrap_or(0);
        r1 = mem.read_u32(r5).unwrap_or(0);
        cpu.set_t(r1 > r3); // cmp/hi r3,r1
        if cpu.t() {
            r1 = 1; // bra 0x8c18d2e8 delay: mov #1,r1
        } else {
            cpu.set_t(r3 > r1); // cmp/hi r1,r3
            if cpu.t() {
                r1 = 0; // bra 0x8c18d2e8 delay: mov #0,r1
            } else {
                r3 = mem.read_u32(r0.wrapping_add(16)).unwrap_or(0);
                r1 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
                cpu.set_t(r1 > r3); // cmp/hi r3,r1
                r1 = u32::from(cpu.t()); // movt r1
            }
        }
        // 0x8c18d2e8
        r1 &= 0xff; // extu.b r1,r1
        cpu.set_t(r1 == 0); // tst r1,r1
        if r1 == 0 {
            // bt 0x8c18d2f6: r1 = [r15+4]; *r1 = r0
            r1 = mem.read_u32(r15.wrapping_add(4)).unwrap_or(0);
            let _ = mem.write_u32(r1, r0);
        } else {
            // 0x8c18d2ee: r0 = [r15+4]; r4 += 28; bra delay: *r0 = r4
            r0 = mem.read_u32(r15.wrapping_add(4)).unwrap_or(0);
            r4 = r4.wrapping_add(28);
            let _ = mem.write_u32(r0, r4);
        }
    }
    // rts delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(r15).unwrap_or(cpu.r[14]);
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[15] = r15_entry;
}
