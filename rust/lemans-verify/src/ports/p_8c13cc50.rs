//! 0x8c13cc50 — 24-byte-node sibling of 0x8c13cacc.
//!
//! Layout: [r4+56]=base B, [r4+60]=count N, [r4+64]=threshold, [r4+68]=flag
//! byte, [r4+0x80]=head H. Node stride 24; tail bound 24*N + B. Flag byte
//! nonzero -> copy 24 bytes from r5 to *B, clear the byte, return 1. Flag
//! zero but N > threshold -> walk from H while 24N+B > r7, copying the first
//! node whose float at +12 is <= f32[0x8c13cd58]; else walk from B while
//! H > r3. A copy advances the head by 24 (wrapping to B at 24N+B) and
//! returns 1; everything else returns 0.

use sh4_oracle::{Cpu, Memory};

/// 6-long copy, r6/r3 interleaved pattern: leaves r6=src[4], r3=src[5].
fn copy24(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..3 {
        cpu.r[6] = mem.read_u32(s).unwrap_or(0);
        cpu.r[3] = mem.read_u32(s + 4).unwrap_or(0);
        let _ = mem.write_u32(d, cpu.r[6]);
        let _ = mem.write_u32(d + 4, cpu.r[3]);
        d += 8;
        s += 8;
    }
}
/// 6-long copy, r6/r1 interleaved pattern (0x8c13cd0e site, r3 is the cursor):
/// leaves r6=src[4], r1=src[5], r3 untouched.
fn copy24_r6r1(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..3 {
        cpu.r[6] = mem.read_u32(s).unwrap_or(0);
        cpu.r[1] = mem.read_u32(s + 4).unwrap_or(0);
        let _ = mem.write_u32(d, cpu.r[6]);
        let _ = mem.write_u32(d + 4, cpu.r[1]);
        d += 8;
        s += 8;
    }
}


pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];

    // 0x8c13cc50: r0 = r4+68 ; r6 = mov.b @r0 ; extu.b x2 ; tst ; bf.s 0x8c13cc6e
    cpu.r[0] = r4.wrapping_add(68);
    cpu.r[6] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[6];
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    let mut r3: u32 = 1; // bf.s delay slot: mov #1,r3 (always runs)
    if cpu.t() {
        // r6 == 0: r0 = r4+64 ; r1 = @r0 ; r0 = @(60,r4) ; cmp/gt r1,r0 ; bt 0x8c13cc6e
        cpu.r[0] = r4.wrapping_add(64);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32)); // cmp/gt r1,r0
        if !cpu.t() {
            r3 = 0; // mov #0,r3
        }
    }
    cpu.r[3] = r3;
    // 0x8c13cc6e: extu.b r3,r0 ; tst ; bf 0x8c13cc78
    cpu.r[0] = r3 & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        return;
    }
    // 0x8c13cc78: extu.b r6,r0 ; tst ; bt 0x8c13cca2
    cpu.r[0] = cpu.r[6] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        return tail(cpu, mem, r4);
    }
    // r6 != 0: copy 24 bytes from r5 to [[r4+56]], clear flag byte, return 1
    cpu.r[7] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // dest
    cpu.r[0] = 68; // mov #68,r0
    cpu.r[1] = 0; // mov #0,r1
    let dst = cpu.r[7];
    let src = cpu.r[5];
    copy24(cpu, mem, dst, src);
    let _ = mem.write_u8(r4.wrapping_add(cpu.r[0]), cpu.r[1] as u8); // mov.b r1,@(r0,r4)
    cpu.r[0] = 1; // rts delay slot: mov #1,r0
}

