//! 0x8c014480 — atan-like kernel: classify fr4, reduce via 0x8c01471c,
//! dispatch on the reduction index to argument setup, call 0x8c014810 and/or
//! 0x8c011cf8, then add the fr14 correction.
//!
//! r13 is the outer sign flag (nonzero only for negative inputs on some
//! paths); fr12-fr15 are callee-saved through the stack. Early exits: NaN
//! (fr0 = NaN), zero (falls into the index switch with fr4 = 0), |x| huge
//! (fr0 = ±π/2), x == ±1 (fr0 = ±π/4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.r[0] = 8;
    cpu.fr[5] = cpu.fr[4];
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[5]);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
    cpu.r[4] = 0; // bt.s delay slot
    if !cpu.t() {
        cpu.r[4] = 8;
    }
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.set_t(cpu.fr[5] == cpu.fr[3]); // fcmp/eq fr3,fr5
    cpu.r[7] = 4;
    cpu.r[6] = 1; // bf.s delay slot
    if cpu.t() {
        cpu.r[4] |= cpu.r[7]; // bra delay slot
        sign_fix(cpu, mem);
        return;
    }
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c014614).unwrap_or(0); // 0x7fffffff
    cpu.r[1] = mem.read_u32(0x8c014618).unwrap_or(0); // 0x7f800000
    cpu.r[2] &= cpu.r[3];
    cpu.set_t(cpu.r[2] == cpu.r[1]); // cmp/eq r1,r2
    if cpu.t() {
        cpu.r[0] = 2;
        cpu.r[4] |= cpu.r[0]; // bra delay slot
        sign_fix(cpu, mem);
        return;
    }
    cpu.set_t(cpu.fr[5] == cpu.fr[5]); // NaN -> false
    if !cpu.t() {
        cpu.r[4] |= cpu.r[6];
    }
    sign_fix(cpu, mem);
}

fn sign_fix(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0144c6: abs the stack copy of fr4; r5 = old r4 class flags
    cpu.r[3] = cpu.r[15];
    cpu.r[2] = mem.read_u32(0x8c01461c).unwrap_or(0); // 0x80000000
    let _ = mem.write_f32(cpu.r[15], cpu.fr[4]);
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.set_t(cpu.r[2] & cpu.r[1] == 0); // tst r2,r1
    cpu.r[5] = cpu.r[4]; // bt.s delay slot: always runs
    if !cpu.t() {
        cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
        cpu.fr[3] = -cpu.fr[3];
        let _ = mem.write_f32(cpu.r[15], cpu.fr[3]);
    }
    nan_test(cpu, mem);
}

fn nan_test(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0144da
    cpu.set_t(cpu.r[4] & cpu.r[6] == 0); // tst r4,r6 (r6 = 1)
    if !cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c014620).unwrap_or(0); // 0x8c1ccf98
        cpu.fr[5] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // bra delay slot: NaN
        finish(cpu, mem);
        return;
    }
    range_test(cpu, mem);
}

fn range_test(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0144e4
    cpu.r[2] = cpu.r[15];
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c014618).unwrap_or(0); // 0x7f800000
    cpu.r[2] = (-23i32) as u32;
    cpu.r[0] = 32;
    cpu.r[1] &= cpu.r[3];
    cpu.r[1] = ((cpu.r[1] as i32) >> 23) as u32; // shad r2,r1
    cpu.r[1] = cpu.r[1].wrapping_add((-127i32) as u32);
    cpu.set_t((cpu.r[1] as i32) >= (cpu.r[0] as i32)); // cmp/ge r0,r1
    cpu.r[13] = 8; // bf.s delay slot: always runs
    cpu.r[13] &= cpu.r[4];
    if cpu.t() {
        huge(cpu, mem);
        return;
    }
    zero_test(cpu, mem);
}

fn huge(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0144fc: |x| huge -> fr5 = ±π/2 by sign flag
    cpu.r[1] = mem.read_u32(0x8c014624).unwrap_or(0); // 0x8c1ccfc0
    cpu.set_t(cpu.r[13] & cpu.r[13] == 0); // tst r13,r13
    cpu.fr[5] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // bf.s delay slot: π/2
    if !cpu.t() {
        cpu.fr[5] = -cpu.fr[5]; // bra delay slot at 0x8c01450a
    }
    finish(cpu, mem);
}

