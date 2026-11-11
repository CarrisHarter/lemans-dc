//! 0x8c08c704 — ring-buffer insert (56-byte / 14-u32 entries) into the
//! struct at r4: base=[r4+0x38], count=[r4+0x3c], limit=[r4+0x40],
//! flag byte=[r4+0x44], head=[r4+0x50]; src record at r5.
//!
//! Paths:
//! - flag==0 && count<=limit -> return 0 (T=1, r0=0, r1=limit)
//! - flag!=0 -> copy 56 bytes r5 -> [r4+0x38], clear flag byte, return 1
//!   (r3 is zeroed at 0x8c08c738 inside the copy block)
//! - flag==0 && count>limit && end>head -> scan forward from head while
//!   entry f32 at +12 > threshold f32[0x8c08c870] (0.001) and end>ptr; copy at
//!   the found slot, head = slot+56, wrap head to base when head==end; return 1
//! - else if head>base -> same scan from base, copy there, head = slot+56,
//!   recompute end, wrap if head==end; return 1
//! - else -> return 0
//!
//! Residue: copy blocks leave r7=[src+40], r6=[src+48], r5=[src+52] (r7/r6
//! pattern) or r3=[src+48], r1=[src+52] (r3/r1 pattern); the rts delay slot
//! sets r0; T is the last cmp of each path.

use sh4_oracle::{Cpu, Memory};

/// Copy 14 u32s src -> dst with the r7/r6 load pattern; returns final
/// (r7, r6, r5) register values.
fn copy14_r76(mem: &mut Memory, src: u32, dst: u32) -> (u32, u32, u32) {
    let mut r7 = 0u32;
    let mut r6 = 0u32;
    for k in 0..6u32 {
        r7 = mem.read_u32(src.wrapping_add(8 * k)).unwrap_or(0);
        r6 = mem.read_u32(src.wrapping_add(8 * k + 4)).unwrap_or(0);
        let _ = mem.write_u32(dst.wrapping_add(8 * k), r7);
        let _ = mem.write_u32(dst.wrapping_add(8 * k + 4), r6);
    }
    r6 = mem.read_u32(src.wrapping_add(48)).unwrap_or(0);
    let r5 = mem.read_u32(src.wrapping_add(52)).unwrap_or(0);
    let _ = mem.write_u32(dst.wrapping_add(48), r6);
    let _ = mem.write_u32(dst.wrapping_add(52), r5);
    (r7, r6, r5)
}

