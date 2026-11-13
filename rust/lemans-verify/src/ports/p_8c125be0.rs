//! 0x8c125be0 — lerp packed 5-6-5 halfwords into vec4s.
//!
//! `[r4+88]` is a halfword pool. `r5` and `r6` are the A and B indices; both
//! are shifted left once (`r6` stays doubled). Each of the `[r4+72]` iterations
//! unpacks one pair (high 5 / mid 6 / low 5), divides by 31 and 63, and lerps
//! by `fr4`. The w component is the constant 1. `0x8c059054(r4, i)` returns a
//! record; the vec4 is stored at `[record+12]+32`. Entry `fr5` is written to
//! `[r4+92]`. `r11`–`r14` and `fr13`–`fr15` are restored. A zero count still
//! doubles `r5`/`r6`, leaves `fr0` = 1, and does not call.

use sh4_oracle::{Cpu, Memory};

fn push_r(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn push_f(cpu: &mut Cpu, mem: &mut Memory, value: f32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], value);
}

fn pop_u32(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

fn pop_f32(cpu: &mut Cpu, mem: &Memory) -> f32 {
    let v = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

fn shll(cpu: &mut Cpu, n: usize) {
    let t = cpu.r[n] >> 31 != 0;
    cpu.r[n] <<= 1;
    cpu.set_t(t);
}

fn shar(cpu: &mut Cpu, n: usize) {
    let t = cpu.r[n] & 1 != 0;
    cpu.r[n] = ((cpu.r[n] as i32) >> 1) as u32;
    cpu.set_t(t);
}

/// Same negative-amount masking as the oracle's `shad`.
fn shad(cpu: &mut Cpu, m: usize, n: usize) {
    let amount = cpu.r[m] as i32;
    if amount >= 0 {
        cpu.r[n] <<= (amount as u32) & 31;
    } else {
        cpu.r[n] = ((cpu.r[n] as i32) >> ((-amount) as u32 & 31)) as u32;
    }
}

fn extu_w(v: u32) -> u32 {
    v as u16 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c125be0: fmov fr15,@-r15
    push_f(cpu, mem, cpu.fr[15]);
    // 0x8c125be2: fmov fr14,@-r15
    push_f(cpu, mem, cpu.fr[14]);
    // 0x8c125be4: fmov fr13,@-r15
    push_f(cpu, mem, cpu.fr[13]);
    // 0x8c125be6: fmov fr5,fr14
    cpu.fr[14] = cpu.fr[5];
    // 0x8c125be8: mov.l r14,@-r15
    push_r(cpu, mem, cpu.r[14]);
    // 0x8c125bea: fmov fr4,fr13
    cpu.fr[13] = cpu.fr[4];
    // 0x8c125bec: mov.l r13,@-r15
    push_r(cpu, mem, cpu.r[13]);
    // 0x8c125bee: mov.l r12,@-r15
    push_r(cpu, mem, cpu.r[12]);
    // 0x8c125bf0: mov.l r11,@-r15
    push_r(cpu, mem, cpu.r[11]);
    // 0x8c125bf2: sts.l pr,@-r15
    push_r(cpu, mem, cpu.pr);
    // 0x8c125bf4: add #-16,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    // 0x8c125bf6: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c125bf8: mov r11,r0 / add #88,r0 / mov.l @r0,r13
    cpu.r[0] = cpu.r[11].wrapping_add(88);
    cpu.r[13] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c125bfe: fldi0 fr0
    cpu.fr[0] = 0.0;
    // 0x8c125c00: fmov fr13,fr15
    cpu.fr[15] = cpu.fr[13];
    // 0x8c125c02: fmul fr0,fr15
    cpu.fr[15] *= cpu.fr[0];
    // 0x8c125c04: fldi1 fr0
    cpu.fr[0] = 1.0;
    // 0x8c125c06: mov r13,r14
    cpu.r[14] = cpu.r[13];
    // 0x8c125c08: shll r5
    shll(cpu, 5);
    // 0x8c125c0a: shll r6
    shll(cpu, 6);
    // 0x8c125c0c: add r5,r14
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[5]);
    // 0x8c125c0e: add r6,r13
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[6]);
    // 0x8c125c10: fadd fr0,fr15
    cpu.fr[15] += cpu.fr[0];
    // 0x8c125c12: bra 0x8c125ccc ; delay mov #0,r12
    cpu.r[12] = 0;

    loop {
        // 0x8c125ccc: mov r11,r0 / add #72,r0 / mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(72)).unwrap_or(0);
        // 0x8c125cd2: cmp/gt r12,r0  (T = r0 > r12, signed)
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[12] as i32));
        // 0x8c125cd4: bt 0x8c125c16
        if !cpu.t() {
            break;
        }

        // 0x8c125c16: mov.w @r14,r0
        cpu.r[0] = mem.read_u16(cpu.r[14]).unwrap_or(0) as i16 as i32 as u32;
        // 0x8c125c18: mov #-11,r1
        cpu.r[1] = (-11i32) as u32;
        // 0x8c125c1a: mov.w @r13,r3
        cpu.r[3] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32;
        // 0x8c125c1c: extu.w r0,r5
        cpu.r[5] = extu_w(cpu.r[0]);
        // 0x8c125c1e: mov r5,r0
        cpu.r[0] = cpu.r[5];
        // 0x8c125c20: and #31,r0
        cpu.r[0] &= 31;
        // 0x8c125c22: lds r0,fpul
        cpu.fpul = cpu.r[0];
        // 0x8c125c24: extu.w r3,r4
        cpu.r[4] = extu_w(cpu.r[3]);
        // 0x8c125c26: mov r4,r3
        cpu.r[3] = cpu.r[4];
        // 0x8c125c28: mova 0x8c125cf0,r0 / fmov @r0,fr6
        cpu.r[0] = 0x8c125cf0;
        cpu.fr[6] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c125c2c: float fpul,fr7
        cpu.fr[7] = cpu.fpul as i32 as f32;
        // 0x8c125c2e: mov r5,r0
        cpu.r[0] = cpu.r[5];
        // 0x8c125c30..38: shar r0 ×5
        for _ in 0..5 {
            shar(cpu, 0);
        }
        // 0x8c125c3a: and #63,r0
        cpu.r[0] &= 63;
        // 0x8c125c3c: lds r0,fpul
        cpu.fpul = cpu.r[0];
        // 0x8c125c3e: shad r1,r5
        shad(cpu, 1, 5);
        // 0x8c125c40: shad r1,r3
        shad(cpu, 1, 3);
        // 0x8c125c42: mova 0x8c125cf4,r0 / fmov @r0,fr4
        cpu.r[0] = 0x8c125cf4;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c125c46: float fpul,fr5   (mid-6 of A)
        cpu.fr[5] = cpu.fpul as i32 as f32;
        // 0x8c125c48: mov r4,r0
        cpu.r[0] = cpu.r[4];
        // 0x8c125c4a..4e: shar r4 ×3
        shar(cpu, 4);
        shar(cpu, 4);
        shar(cpu, 4);
        // 0x8c125c50: and #31,r0
        cpu.r[0] &= 31;
        // 0x8c125c52: shar r4
        shar(cpu, 4);
        // 0x8c125c54: lds r0,fpul
        cpu.fpul = cpu.r[0];
        // 0x8c125c56: shar r4
        shar(cpu, 4);
        // 0x8c125c58: float fpul,fr2   (low-5 of B)
        cpu.fr[2] = cpu.fpul as i32 as f32;
        // 0x8c125c5a: mov r4,r0
        cpu.r[0] = cpu.r[4];
        // 0x8c125c5c: lds r5,fpul
        cpu.fpul = cpu.r[5];
        // 0x8c125c5e: float fpul,fr3   (high-5 of A)
        cpu.fr[3] = cpu.fpul as i32 as f32;
        // 0x8c125c60: lds r3,fpul
        cpu.fpul = cpu.r[3];
        // 0x8c125c62: and #63,r0
        cpu.r[0] &= 63;
        // 0x8c125c64: float fpul,fr1   (high-5 of B)
        cpu.fr[1] = cpu.fpul as i32 as f32;
        // 0x8c125c66: lds r0,fpul      (mid-6 of B; survives the call)
        cpu.fpul = cpu.r[0];
        // 0x8c125c68: mov #0,r2 / add r15,r2 / fmov fr15,@r2
        cpu.r[2] = cpu.r[15];
        let _ = mem.write_f32(cpu.r[2], cpu.fr[15]);
        // 0x8c125c6e: float fpul,fr0   (mid-6 of B)
        cpu.fr[0] = cpu.fpul as i32 as f32;
        // 0x8c125c70: mov #4,r2
        cpu.r[2] = 4;
        // high: fr3 = A/31, fr1 = B/31, lerp by fr13
        cpu.fr[3] /= cpu.fr[6];
        cpu.fr[1] /= cpu.fr[6];
        cpu.fr[1] -= cpu.fr[3];
        cpu.fr[1] *= cpu.fr[13];
        cpu.fr[1] += cpu.fr[3];
        // 0x8c125c7c: add r15,r2 / fmov fr1,@r2
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
        let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);
        // mid: fr5 = A/63, fr0 = B/63
        cpu.fr[5] /= cpu.fr[4];
        cpu.fr[0] /= cpu.fr[4];
        cpu.fr[0] -= cpu.fr[5];
        cpu.fr[0] *= cpu.fr[13];
        // 0x8c125c88: mov #8,r2
        cpu.r[2] = 8;
        cpu.fr[0] += cpu.fr[5];
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
        let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
        // low: fr7 = A/31, fr2 = B/31
        cpu.fr[7] /= cpu.fr[6];
        cpu.fr[2] /= cpu.fr[6];
        cpu.fr[2] -= cpu.fr[7];
        cpu.fr[2] *= cpu.fr[13];
        // 0x8c125c98: mov #12,r2
        cpu.r[2] = 12;
        cpu.fr[2] += cpu.fr[7];
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
        let _ = mem.write_f32(cpu.r[2], cpu.fr[2]);
        // 0x8c125ca0: mov r12,r5 / mov.l 0x8c125cf8,r0
        cpu.r[5] = cpu.r[12];
        cpu.r[0] = mem.read_u32(0x8c125cf8).unwrap_or(0);
        // 0x8c125ca4: jsr @r0 ; delay mov r11,r4. Return 0x8c125ca8.
        cpu.r[4] = cpu.r[11];
        cpu.pr = 0x8c125ca8;
        crate::registry::p_8c059054::run(cpu, mem);
        // 0x8c125ca8: mov r15,r4
        cpu.r[4] = cpu.r[15];
        // 0x8c125caa: mov.l @(12,r0),r5
        cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0);
        // 0x8c125cac: add #1,r12
        cpu.r[12] = cpu.r[12].wrapping_add(1);
        // 0x8c125cae: mov.l @r4,r3
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        // 0x8c125cb0: add #32,r5
        cpu.r[5] = cpu.r[5].wrapping_add(32);
        // 0x8c125cb2: mov.l @(4,r4),r1
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        // 0x8c125cb4: mov r11,r0 / add #80,r0
        cpu.r[0] = cpu.r[11].wrapping_add(80);
        // 0x8c125cb8: mov.l r3,@r5
        let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
        // 0x8c125cba: mov.l r1,@(4,r5)
        let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[1]);
        // 0x8c125cbc: mov.l @(8,r4),r3
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
        // 0x8c125cbe: mov.l @(12,r4),r1
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
        // 0x8c125cc0: mov.l r3,@(8,r5)
        let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
        // 0x8c125cc2: mov.l r1,@(12,r5)
        let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[1]);
        // 0x8c125cc4: mov.l @r0,r0 / shll r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        shll(cpu, 0);
        // 0x8c125cc8: add r0,r14 / add r0,r13
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]);
        cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]);
    }

    // 0x8c125cd6: add #92,r11 / fmov fr14,@r11  (entry fr5)
    cpu.r[11] = cpu.r[11].wrapping_add(92);
    let _ = mem.write_f32(cpu.r[11], cpu.fr[14]);
    // 0x8c125cda: add #16,r15
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    // 0x8c125cdc: lds.l @r15+,pr
    cpu.pr = pop_u32(cpu, mem);
    // 0x8c125cde: mov.l @r15+,r11
    cpu.r[11] = pop_u32(cpu, mem);
    // 0x8c125ce0: mov.l @r15+,r12
    cpu.r[12] = pop_u32(cpu, mem);
    // 0x8c125ce2: mov.l @r15+,r13
    cpu.r[13] = pop_u32(cpu, mem);
    // 0x8c125ce4: mov.l @r15+,r14
    cpu.r[14] = pop_u32(cpu, mem);
    // 0x8c125ce6: fmov @r15+,fr13
    cpu.fr[13] = pop_f32(cpu, mem);
    // 0x8c125ce8: fmov @r15+,fr14
    cpu.fr[14] = pop_f32(cpu, mem);
    // 0x8c125cea: rts ; delay fmov @r15+,fr15
    cpu.fr[15] = pop_f32(cpu, mem);
}
