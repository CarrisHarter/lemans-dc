//! 0x8c073738 — count matches among 0x30-byte entries at `param_5` in index
//! range [param_4, param_6), skipping index param_1.
//!
//! CFG sketch (flag = byte at the global PTR_DAT_8c073820 = 0x8c1f8210):
//!   key = (p3low > p2low)            ? (p2low<<16)|p3low   // bt 0x8c07375e
//!       : flag != 0                  ? (p3low<<16)|p2low   // 0x8c073768
//!       :                              (p2low<<16)|p3low   // 0x8c07375e
//!   if !(param_6 > param_4) return 0            // bf.s 0x8c07380c
//!   loop over i = param_4 .. param_6-1, entry = base + i*0x30:
//!     if i == param_4_skip(r4): no count
//!     else if entry[8/0xc/0x10] (u32) == key:
//!       flag != 0 → count += 2                   // bra delay slot
//!       flag == 0 → count += 1 iff one of (w2==p2low&&w0==p3low),
//!                   (w0==p2low&&w4==p3low), (w4==p2low&&w2==p3low)
//!   return count
//!
//! Side effects the decompiled C does not show: r1 := param_5 (kept), r2 :=
//! entry_r15+4 whenever the loop runs (loop-end `mov r15,r2; add #40,r2` with
//! r15 already 36 below entry), r3 := base+4 in the loop (param_6 only when
//! the loop is skipped), r5 := p3low and r6 := flag byte in the loop, r7 ends
//! at param_6, T is clear on every exit. r8..r14 are pushed/popped, so their
//! final values are the seeds; the two spills to @r15/@(4,r15) and the prologue
//! live in the stack window.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sp = cpu.r[15]; // entry r15; prologue/epilogue net zero, never touched here
    let base = mem.read_u32(sp).unwrap_or(0); // mov.l @(36,r15),r1 (r15 is sp-36 there)
    let p6 = mem.read_u32(sp.wrapping_add(4)).unwrap_or(0); // @(40,r15) = param_6
    cpu.r[1] = base;
    let p2 = cpu.r[5] & 0xffff; // extu.w r5,r3
    let p3 = cpu.r[6] & 0xffff; // extu.w r6,r11
    let key;
    if p3 > p2 {
        // cmp/hi r3,r11 → T = r11 > r3; bt 0x8c07375e (delay: or r0,r11)
        key = (p2 << 16) | p3;
    } else {
        let flag = mem.read_u8(0x8c1f8210).unwrap_or(0) as u32; // mov.l 0x8c073820
        cpu.set_t(flag == 0); // tst r0,r0 sets T when the AND is zero
        key = if flag != 0 {
            (p3 << 16) | p2 // bf 0x8c073768: shll16 r11; or r3,r11
        } else {
            (p2 << 16) | p3 // fall through to 0x8c07375e
        };
    }
    cpu.r[3] = p6; // mov.l @(40,r15),r3
    cpu.set_t((p6 as i32) > (cpu.r[7] as i32)); // cmp/gt r7,r3 → T = r3 > r7
    if !((p6 as i32) > (cpu.r[7] as i32)) {
        cpu.r[0] = 0; // bf.s taken; delay slot mov #0,r12; then mov r12,r0
        return;
    }
    let flag = mem.read_u8(0x8c1f8210).unwrap_or(0) as u32; // mov.b @r3,r9; extu.b
    cpu.r[5] = p3; // extu.w r6,r5
    cpu.r[6] = flag; // extu.b r9,r6
    cpu.r[3] = base.wrapping_add(4);
    let mut r7 = cpu.r[7];
    let mut off = r7.wrapping_mul(0x30); // shll/add/shll2/shll2 = i*0x30
    let mut count = 0u32;
    loop {
        // 0x8c0737a4: cmp/eq r4,r7 → T = r7 == r4
        if r7 != cpu.r[4] {
            let a8 = mem
                .read_u32(base.wrapping_add(8).wrapping_add(off))
                .unwrap_or(0);
            let mut matched = a8 == key;
            cpu.set_t(matched); // cmp/eq r11,r9 → T = r9 == r11
            if !matched {
                let a12 = mem
                    .read_u32(base.wrapping_add(12).wrapping_add(off))
                    .unwrap_or(0);
                matched = a12 == key;
                cpu.set_t(matched);
            }
            if !matched {
                let a16 = mem
                    .read_u32(base.wrapping_add(16).wrapping_add(off))
                    .unwrap_or(0);
                matched = a16 == key;
                cpu.set_t(matched);
            }
            if matched {
                cpu.set_t(flag == 0); // tst r6,r6
                if flag != 0 {
                    // bra 0x8c0737fe, delay slot add #2,r12
                    count = count.wrapping_add(2);
                } else {
                    // 0x8c0737c4: three u16 pair comparisons
                    let w2 = mem
                        .read_u16(base.wrapping_add(2).wrapping_add(off))
                        .unwrap_or(0) as u32;
                    let w0 = mem
                        .read_u16(base.wrapping_add(off))
                        .unwrap_or(0) as u32;
                    let w4 = mem
                        .read_u16(base.wrapping_add(4).wrapping_add(off))
                        .unwrap_or(0) as u32;
                    if w2 == p2 && w0 == p3 {
                        count = count.wrapping_add(1); // delay of bra at 0x8c0737d6
                    } else if w0 == p2 && w4 == p3 {
                        count = count.wrapping_add(1); // delay of bra at 0x8c0737ea
                    } else if w4 == p2 && w2 == p3 {
                        count = count.wrapping_add(1); // 0x8c0737fc
                    }
                }
            }
        }
        // 0x8c0737fe: loop end — runs every iteration
        cpu.r[2] = sp.wrapping_add(4); // mov r15,r2; add #40,r2 (r15 = sp-36 there)
        r7 = r7.wrapping_add(1);
        off = off.wrapping_add(0x30); // delay slot of bt.s
        cpu.set_t((p6 as i32) > (r7 as i32)); // cmp/gt r7,r9 → T = r9 > r7
        if !((p6 as i32) > (r7 as i32)) {
            break; // bt.s not taken
        }
    }
    cpu.r[7] = r7;
    cpu.r[0] = count; // mov r12,r0
}