fn zero_test(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01450c
    cpu.set_t(cpu.r[7] & cpu.r[4] == 0); // tst r7,r4 (r7 = 4)
    if cpu.t() {
        one_test(cpu, mem);
        return;
    }
    cpu.fr[5] = cpu.fr[4]; // bra delay slot
    finish(cpu, mem);
}

fn one_test(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014514
    cpu.r[1] = mem.read_u32(0x8c014628).unwrap_or(0); // 0x8c1ccfa4
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.fr[5] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // 1.0
    cpu.r[3] = mem.read_u32(0x8c01462c).unwrap_or(0); // 0x8c1ccfc4
    cpu.set_t(cpu.fr[5] > cpu.fr[3]); // fcmp/gt fr3,fr5
    cpu.fr[4] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // bf.s delay slot: π/4
    if cpu.t() {
        // |x| < 1
        cpu.r[2] = mem.read_u32(0x8c014630).unwrap_or(0); // 0x8c1ccf9c
        cpu.fr[14] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // 0.0
        cpu.fr[15] = cpu.fr[3]; // bra delay slot
        reduce(cpu, mem);
        return;
    }
    cpu.set_t(cpu.fr[5] == cpu.fr[3]); // fcmp/eq fr3,fr5
    if cpu.t() {
        // |x| == 1 -> fr0 = ±π/4 by sign flag
        cpu.set_t(cpu.r[13] & cpu.r[13] == 0); // tst r13,r13
        if !cpu.t() {
            cpu.fr[4] = -cpu.fr[4];
        }
        cpu.fr[0] = cpu.fr[4]; // bra delay slot
        epilogue(cpu, mem);
        return;
    }
    // |x| > 1: fr5 = 1+|x|, fr14 = π/4, fr15 = (-1+|x|)/(1+|x|)
    cpu.r[2] = mem.read_u32(0x8c014634).unwrap_or(0); // 0x8c1ccfd4
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // -1.0
    cpu.fr[5] = cpu.fr[5] + cpu.fr[3]; // fadd fr3,fr5
    cpu.fr[14] = cpu.fr[4];
    cpu.fr[2] = cpu.fr[2] + cpu.fr[3]; // fadd fr3,fr2
    cpu.fr[15] = cpu.fr[2];
    cpu.fr[15] = cpu.fr[15] / cpu.fr[5]; // fdiv fr5,fr15
    reduce(cpu, mem);
}

fn reduce(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014548
    cpu.r[3] = mem.read_u32(0x8c014638).unwrap_or(0); // 0x8c1cd284
    cpu.r[2] = mem.read_u32(0x8c01463c).unwrap_or(0); // 0x8c01471c
    cpu.r[4] = cpu.r[15]; // mov r15,r4 (was missing: r4 held class flags)
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[5] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[4] = cpu.fr[15]; // jsr delay slot
    cpu.pr = 0x8c014556;
    crate::registry::p_8c01471c::run(cpu, mem);
    dispatch(cpu, mem);
}

