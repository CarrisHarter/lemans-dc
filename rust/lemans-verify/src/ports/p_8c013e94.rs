//! 0x8c013e94 — classify the float in fr4 and evaluate the positive branch:
//! negative/NaN → fr0 = NaN (0x7f800001), zero → fr0 = f32[0x8c1ccfd0],
//! +Inf → fr0 = +Inf, 1.0 → fr0 = 0.0, otherwise a 7-step Horner loop in fr5^2
//! over the table at 0x8c1cd1c8.
//!
//! Flag bits accumulated in r4: 8 = sign (cmp/pz of the raw bits), 4 = zero
//! (fcmp/eq against fldi0), 2 = Inf (|bits| == 0x7f800000), 1 = NaN
//! (fcmp/eq fr4,fr4). The dispatch tests them with tst r0,r7/r3/r5/r1.
//!
//! Hidden side effects the C does not show: fr4 is overwritten with the NaN
//! constant by the *delay slot* of `bf.s` at 0x8c013ed8 on every path,
//! fr3 = 0.0 from fldi0 is left in fr3 on early paths, r5 = 8 / r7 = 1
//! constants stay in the registers, fpul = 128 on the normal path,
//! r5 ends at 0x8c1cd1e8 (post-increment loop), r0 ends as the raw exponent
//! field (orig & 0x7f800000), r1 = 0x8c1cd1c4, r2 = 0x8c1ccfc8, r3 = 0x8c1cd1c0,
//! T ends as the final cmp/pl r4 (= 0) on the normal path, and the rts delay
//! slot restores r15 += 12 on every path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // add #-12,r15
    // fmov fr4,@r3 ([r15+8]) and fmov fr4,@(r0,r15) ([r15+4]) are stack-only;
    // the values are tracked locally.
    let orig = cpu.fr[4].to_bits();
    cpu.r[0] = 4; // mov #4,r0
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[3] = orig; // mov.l @(4,r15),r3
    cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
    cpu.r[5] = 8; // bt.s delay slot: mov #8,r5
    if !cpu.t() {
        cpu.r[4] = cpu.r[5]; // mov r5,r4
    }
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.set_t(cpu.fr[4] == cpu.fr[3]); // fcmp/eq fr3,fr4: T = (fr4 == 0.0)
    cpu.r[6] = mem.read_u32(0x8c013f88).unwrap(); // 0x7f800000
    cpu.r[7] = 1; // bf.s delay slot: mov #1,r7
    if cpu.t() {
        cpu.r[2] = 4; // mov #4,r2
        cpu.r[4] |= cpu.r[2]; // bra delay slot: or r2,r4
    } else {
        cpu.r[1] = orig; // mov.l @(4,r15),r1
        cpu.r[3] = mem.read_u32(0x8c013f8c).unwrap(); // 0x7fffffff
        cpu.r[1] &= cpu.r[3]; // and r3,r1
        cpu.set_t(cpu.r[1] == cpu.r[6]); // cmp/eq r6,r1
        if cpu.t() {
            cpu.r[1] = 2; // mov #2,r1
            cpu.r[4] |= cpu.r[1]; // bra delay slot: or r1,r4
        } else {
            cpu.set_t(cpu.fr[4] == cpu.fr[4]); // fcmp/eq fr4,fr4
            if !cpu.t() {
                cpu.r[4] |= cpu.r[7]; // or r7,r4 (NaN)
            }
        }
    }
    cpu.r[2] = mem.read_u32(0x8c013f90).unwrap(); // 0x8c1ccf98
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    cpu.set_t(cpu.r[0] & cpu.r[7] == 0); // tst r0,r7
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap(); // bf.s delay slot: fmov @r2,fr4 = NaN
    if !cpu.t() {
        cpu.fr[0] = cpu.fr[4]; // NaN path: fmov fr4,fr0 ; rts
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        return;
    }
    cpu.r[3] = 4; // mov #4,r3
    cpu.set_t(cpu.r[0] & cpu.r[3] == 0); // tst r0,r3
    if !cpu.t() {
        // zero: r0 = 0x8c1ccfd0 ; fr0 = f32[r0] ; rts
        cpu.r[0] = mem.read_u32(0x8c013f94).unwrap();
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        return;
    }
    cpu.set_t(cpu.r[0] & cpu.r[5] == 0); // tst r0,r5
    if !cpu.t() {
        // negative: fr0 = fr4 (NaN) ; rts
        cpu.fr[0] = cpu.fr[4];
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        return;
    }
    cpu.r[1] = 2; // mov #2,r1
    cpu.set_t(cpu.r[0] & cpu.r[1] == 0); // tst r0,r1
    if !cpu.t() {
        // +Inf: r0 = 0x8c1ccfa0 ; fr0 = f32[r0] ; rts
        cpu.r[0] = mem.read_u32(0x8c013f98).unwrap();
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        return;
    }

    // ---- normal path, 0x8c013f00 ----
    cpu.r[3] = mem.read_u32(0x8c013f9c).unwrap(); // 0x8c1ccfa4
    cpu.r[0] = 8; // mov #8,r0
    cpu.fr[2] = f32::from_bits(orig); // fmov @(r0,r15),fr2 — [r15+8] = orig fr4
    cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr3 = 1.0
    cpu.set_t(cpu.fr[2] == cpu.fr[3]); // fcmp/eq fr3,fr2
    if cpu.t() {
        // input == 1.0: fr0 = 0.0 ; rts
        cpu.r[1] = mem.read_u32(0x8c013fa0).unwrap(); // 0x8c1ccf9c
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        return;
    }
    // 0x8c013f14: normalize mantissa into [1,2) via [r15] round trip
    let masked = orig & 0x807fffff; // r2 = 0x807fffff ; r1 = orig & r2
    cpu.r[2] = mem.read_u32(0x8c013fb0).unwrap(); // 0x8c1ccfc8
    let norm = masked | 0x3f800000; // or r1,r0 with r1 = 0x3f800000
    cpu.r[1] = mem.read_u32(0x8c013fb4).unwrap(); // 0x8c1cd1c4
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap(); // fmov @r2,fr1 = ln2
    cpu.r[0] = orig; // mov.l @r3,r0 — [r15+8] = orig
    cpu.r[3] = (-23i32) as u32; // mov #-23,r3
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0 = -sqrt2
    cpu.r[6] &= cpu.r[0]; // and r0,r6 — r6 = exponent field, r0 keeps orig bits
    cpu.r[6] = ((cpu.r[6] as i32) >> 23) as u32; // shad r3,r6 → unbiased exponent + 127
    let half_ptr = mem.read_u32(0x8c013fac).unwrap(); // 0x8c1ccfa8
    cpu.r[6] = cpu.r[6].wrapping_add((-127i32) as u32); // add #-127,r6 → 128
    cpu.fpul = cpu.r[6]; // lds r6,fpul
    cpu.fr[2] = mem.read_f32(half_ptr).unwrap(); // fmov @r3,fr2 = 0.5
    cpu.fr[3] = cpu.fpul as i32 as f32; // float fpul,fr3
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // fadd fr2,fr3
    cpu.fr[7] = cpu.fr[3]; // fmov fr3,fr7
    cpu.fr[7] = cpu.fr[7] * cpu.fr[1]; // fmul fr1,fr7
    cpu.fr[3] = f32::from_bits(norm); // fmov @r15,fr3 — [r15] = norm
    cpu.r[3] = mem.read_u32(0x8c013fb8).unwrap(); // 0x8c1cd1c0
    cpu.fr[0] = cpu.fr[0] + cpu.fr[3]; // fadd fr3,fr0
    cpu.r[5] = mem.read_u32(0x8c013fbc).unwrap(); // 0x8c1cd1c8
    cpu.r[4] = 7; // mov #7,r4
    cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr2 = +sqrt2
    cpu.fr[4] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5+,fr4
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[2] = cpu.fr[2] + cpu.fr[3]; // fadd fr3,fr2
    cpu.fr[5] = cpu.fr[0]; // fmov fr0,fr5
    cpu.fr[5] = cpu.fr[5] / cpu.fr[2]; // fdiv fr2,fr5
    cpu.fr[6] = cpu.fr[5]; // fmov fr5,fr6
    cpu.fr[6] = cpu.fr[6] * cpu.fr[5]; // fmul fr5,fr6
    loop {
        cpu.fr[3] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5+,fr3
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        cpu.r[4] = cpu.r[4].wrapping_sub(1); // add #-1,r4
        cpu.fr[0] = cpu.fr[6]; // fmov fr6,fr0
        cpu.set_t((cpu.r[4] as i32) > 0); // cmp/pl r4
        cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[4], cpu.fr[3]); // fmac fr0,fr4,fr3
        cpu.fr[4] = cpu.fr[3]; // bt.s delay slot: fmov fr3,fr4
        if !cpu.t() {
            break;
        }
    }
    cpu.fr[6] = cpu.fr[4]; // fmov fr4,fr6
    cpu.fr[4] = cpu.fr[5]; // fmov fr5,fr4
    cpu.fr[4] = cpu.fr[4] * cpu.fr[6]; // fmul fr6,fr4
    cpu.fr[0] = cpu.fr[7]; // fmov fr7,fr0
    cpu.fr[0] = cpu.fr[0] + cpu.fr[4]; // fadd fr4,fr0
    cpu.r[15] = cpu.r[15].wrapping_add(12); // rts delay slot: add #12,r15
}
