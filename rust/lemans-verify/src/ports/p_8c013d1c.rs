//! 0x8c013d1c — exp-like float kernel with polynomial refinement + tail call.
//!
//! Classifies fr4 (sign/zero/Inf/NaN via bit tricks), takes early exits for
//! zero (fr0 = 1.0) and out-of-range exponents (±Inf/0), else scales fr4 by
//! 1/ln2, splits int/fraction via ftrc, runs two 6-tap fmac polynomial loops,
//! then tail-calls 0x8c014810 (fr4=denominator... precisely fr4/fr6 quotient
//! in the jsr delay slot, r4 = truncated exponent count) which returns fr0.

use sh4_oracle::{Cpu, Memory};


pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.r[3] = cpu.r[15];
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[4]);
    cpu.r[0] = 4;
    cpu.r[4] = 0;
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[4]);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
    cpu.r[6] = 8; // bt.s delay slot
    let t_pz = cpu.t();
    if !t_pz {
        cpu.r[4] = cpu.r[6];
    }
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.set_t(cpu.fr[4] == cpu.fr[3]); // fcmp/eq fr3,fr4
    cpu.r[5] = mem.read_u32(0x8c013e5c).unwrap_or(0); // 0x7f800000
    cpu.r[7] = 1; // bf.s delay slot
    let t_zero = cpu.t();
    if t_zero {
        cpu.r[2] = 4;
        cpu.r[4] |= cpu.r[2]; // bra delay slot: or r2,r4
        tail_classified(cpu, mem);
        return;
    }
    // fr4 != 0
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c013e60).unwrap_or(0); // 0x7fffffff
    cpu.r[1] &= cpu.r[3];
    cpu.set_t(cpu.r[1] == cpu.r[5]); // cmp/eq r5,r1
    if cpu.t() {
        cpu.r[1] = 2;
        cpu.r[4] |= cpu.r[1]; // bra delay slot
        tail_classified(cpu, mem);
        return;
    }
    cpu.set_t(cpu.fr[4] == cpu.fr[4]); // fcmp/eq fr4,fr4 (false for NaN)
    if !cpu.t() {
        cpu.r[4] |= cpu.r[7]; // or r7,r4 (r7 = 1)
    }
    tail_classified(cpu, mem);
}

