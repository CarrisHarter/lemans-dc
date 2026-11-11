//! 0x8c0ae1a0 — overwrite every table entry equal to `r5` with `r6` (u16),
//! walking the length-prefixed table (mode 1) or the flat table (mode 0);
//! returns the number of replacements in `r0`.
//!
//! Original (mode 1 path):
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ; mov.l r11,@-r15
//! mov.l @r4,r0 ; cmp/eq #1,r0 ; bf.s mode0 ; mov #0,r1
//! bra check ; mov r1,r3
//! body: mov r3,r13 ; shll r13 ; mov.l @(4,r4),r11 ; mov r13,r0 ; mov r3,r7
//! mov.w @(r0,r11),r11 ; add #1,r7 ; mov r7,r14 ; mov r7,r0 ; extu.w r11,r11
//! shll r0 ; bra inner_check ; add r11,r14
//! inner: mov.l @(4,r4),r11 ; mov.w @(r0,r11),r12 ; extu.w r12,r12
//! cmp/eq r5,r12 ; bf next ; mov.w r6,@(r0,r11) ; add #1,r1
//! next: add #2,r0 ; add #1,r7
//! inner_check: cmp/gt r7,r14 ; bt inner
//! mov.l @(4,r4),r7 ; mov r13,r0 ; mov.w @(r0,r7),r0 ; extu.w r0,r0
//! add #1,r0 ; add r0,r3
//! check: mov.l @(8,r4),r0 ; cmp/gt r3,r0 ; bt body ; bra epilogue ; nop
//! mode0: tst r0,r0 ; bf epilogue ; mov r1,r11 ; bra check0 ; mov r1,r0
//! ... cmp/eq r5,r3 ; mov.w r6,@(r0,r7) ...
//! epilogue: mov.l @r15+,r11 ; mov r1,r0 ; mov.l @r15+,r12 ; mov.l @r15+,r13
//! rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r0` = `r1` = match count, `r3` = final outer
//! index (mode 1) or `r4->count` (mode 0), `r7` = table base, `r1` is zeroed
//! by the `bf.s` delay slot on every path, and r11..r14 are pushed/popped.
//! T ends 0 on every path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mode = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mode;
    cpu.r[1] = 0; // bf.s delay slot runs on every path
    if mode == 1 {
        cpu.r[3] = cpu.r[1]; // bra delay slot
        loop {
            let count = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] = count;
            let t = (count as i32) > (cpu.r[3] as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
            let i = cpu.r[3];
            let i2 = i.wrapping_mul(2); // r13, popped later
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            let len = mem.read_u16(i2.wrapping_add(base)).unwrap_or(0) as u32;
            let mut k = i.wrapping_add(1);
            let kend = k.wrapping_add(len); // r14, popped later
            let mut k2 = k.wrapping_mul(2); // r0 in the inner loop
            cpu.r[7] = k;
            loop {
                let t2 = (kend as i32) > (k as i32);
                cpu.set_t(t2);
                if !t2 {
                    break;
                }
                let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
                let v = mem.read_u16(k2.wrapping_add(base)).unwrap_or(0) as u32;
                if v == cpu.r[5] {
                    let _ = mem.write_u16(k2.wrapping_add(base), cpu.r[6] as u16);
                    cpu.r[1] = cpu.r[1].wrapping_add(1);
                }
                k2 = k2.wrapping_add(2);
                k = k.wrapping_add(1);
                cpu.r[7] = k;
            }
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[7] = base;
            let len = mem.read_u16(i2.wrapping_add(base)).unwrap_or(0) as u32;
            cpu.r[0] = len.wrapping_add(1);
            cpu.r[3] = cpu.r[3].wrapping_add(len.wrapping_add(1));
        }
    } else if mode == 0 {
        let mut i = cpu.r[1]; // r11, popped later
        cpu.r[0] = cpu.r[1]; // bra delay slot
        loop {
            let count = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.r[3] = count;
            let t = (count as i32) > (i as i32);
            cpu.set_t(t);
            if !t {
                break;
            }
            let base = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[7] = base;
            let off = cpu.r[0];
            let v = mem.read_u16(off.wrapping_add(base)).unwrap_or(0) as u32;
            cpu.r[3] = v;
            if v == cpu.r[5] {
                let _ = mem.write_u16(off.wrapping_add(base), cpu.r[6] as u16);
                cpu.r[1] = cpu.r[1].wrapping_add(1);
            }
            cpu.r[0] = off.wrapping_add(2);
            i = i.wrapping_add(1);
        }
    } else {
        cpu.set_t(false); // tst r0,r0 with mode != 0
    }
    cpu.r[0] = cpu.r[1]; // epilogue: mov r1,r0
}
