//! 0x8c01412c — sin-like kernel: classify fr4, reduce mod π/2 via 0x8c01471c,
//! dispatch on the octant flags to the 0x8c011d3c/0x8c011d1c polynomial pair.
//!
//! Classification mirrors 0x8c013d1c (sign/zero/Inf/NaN bit tricks into r4).
//! r14 ends 0 or 1 (negated-input flag). NaN/Inf exit early with fr0 = NaN.
//! r0 > π/4 exits with fr0 = 0. The octant switch (r0&7 vs r13=1, then
//! cmp/eq 0..7) selects which polynomial result lands in fr5, with fneg
//! fixups; final `tst r14,r14` negates again for negative inputs.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[0] = 4;
    cpu.fr[5] = cpu.fr[4];
    cpu.r[6] = 0;
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[5]);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = cpu.r[6];
    cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
    cpu.r[5] = 8; // bt.s delay slot
    if !cpu.t() {
        cpu.r[4] = cpu.r[5];
    }
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.set_t(cpu.fr[5] == cpu.fr[3]); // fcmp/eq fr3,fr5
    cpu.r[13] = 1; // bf.s delay slot
    if cpu.t() {
        cpu.r[2] = 4;
        cpu.r[4] |= cpu.r[2]; // bra delay slot
        classified(cpu, mem);
        return;
    }
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c01421c).unwrap_or(0); // 0x7fffffff
    cpu.r[2] = mem.read_u32(0x8c014220).unwrap_or(0); // 0x7f800000
    cpu.r[1] &= cpu.r[3];
    cpu.set_t(cpu.r[1] == cpu.r[2]); // cmp/eq r2,r1
    if cpu.t() {
        cpu.r[0] = 2;
        cpu.r[4] |= cpu.r[0]; // bra delay slot
        classified(cpu, mem);
        return;
    }
    cpu.set_t(cpu.fr[5] == cpu.fr[5]); // NaN -> false
    if !cpu.t() {
        cpu.r[4] |= cpu.r[13];
    }
    classified(cpu, mem);
}

fn classified(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01416e
    cpu.r[2] = 3;
    cpu.r[14] = cpu.r[4];
    cpu.set_t(cpu.r[14] & cpu.r[2] == 0); // tst r14,r2
    if !cpu.t() {
        cpu.r[1] = mem.read_u32(0x8c014224).unwrap_or(0); // 0x8c1ccf98
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // NaN
        epilogue(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[14] & cpu.r[5] == 0); // tst r14,r5 (r5 = 8)
    if !cpu.t() {
        cpu.fr[4] = -cpu.fr[4]; // fneg fr4
        cpu.r[14] = cpu.r[13]; // bra delay slot: mov r13,r14 (r13 = 1)
    } else {
        cpu.r[14] = cpu.r[6]; // mov r6,r14 (0)
    }
    reduce(cpu, mem);
}

fn reduce(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01418a
    cpu.r[2] = mem.read_u32(0x8c01422c).unwrap_or(0); // 0x8c01471c
    cpu.r[4] = cpu.r[15];
    cpu.r[3] = mem.read_u32(0x8c014228).unwrap_or(0); // 0x8c1ccfc4
    cpu.fr[5] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // jsr delay slot
    cpu.pr = 0x8c014194;
    crate::registry::p_8c01471c::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c014228).unwrap_or(0); // 0x8c1ccfc4
    cpu.fr[4] = cpu.fr[0];
    cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[4] > cpu.fr[3]); // fcmp/gt fr3,fr4
    if !cpu.t() {
        dispatch(cpu, mem);
        return;
    }
    cpu.r[1] = mem.read_u32(0x8c014230).unwrap_or(0); // 0x8c1ccf9c
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // 0.0
    epilogue(cpu, mem);
}

fn dispatch(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0141a6
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] &= 7;
    cpu.set_t(cpu.r[13] & cpu.r[0] == 0); // tst r13,r0
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // bt.s delay slot: always runs
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c014234).unwrap_or(0); // 0x8c1ccff4
        cpu.fr[3] = cpu.fr[4];
        cpu.fr[4] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        cpu.fr[4] = cpu.fr[4] + cpu.fr[3]; // fadd fr3,fr4
    }
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0); // cmp/eq #0,r0
    if cpu.t() {
        octant_0_7(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 7); // cmp/eq #7,r0
    if cpu.t() {
        octant_0_7(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.t() {
        octant_1_2(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
    if cpu.t() {
        octant_1_2(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
    if cpu.t() {
        octant_3_4(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0
    if cpu.t() {
        octant_3_4(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 5); // cmp/eq #5,r0
    if cpu.t() {
        octant_5_6(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 6); // cmp/eq #6,r0
    if cpu.t() {
        octant_5_6(cpu, mem);
        return;
    }
    finish(cpu, mem);
}

fn octant_0_7(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0141de: jsr 0x8c011d3c; bra 0x8c0141ee (nop delay); bra 0x8c01420a, delay fmov fr0,fr5
    cpu.r[3] = mem.read_u32(0x8c014238).unwrap_or(0);
    cpu.pr = 0x8c0141e4;
    crate::registry::p_8c011d3c::run(cpu, mem);
    cpu.fr[5] = cpu.fr[0]; // bra delay slot at 0x8c0141f0
    finish(cpu, mem);
}

fn octant_1_2(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0141e8: jsr 0x8c011d1c; bra 0x8c01420a, delay fmov fr0,fr5
    cpu.r[3] = mem.read_u32(0x8c01423c).unwrap_or(0);
    cpu.pr = 0x8c0141ec;
    crate::registry::p_8c011d1c::run(cpu, mem);
    cpu.fr[5] = cpu.fr[0];
    finish(cpu, mem);
}

fn octant_3_4(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0141f2: jsr 0x8c011d3c; fmov fr0,fr4; bra 0x8c014208, delay fmov fr0,fr5; fneg fr5
    cpu.r[3] = mem.read_u32(0x8c014238).unwrap_or(0);
    cpu.pr = 0x8c0141f6;
    crate::registry::p_8c011d3c::run(cpu, mem);
    cpu.fr[4] = cpu.fr[0];
    cpu.fr[5] = cpu.fr[0]; // bra delay slot
    cpu.fr[5] = -cpu.fr[5]; // fneg fr5 at 0x8c014208
    finish(cpu, mem);
}

fn octant_5_6(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0141fe: jsr 0x8c011d1c; fmov fr0,fr5; fmov fr0,fr4; bra 0x8c014208: fneg fr5
    cpu.r[3] = mem.read_u32(0x8c01423c).unwrap_or(0);
    cpu.pr = 0x8c014202;
    crate::registry::p_8c011d1c::run(cpu, mem);
    cpu.fr[5] = cpu.fr[0];
    cpu.fr[4] = cpu.fr[0];
    cpu.fr[5] = -cpu.fr[5];
    finish(cpu, mem);
}

fn finish(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01420a
    cpu.set_t(cpu.r[14] & cpu.r[14] == 0); // tst r14,r14
    if !cpu.t() {
        cpu.fr[5] = -cpu.fr[5]; // fneg fr5
    }
    cpu.fr[0] = cpu.fr[5];
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014212
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