fn tail_classified(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c013d5c
    cpu.r[0] = cpu.r[4];
    cpu.set_t(cpu.r[0] & cpu.r[7] == 0); // tst r0,r7
    if cpu.t() {
        // NaN path test first: bt 0x8c013d68
        nan_or_zero_test(cpu, mem);
        return;
    }
    cpu.r[3] = mem.read_u32(0x8c013e64).unwrap_or(0); // 0x8c1ccf98
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // bra delay = nop; target 0x8c013d94
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn nan_or_zero_test(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c013d68
    cpu.r[3] = 4;
    cpu.r[7] = mem.read_u32(0x8c013e68).unwrap_or(0); // 0x8c1ccfa4
    cpu.set_t(cpu.r[0] & cpu.r[3] == 0); // tst r0,r3
    if cpu.t() {
        exponent_test(cpu, mem);
        return;
    }
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap_or(0.0); // 1.0
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn exponent_test(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c013d7a
    cpu.r[3] = cpu.r[15];
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[3] = (-23i32) as u32;
    cpu.r[1] = 15;
    cpu.r[2] &= cpu.r[5];
    cpu.r[2] = ((cpu.r[2] as i32) >> 23) as u32; // shad r3,r2
    cpu.r[2] = cpu.r[2].wrapping_add((-127i32) as u32);
    cpu.set_t((cpu.r[2] as i32) >= (cpu.r[1] as i32)); // cmp/ge r1,r2
    if !cpu.t() {
        main_path(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] & cpu.r[6] == 0); // tst r0,r6
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c013e70).unwrap_or(0); // 0x8c1ccfa0
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // +Inf
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[3] = mem.read_u32(0x8c013e6c).unwrap_or(0); // 0x8c1ccf9c
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // +0.0
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn main_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c013daa
    cpu.r[3] = mem.read_u32(0x8c013e74).unwrap_or(0); // 0x8c1cd1b8
    cpu.r[0] = 8;
    cpu.fr[1] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[0] = 4;
    cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.r[4] = mem.read_u32(0x8c013e78).unwrap_or(0); // 0x80000000
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3]; // fmul fr3,fr1
    cpu.r[2] = mem.read_u32(0x8c013e7c).unwrap_or(0); // 0x4b000000
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[1]);
    cpu.r[6] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] &= cpu.r[6];
    cpu.r[6] &= cpu.r[5];
    cpu.set_t((cpu.r[6] as i32) >= (cpu.r[2] as i32)); // cmp/ge r2,r6
    if !cpu.t() {
        cpu.fpul = cpu.fr[1] as i32 as u32; // ftrc fr1,fpul
        cpu.r[0] = cpu.fpul; // sts fpul,r0
        cpu.fpul = cpu.r[0]; // lds r0,fpul
        cpu.r[0] = 4;
        cpu.fr[2] = cpu.fpul as i32 as f32; // float fpul,fr2
        let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[2]);
    }
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = 4;
    cpu.fr[3] = cpu.fr[1]; // fmov fr1,fr3
    cpu.r[3] |= cpu.r[4];
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[3]);
    cpu.r[3] = cpu.r[15];
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[3] - cpu.fr[0]; // fsub fr0,fr3
    cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc fr0,fpul
    let _ = mem.write_f32(cpu.r[15], cpu.fr[3]);
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[3] = (-23i32) as u32;
    cpu.r[5] &= cpu.r[2];
    cpu.r[5] = ((cpu.r[5] as i32) >> 23) as u32; // shad r3,r5
    cpu.r[2] = (-1i32) as u32;
    cpu.r[5] = cpu.r[5].wrapping_add((-127i32) as u32);
    cpu.set_t((cpu.r[5] as i32) >= (cpu.r[2] as i32)); // cmp/ge r2,r5
    cpu.r[4] = cpu.fpul; // bf.s delay slot: sts fpul,r4
    if !cpu.t() {
        poly_part(cpu, mem);
        return;
    }
    cpu.r[1] = cpu.r[15];
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c013e78).unwrap_or(0); // 0x80000000
    cpu.set_t(cpu.r[3] & cpu.r[0] == 0); // tst r3,r0
    if cpu.t() {
        cpu.r[1] = mem.read_u32(0x8c013e80).unwrap_or(0); // 0x8c1ccfd4
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
        cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        cpu.fr[2] = cpu.fr[2] + cpu.fr[3]; // fadd fr3,fr2
        let _ = mem.write_f32(cpu.r[15], cpu.fr[2]);
        poly_part(cpu, mem);
        return;
    }
    cpu.fr[2] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] + cpu.fr[3]; // fadd fr3,fr2
    let _ = mem.write_f32(cpu.r[15], cpu.fr[2]);
    cpu.r[4] = cpu.r[4].wrapping_sub(1); // bra delay slot: add #-1,r4
    poly_part(cpu, mem);
}

fn poly_part(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c013e1a
    cpu.r[3] = mem.read_u32(0x8c013e84).unwrap_or(0); // 0x8c1ccfc8
    cpu.r[7] = 6;
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[7];
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.r[6] = mem.read_u32(0x8c013e88).unwrap_or(0); // 0x8c1cd180
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3]; // fmul fr3,fr1
    cpu.fr[4] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
    cpu.r[6] = cpu.r[6].wrapping_add(4);
    loop {
        cpu.fr[3] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        cpu.r[6] = cpu.r[6].wrapping_add(4);
        cpu.r[5] = cpu.r[5].wrapping_sub(1);
        cpu.fr[0] = cpu.fr[1];
        cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
        cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[4], cpu.fr[3]); // fmac fr0,fr4,fr3
        cpu.fr[4] = cpu.fr[3]; // bt.s delay slot: always runs
        if !cpu.t() {
            break;
        }
    }
    cpu.r[6] = mem.read_u32(0x8c013e8c).unwrap_or(0); // 0x8c1cd19c
    cpu.r[5] = cpu.r[7];
    cpu.fr[5] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
    cpu.r[6] = cpu.r[6].wrapping_add(4);
    loop {
        cpu.fr[3] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        cpu.r[6] = cpu.r[6].wrapping_add(4);
        cpu.r[5] = cpu.r[5].wrapping_sub(1);
        cpu.fr[0] = cpu.fr[1];
        cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
        cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[5], cpu.fr[3]); // fmac fr0,fr5,fr3
        cpu.fr[5] = cpu.fr[3]; // bt.s delay slot: always runs
        if !cpu.t() {
            break;
        }
    }
    cpu.r[3] = mem.read_u32(0x8c013e90).unwrap_or(0); // 0x8c014810
    cpu.fr[6] = cpu.fr[5]; // fmov fr5,fr6
    cpu.fr[4] = cpu.fr[4] / cpu.fr[6]; // jsr delay slot: fdiv fr6,fr4
    cpu.pr = 0x8c013e54;
    crate::registry::p_8c014810::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
