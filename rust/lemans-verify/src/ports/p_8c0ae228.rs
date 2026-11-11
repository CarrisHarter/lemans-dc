//! 0x8c0ae228 — replace each table entry with the low u16 of
//! `u32[r5 + entry*4]` (length-prefixed walk, mode 1; flat walk, mode 0).
//!
//! Original (mode 1 path):
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ; mov.l @r4,r0
//! cmp/eq #1,r0 ; bf mode0 ; bra check ; mov #0,r1
//! body: mov r1,r14 ; shll r14 ; mov.l @(4,r4),r6 ; mov r14,r0 ; mov r1,r3
//! mov.w @(r0,r6),r0 ; add #1,r3 ; mov r3,r7 ; mov r3,r6 ; extu.w r0,r0
//! shll r6 ; bra inner_check ; add r0,r7
//! inner: mov.l @(4,r4),r12 ; mov r6,r0 ; add #1,r3 ; mov.w @(r0,r12),r0
//! extu.w r0,r13 ; mov r5,r0 ; shll2 r13 ; mov.l @(r0,r13),r13
//! mov r6,r0 ; mov.w r13,@(r0,r12) ; add #2,r6
//! inner_check: cmp/gt r3,r7 ; bt inner
//! mov.l @(4,r4),r3 ; mov r14,r0 ; mov.w @(r0,r3),r0 ; extu.w r0,r0
//! add #1,r0 ; add r0,r1
//! check: mov.l @(8,r4),r0 ; cmp/gt r1,r0 ; bt body ; bra epilogue ; nop
//! mode0: tst r0,r0 ; bf epilogue ; mov #0,r6 ; bra check0 ; mov r6,r7
//! ... r1 = extu.w(idx) ; shll2 r1 ; mov.l @(r0,r1),r1 ; mov.w r1,@(r0,r3) ...
//! epilogue: mov.l @r15+,r12 ; mov #0,r0 ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r0` returns 0, `r1` = final outer index (mode 1)
//! or the last loaded u32 (mode 0), `r3` = table base, `r6`/`r7` keep the
//! inner counters, r12/r13/r14 are pushed/popped. T ends 0 on every path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mode = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mode;
    if mode == 1 {
        cpu.r[1] = 0; // bra delay slot
        loop {
            let count = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] = count;
            let t = (count as i32) > (cpu.r[1] as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
            let i = cpu.r[1];
            let i2 = i.wrapping_mul(2); // r14, popped later
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            let len = mem.read_u16(i2.wrapping_add(base)).unwrap_or(0) as u32;
            let mut k = i.wrapping_add(1);
            let kend = k.wrapping_add(len);
            let mut k2 = k.wrapping_mul(2); // r6 in the inner loop
            cpu.r[3] = k;
            cpu.r[7] = kend;
            cpu.r[6] = k2;
            loop {
                let t2 = (kend as i32) > (k as i32);
                cpu.set_t(t2);
                if !t2 {
                    break;
                }
                let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
                let idx = mem.read_u16(k2.wrapping_add(base)).unwrap_or(0) as u32;
                k = k.wrapping_add(1);
                cpu.r[3] = k;
                let v = mem.read_u32(cpu.r[5].wrapping_add(idx.wrapping_mul(4))).unwrap_or(0);
                cpu.r[13] = v; // r13, popped later
                let _ = mem.write_u16(k2.wrapping_add(base), v as u16);
                k2 = k2.wrapping_add(2);
                cpu.r[6] = k2;
            }
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[3] = base;
            let len = mem.read_u16(i2.wrapping_add(base)).unwrap_or(0) as u32;
            cpu.r[0] = len.wrapping_add(1);
            cpu.r[1] = cpu.r[1].wrapping_add(len.wrapping_add(1));
        }
        cpu.r[0] = 0;
    } else if mode == 0 {
        cpu.r[6] = 0;
        cpu.r[7] = 0; // bra delay slot
        loop {
            let count = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] = count;
            let t = (count as i32) > (cpu.r[6] as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[3] = base;
            let off = cpu.r[7];
            let idx = mem.read_u16(off.wrapping_add(base)).unwrap_or(0) as u32;
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(idx.wrapping_mul(4))).unwrap_or(0);
            let _ = mem.write_u16(off.wrapping_add(base), cpu.r[1] as u16);
            cpu.r[7] = off.wrapping_add(2);
        }
        cpu.r[0] = 0;
    } else {
        cpu.set_t(false); // tst r0,r0 with mode != 0
        cpu.r[0] = 0;
    }
}