fn tail(cpu: &mut Cpu, mem: &mut Memory, r4: u32) {
    // 0x8c13cca2..0x8c13ccb8: end = 24*N + B in r1 ; r6 = H ; cmp/hi r6,r1
    cpu.r[3] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0); // N
    cpu.r[0] = mem.read_u32(0x8c13cd54).unwrap_or(0); // 0x80
    cpu.r[1] = cpu.r[3];
    cpu.set_t(cpu.r[1] >> 31 != 0); // shll r1: T = old bit 31
    cpu.r[1] <<= 1; // 2N
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // 3N
    cpu.r[3] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // B
    cpu.r[1] <<= 2; // shll2 -> 12N
    cpu.r[6] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // H
    cpu.set_t(cpu.r[1] >> 31 != 0); // shll r1: T = old bit 31 of 12N
    cpu.r[1] <<= 1; // 24N
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // end = 24N + B
    cpu.set_t(cpu.r[1] > cpu.r[6]); // cmp/hi r6,r1
    cpu.r[7] = cpu.r[6]; // bf.s delay slot: mov r6,r7 (always runs)
    let end = cpu.r[1];

    if cpu.t() {
        // first loop 0x8c13ccc0: r0 = r7+12 ; fr1 = @r0 ; fcmp/gt fr0,fr1
        cpu.r[0] = 0x8c13cd58; // mova
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        loop {
            cpu.r[0] = cpu.r[7].wrapping_add(12);
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
            cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
            if cpu.t() {
                // 0x8c13ccf6: add #24,r7 ; cmp/hi r7,r1 ; bt 0x8c13ccc0
                cpu.r[7] = cpu.r[7].wrapping_add(24);
                cpu.set_t(end > cpu.r[7]);
                if cpu.t() {
                    continue;
                }
                break;
            }
            let dst = cpu.r[7];
            let src = cpu.r[5];
            copy24(cpu, mem, dst, src);
            cpu.r[0] = mem.read_u32(0x8c13cd54).unwrap_or(0); // 0x80
            cpu.r[7] = cpu.r[7].wrapping_add(24);
            let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[7]); // mov.l r7,@(r0,r4)
            cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // mov.l @(r0,r4),r3
            cpu.set_t(cpu.r[1] == cpu.r[3]); // cmp/eq r1,r3
            if cpu.t() {
                cpu.r[1] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]);
            }
            cpu.r[0] = 1; // rts delay slot: mov #1,r0
            return;
        }
    }
    // 0x8c13ccfc: cmp/hi r3,r6 ; bf 0x8c13cd4e
    cpu.set_t(cpu.r[6] > cpu.r[3]);
    if !cpu.t() {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        return;
    }
    // second loop 0x8c13cd04: r0 = r3+12 ; fr1 = @r0 ; fcmp/gt fr0,fr1
    cpu.r[0] = 0x8c13cd58; // mova
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    let h = cpu.r[6];
    loop {
        cpu.r[0] = cpu.r[3].wrapping_add(12);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
        if cpu.t() {
            // 0x8c13cd48: add #24,r3 ; cmp/hi r3,r6 ; bt 0x8c13cd04
            cpu.r[3] = cpu.r[3].wrapping_add(24);
            cpu.set_t(h > cpu.r[3]);
            if cpu.t() {
                continue;
            }
            cpu.r[0] = 0; // rts delay slot at 0x8c13cd4e: mov #0,r0
            return;
        }
        let dst = cpu.r[3];
        let src = cpu.r[5];
        copy24_r6r1(cpu, mem, dst, src);
        cpu.r[0] = mem.read_u32(0x8c13cd54).unwrap_or(0); // 0x80
        cpu.r[3] = cpu.r[3].wrapping_add(24);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[3]); // mov.l r3,@(r0,r4)
        // recompute end = 24*N + B into r3
        cpu.r[1] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0); // N
        cpu.r[5] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // B
        let mut e = cpu.r[1];
        cpu.set_t(e >> 31 != 0); // shll r3
        e <<= 1; // 2N
        e = e.wrapping_add(cpu.r[1]); // 3N
        e <<= 2; // shll2 -> 12N
        cpu.set_t(e >> 31 != 0); // shll r3
        e <<= 1; // 24N
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // [r4+0x80]
        e = e.wrapping_add(cpu.r[5]);
        cpu.r[3] = e;
        cpu.set_t(cpu.r[1] == e); // cmp/eq r1,r3
        if cpu.t() {
            let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[5]);
        }
        cpu.r[0] = 1; // rts delay slot: mov #1,r0
        return;
    }
}