/// Copy 14 u32s src -> dst with the r3/r1 load pattern; returns final (r3, r1).
fn copy14_r31(mem: &mut Memory, src: u32, dst: u32) -> (u32, u32) {
    let mut r3 = 0u32;
    let mut r1 = 0u32;
    for k in 0..6u32 {
        r3 = mem.read_u32(src.wrapping_add(8 * k)).unwrap_or(0);
        r1 = mem.read_u32(src.wrapping_add(8 * k + 4)).unwrap_or(0);
        let _ = mem.write_u32(dst.wrapping_add(8 * k), r3);
        let _ = mem.write_u32(dst.wrapping_add(8 * k + 4), r1);
    }
    r3 = mem.read_u32(src.wrapping_add(48)).unwrap_or(0);
    r1 = mem.read_u32(src.wrapping_add(52)).unwrap_or(0);
    let _ = mem.write_u32(dst.wrapping_add(48), r3);
    let _ = mem.write_u32(dst.wrapping_add(52), r1);
    (r3, r1)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let off_head = mem.read_u32(0x8c08c86c).unwrap_or(0); // 0x80: head ptr field
    // mov r4,r0 ; add #68,r0 ; mov.b @r0,r6 ; extu.b r6,r6 ; extu.b r6,r0
    let flag = mem.read_u8(r4.wrapping_add(68)).unwrap_or(0) as u32;
    cpu.r[6] = flag;
    cpu.set_t(flag == 0); // tst r0,r0
    let mut r3: u32 = 1; // mov #1,r3 (bf.s delay slot runs either way)
    if flag == 0 {
        // bf.s not taken: r1 = [r4+0x40] ; r0 = [r4+0x3c] ; cmp/gt r1,r0 ; bt ; mov #0,r3
        let r1 = mem.read_u32(r4.wrapping_add(64)).unwrap_or(0);
        let r0 = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
        cpu.r[1] = r1;
        let t = (r0 as i32) > (r1 as i32); // cmp/gt r1,r0 -> T = r0 > r1
        cpu.set_t(t);
        if !t {
            r3 = 0; // mov #0,r3
        }
    }
    cpu.r[3] = r3;
    // 0x8c08c722: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c08c72c
    cpu.set_t(r3 == 0);
    if r3 == 0 {
        cpu.r[0] = 0; // rts ; mov #0,r0
        return; // T = 1
    }
    // 0x8c08c72c: extu.b r6,r0 ; tst r0,r0 ; bt 0x8c08c776
    cpu.set_t(flag == 0);
    if flag != 0 {
        // copy 56 bytes r5 -> [r4+0x38], clear flag, return 1
        let dst = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0);
        let src = cpu.r[5];
        cpu.r[1] = dst;
        cpu.r[0] = 68; // mov #68,r0
        cpu.r[3] = 0; // mov #0,r3 (0x8c08c738)
        let (r7, r6, r5) = copy14_r76(mem, src, dst);
        cpu.r[7] = r7;
        cpu.r[6] = r6;
        cpu.r[5] = r5;
        let _ = mem.write_u8(r4.wrapping_add(68), 0); // mov.b r3,@(r0,r4)
        cpu.r[0] = 1; // rts ; mov #1,r0 — T stays 0
        return;
    }
    // 0x8c08c776: r1 = count ; r3 = count*56 + base (end) ; r7 = head ; r6 = base
    let count = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
    cpu.r[1] = count;
    let mut r3 = count << 2; // shll2 r3
    cpu.set_t((r3 >> 31) != 0);
    r3 = r3 << 1; // shll r3
    r3 = r3.wrapping_sub(count); // sub r1,r3 -> count*7
    let r7 = mem.read_u32(r4.wrapping_add(off_head)).unwrap_or(0); // head
    cpu.r[7] = r7;
    r3 = r3 << 2; // shll2 -> count*28
    let r6 = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // base
    cpu.r[6] = r6;
    cpu.set_t((r3 >> 31) != 0);
    r3 = r3 << 1; // shll -> count*56
    r3 = r3.wrapping_add(r6); // add r6,r3 -> end
    cpu.r[3] = r3;
    // cmp/hi r7,r3 -> T = r3 > r7 ; bf.s 0x8c08c7f2 ; delay: mov r7,r1
    cpu.set_t(r3 > r7);
    let mut r1 = r7;
    cpu.r[1] = r1;
    if r3 > r7 {
        // head path: scan from r1 = head (0x8c08c792: mova ; fmov @r0,fr0)
        cpu.fr[0] = mem.read_f32(0x8c08c870).unwrap_or(0.0);
        loop {
            // 0x8c08c796: mov r1,r0 ; add #12,r0 ; fmov @r0,fr1 ; fcmp/gt fr0,fr1 ; bt 0x8c08c7ec
            cpu.r[0] = r1.wrapping_add(12);
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
            cpu.set_t(cpu.fr[1] > cpu.fr[0]);
            if cpu.t() {
                // 0x8c08c7ec: add #56,r1 ; cmp/hi r1,r3 ; bt 0x8c08c796
                r1 = r1.wrapping_add(56);
                cpu.r[1] = r1;
                cpu.set_t(r3 > r1);
                if r3 > r1 {
                    continue;
                }
                break; // scan exhausted -> tail section
            }
            // copy 56 bytes r5 -> r1 (0x8c08c7a0)
            let src = cpu.r[5];
            let (r7f, r6f, r5f) = copy14_r76(mem, src, r1);
            cpu.r[7] = r7f;
            cpu.r[6] = r6f;
            cpu.r[5] = r5f;
            r1 = r1.wrapping_add(56); // add #56,r1
            let _ = mem.write_u32(r4.wrapping_add(off_head), r1); // mov.l r1,@(r0,r4)
            let r1b = mem.read_u32(r4.wrapping_add(off_head)).unwrap_or(0); // mov.l @(r0,r4),r1
            cpu.r[1] = r1b;
            cpu.set_t(r1b == r3); // cmp/eq r3,r1
            if r1b == r3 {
                let base = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // mov.l @(56,r4),r1
                let _ = mem.write_u32(r4.wrapping_add(off_head), base); // mov.l r1,@(r0,r4)
                cpu.r[1] = base;
            }
            cpu.r[0] = 1; // rts ; mov #1,r0
            return;
        }
    }
    // tail section 0x8c08c7f2: cmp/hi r6,r7 -> T = r7 > r6 ; bf 0x8c08c866
    cpu.set_t(r7 > r6);
    if !(r7 > r6) {
        cpu.r[0] = 0; // rts ; mov #0,r0
        return; // T = 0
    }
    // 0x8c08c7f6: mova 0x8c08c870,r0 ; fmov @r0,fr0 ; scan from r6 = base
    cpu.fr[0] = mem.read_f32(0x8c08c870).unwrap_or(0.0);
    let mut r6 = r6;
    loop {
        // 0x8c08c7fa: mov r6,r0 ; add #12,r0 ; fmov @r0,fr1 ; fcmp/gt fr0,fr1 ; bt 0x8c08c860
        cpu.r[0] = r6.wrapping_add(12);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            // 0x8c08c860: add #56,r6 ; cmp/hi r6,r7 ; bt 0x8c08c7fa
            r6 = r6.wrapping_add(56);
            cpu.r[6] = r6;
            cpu.set_t(r7 > r6);
            if r7 > r6 {
                continue;
            }
        }
        break; // copy at r6
    }
    // copy 56 bytes r5 -> r6 (0x8c08c804, r3/r1 pattern)
    let src = cpu.r[5];
    let _ = copy14_r31(mem, src, r6);
    r6 = r6.wrapping_add(56); // add #56,r6
    cpu.r[6] = r6;
    let _ = mem.write_u32(r4.wrapping_add(off_head), r6); // mov.l r6,@(r0,r4)
    // 0x8c08c842: r1 = count ; r5 = base ; r3 = count*56 + base ; r1 = head ; cmp/eq r1,r3
    let count = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
    let base = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0);
    cpu.r[5] = base;
    let mut r3 = count << 2;
    cpu.set_t((r3 >> 31) != 0);
    r3 = r3 << 1;
    r3 = r3.wrapping_sub(count);
    r3 = r3 << 2;
    cpu.set_t((r3 >> 31) != 0);
    r3 = r3 << 1;
    let r1 = mem.read_u32(r4.wrapping_add(off_head)).unwrap_or(0);
    r3 = r3.wrapping_add(base);
    cpu.r[3] = r3;
    cpu.r[1] = r1;
    cpu.set_t(r1 == r3); // cmp/eq r1,r3
    if r1 == r3 {
        let _ = mem.write_u32(r4.wrapping_add(off_head), base); // mov.l r5,@(r0,r4)
    }
    cpu.r[0] = 1; // rts ; mov #1,r0
}
