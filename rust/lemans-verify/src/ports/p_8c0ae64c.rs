//! 0x8c0ae64c — walk the table at `r4` and store the min/max of every
//! reachable u16 entry into `r4+0xc` / `r4+0x10` (min starts at the
//! DAT_8c0ae6d4 literal 999999999, max at 0; comparisons are signed).
//!
//! Original (mode 1 path):
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ; mov.l r11,@-r15
//! mov.l @r4,r0 ; mov.l 0x8c0ae6d4,r3 ; cmp/eq #1,r0 ; bf.s mode0 ; mov #0,r5
//! mov.l @(8,r4),r13 ; bra check ; mov r5,r6
//! body: mov.l @(4,r4),r12 ; mov r6,r0 ; shll r0 ; mov.w @(r0,r12),r1
//! mov r6,r7 ; add #1,r7 ; mov r7,r0 ; mov r7,r14 ; extu.w r1,r11 ; shll r0
//! bra inner_check ; add r11,r14
//! inner: mov.w @(r0,r12),r1 ; extu.w r1,r1 ; cmp/gt r1,r3 ; bf s1 ; mov r1,r3
//! s1: cmp/gt r5,r1 ; bf s2 ; mov r1,r5 ; s2: add #2,r0 ; add #1,r7
//! inner_check: cmp/gt r7,r14 ; bt inner
//! add #1,r11 ; add r11,r6
//! check: cmp/gt r6,r13 ; bt body ; bra stores ; nop
//! mode0: tst r0,r0 ; bf stores ; mov.l @(8,r4),r7 ; mov r5,r6 ; bra check0 ; mov r5,r0
//! body0: mov.l @(4,r4),r1 ; mov.w @(r0,r1),r1 ; extu.w r1,r1 ; cmp/gt r1,r3
//! ... add #2,r0 ; add #1,r6 ; check0: cmp/gt r6,r7 ; bt body0
//! stores: mov.l r3,@(12,r4) ; mov.l r5,@(16,r4)
//! mov.l @r15+,r11 ; mov.l @r15+,r12 ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r3`/`r5` end as the stored min/max, `r6` = final
//! outer index (mode 1) or entry count (mode 0), `r7` = last inner counter or
//! the count (mode 0), `r0` = last inner byte offset, `r1` = last entry read,
//! and the stores run on every path (mode not 0/1 stores 999999999/0).
//! r11..r14 are pushed/popped. T ends 0 on every path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mode = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mode;
    cpu.r[3] = mem.read_u32(0x8c0ae6d4).unwrap_or(0); // 999999999
    cpu.r[5] = 0; // bf.s delay slot runs on every path
    if mode == 1 {
        let count = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
        cpu.r[6] = 0; // bra delay slot: r6 = r5
        loop {
            let t = (count as i32) > (cpu.r[6] as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
            let i = cpu.r[6];
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            let len = mem.read_u16(i.wrapping_mul(2).wrapping_add(base)).unwrap_or(0) as u32;
            let mut k = i.wrapping_add(1);
            let kend = k.wrapping_add(len);
            let mut k2 = k.wrapping_mul(2);
            cpu.r[0] = k2;
            cpu.r[7] = k;
            loop {
                let t2 = (kend as i32) > (k as i32);
                cpu.set_t(t2);
                if !t2 {
                    break;
                }
                let v = mem.read_u16(k2.wrapping_add(base)).unwrap_or(0) as u32;
                cpu.r[1] = v;
                if (cpu.r[3] as i32) > (v as i32) {
                    cpu.r[3] = v;
                }
                if (v as i32) > (cpu.r[5] as i32) {
                    cpu.r[5] = v;
                }
                k2 = k2.wrapping_add(2);
                cpu.r[0] = k2;
                k = k.wrapping_add(1);
                cpu.r[7] = k;
            }
            cpu.r[6] = cpu.r[6].wrapping_add(len.wrapping_add(1));
        }
    } else if mode == 0 {
        let count = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
        cpu.r[7] = count;
        cpu.r[6] = 0;
        cpu.r[0] = 0; // bra delay slot: r0 = r5
        let mut off = 0u32;
        loop {
            let t = (count as i32) > (cpu.r[6] as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[1] = base;
            let v = mem.read_u16(off.wrapping_add(base)).unwrap_or(0) as u32;
            cpu.r[1] = v;
            if (cpu.r[3] as i32) > (v as i32) {
                cpu.r[3] = v;
            }
            if (v as i32) > (cpu.r[5] as i32) {
                cpu.r[5] = v;
            }
            off = off.wrapping_add(2);
            cpu.r[0] = off;
            cpu.r[6] = cpu.r[6].wrapping_add(1);
        }
    } else {
        cpu.set_t(false); // tst r0,r0 with mode != 0
    }
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[5]);
}
