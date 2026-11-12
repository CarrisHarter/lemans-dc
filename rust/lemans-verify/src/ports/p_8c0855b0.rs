//! 0x8c0855b0 — TDLM "time from three shorts" helper.
//!
//! Each argument points at four sign-extended `short`s (a0,a1,a2,a3):
//! value = ((a3*60 + a2)*60 + a1)*1000 + a0 (32-bit wrap), then the routine
//! returns value(r4) / value(r5) as single precision in fr0. The last loads
//! leave r0/r5 holding the two values, r1/r7 the two `mul.l` products, r6 the
//! first a0, r4 the second b0, r3 the pool constant 1000, mach/macl the second
//! product's high/low words, fpul the divisor and fr1 that divisor as float. r13/r14 are
//! pushed and popped, so they end at their entry values; r15 nets unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let p2 = cpu.r[5];

    // mov.w @(6,r4),r0 ; exts.w r0,r7
    let a3 = mem.read_u16(p1.wrapping_add(6)).unwrap_or(0) as i16 as i32;
    // mov.w @r4,r6
    let a0 = mem.read_u16(p1).unwrap_or(0) as i16 as i32;
    // mov.w @(4,r4),r0 ; exts.w r0,r1
    let a2 = mem.read_u16(p1.wrapping_add(4)).unwrap_or(0) as i16 as i32;
    // mov.w @(2,r4),r0 ; exts.w r0,r14
    let a1 = mem.read_u16(p1.wrapping_add(2)).unwrap_or(0) as i16 as i32;

    // mov.w @(6,r5),r0 ; exts.w r0,r7
    let b3 = mem.read_u16(p2.wrapping_add(6)).unwrap_or(0) as i16 as i32;
    // mov.w @(4,r5),r0 ; exts.w r0,r0
    let b2 = mem.read_u16(p2.wrapping_add(4)).unwrap_or(0) as i16 as i32;
    // mov.w @(2,r5),r0 ; exts.w r0,r0
    let b1 = mem.read_u16(p2.wrapping_add(2)).unwrap_or(0) as i16 as i32;
    // mov.w @r5,r4 ; exts.w r4,r0
    let b0 = mem.read_u16(p2).unwrap_or(0) as i16 as i32;

    // mov.l @0x8c085624,r3 — literal 0x3e8 (1000).
    let k = mem.read_u32(0x8C08_5624).unwrap_or(0x3E8) as i32;

    // 0x8c0855be..: r13 = 60*a3 + a2 ; r1 = 60*r13 + a1.
    let hms_a = 60i32
        .wrapping_mul(60i32.wrapping_mul(a3).wrapping_add(a2))
        .wrapping_add(a1);
    let hms_b = 60i32
        .wrapping_mul(60i32.wrapping_mul(b3).wrapping_add(b2))
        .wrapping_add(b1);

    cpu.r[3] = k as u32;

    // 0x8c0855f8: mul.l r3,r1 ; 0x8c0855fe: sts macl,r1
    let prod_a = (k as i64) * (hms_a as i64);
    cpu.mach = (prod_a >> 32) as u32;
    cpu.macl = prod_a as u32;
    cpu.r[1] = cpu.macl;

    // 0x8c085606: mul.l r3,r7 ; 0x8c08560c: sts macl,r7
    let prod_b = (k as i64) * (hms_b as i64);
    cpu.mach = (prod_b >> 32) as u32;
    cpu.macl = prod_b as u32;
    cpu.r[7] = cpu.macl;

    // 0x8c085608: exts.w r6,r5 ; 0x8c08560e: add r1,r5
    cpu.r[6] = a0 as u32;
    cpu.r[5] = (a0 as u32).wrapping_add(cpu.r[1]);
    // 0x8c08560a: exts.w r4,r0 ; 0x8c085612: add r7,r0
    cpu.r[4] = b0 as u32;
    cpu.r[0] = (b0 as u32).wrapping_add(cpu.r[7]);

    // 0x8c085610: lds r5,fpul ; 0x8c085614: float fpul,fr0
    cpu.fpul = cpu.r[5];
    cpu.fr[0] = cpu.fpul as i32 as f32;
    // 0x8c085616: lds r0,fpul ; 0x8c08561a: float fpul,fr1 ;
    // rts delay 0x8c085620: fdiv fr1,fr0
    cpu.fpul = cpu.r[0];
    cpu.fr[1] = cpu.fpul as i32 as f32;
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1];
}