fn dispatch(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014556
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c014628).unwrap_or(0); // 0x8c1ccfa4
    cpu.r[14] = mem.read_u32(0x8c014640).unwrap_or(0); // 0x8c011cf8
    cpu.set_t(cpu.r[0] == 0); // cmp/eq #0,r0
    cpu.fr[13] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // bt.s delay slot: 1.0
    if cpu.t() {
        cpu.fr[4] = cpu.fr[15]; // bra delay slot
        call_poly(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 1);
    if cpu.t() {
        idx_1_2(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 2);
    if cpu.t() {
        idx_1_2(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 3);
    if cpu.t() {
        idx_3_4(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 4);
    if cpu.t() {
        idx_3_4(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 5);
    if cpu.t() {
        idx_5_6(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 6);
    if cpu.t() {
        idx_5_6(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 7);
    if cpu.t() {
        idx_7_8(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 8);
    if cpu.t() {
        idx_7_8(cpu, mem);
        return;
    }
    tail(cpu, mem);
}

fn idx_1_2(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01458a: r4 = -2, fr12 = fr15-0.25, fr14 += 0.24497866, jsr 0x8c014810
    cpu.r[1] = mem.read_u32(0x8c014648).unwrap_or(0);
    cpu.r[4] = (-2i32) as u32;
    cpu.r[2] = mem.read_u32(0x8c014644).unwrap_or(0);
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[3] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] + cpu.fr[15]; // fadd fr15,fr2
    cpu.r[3] = mem.read_u32(0x8c01464c).unwrap_or(0); // 0x8c014810
    cpu.fr[14] = cpu.fr[14] + cpu.fr[3]; // fadd fr3,fr14
    cpu.fr[12] = cpu.fr[2];
    cpu.fr[4] = cpu.fr[15]; // jsr delay slot
    cpu.pr = 0x8c0145a0;
    crate::registry::p_8c014810::run(cpu, mem);
    cpu.fr[13] = cpu.fr[13] + cpu.fr[0]; // fadd fr0,fr13
    cpu.fr[12] = cpu.fr[12] / cpu.fr[13]; // fdiv fr13,fr12
    cpu.fr[4] = cpu.fr[12]; // bra delay slot
    call_poly(cpu, mem);
}

fn idx_3_4(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0145a8: r4 = -1, fr12 = fr15-0.5, fr14 += 0.4636476, jsr 0x8c014810
    cpu.r[1] = mem.read_u32(0x8c014654).unwrap_or(0);
    cpu.r[4] = (-1i32) as u32;
    cpu.r[2] = mem.read_u32(0x8c014650).unwrap_or(0);
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[3] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] + cpu.fr[15];
    cpu.r[3] = mem.read_u32(0x8c01464c).unwrap_or(0);
    cpu.fr[14] = cpu.fr[14] + cpu.fr[3];
    cpu.fr[12] = cpu.fr[2];
    cpu.fr[4] = cpu.fr[15]; // jsr delay slot
    cpu.pr = 0x8c0145be;
    crate::registry::p_8c014810::run(cpu, mem);
    cpu.fr[13] = cpu.fr[13] + cpu.fr[0];
    cpu.fr[12] = cpu.fr[12] / cpu.fr[13];
    cpu.fr[4] = cpu.fr[12]; // bra delay slot
    call_poly(cpu, mem);
}

fn idx_5_6(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0145c6: no jsr; fr1 = fr15-0.75, fr13 += fr0*fr15 (fr0 = reduce residue), fr4 = fr1/fr13
    cpu.r[1] = mem.read_u32(0x8c01465c).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c014660).unwrap_or(0);
    cpu.r[2] = mem.read_u32(0x8c014658).unwrap_or(0);
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[15];
    cpu.fr[1] = cpu.fr[1] + cpu.fr[2]; // fadd fr2,fr1
    cpu.fr[3] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.fr[13] = cpu.fr[13] + cpu.fr[0] * cpu.fr[15]; // fmac fr0,fr15,fr13
    cpu.fr[14] = cpu.fr[14] + cpu.fr[3]; // fadd fr3,fr14
    cpu.fr[4] = cpu.fr[1];
    cpu.fr[4] = cpu.fr[4] / cpu.fr[13]; // bra delay slot: fdiv fr13,fr4
    call_poly(cpu, mem);
}

fn idx_7_8(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0145e0: no jsr; fr13 += fr15, fr2 = fr15-1, fr4 = fr2/fr13
    cpu.r[1] = mem.read_u32(0x8c014634).unwrap_or(0);
    cpu.fr[13] = cpu.fr[13] + cpu.fr[15]; // fadd fr15,fr13
    cpu.r[2] = mem.read_u32(0x8c01462c).unwrap_or(0);
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[3] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] + cpu.fr[15];
    cpu.fr[14] = cpu.fr[14] + cpu.fr[3];
    cpu.fr[4] = cpu.fr[2];
    cpu.fr[4] = cpu.fr[4] / cpu.fr[13]; // fdiv fr13,fr4
    call_poly(cpu, mem);
}

fn call_poly(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0145f2: jsr @r14 (0x8c011cf8); fr5 = fr14 + fr0
    cpu.pr = 0x8c0145f6;
    crate::registry::p_8c011cf8::run(cpu, mem);
    cpu.fr[5] = cpu.fr[14];
    cpu.fr[5] = cpu.fr[5] + cpu.fr[0]; // fadd fr0,fr5
    tail(cpu, mem);
}

fn tail(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0145fa
    cpu.set_t(cpu.r[13] & cpu.r[13] == 0); // tst r13,r13
    if !cpu.t() {
        cpu.fr[5] = -cpu.fr[5];
    }
    finish(cpu, mem);
}

fn finish(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014600
    cpu.fr[0] = cpu.fr[5];
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014602
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[12] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
