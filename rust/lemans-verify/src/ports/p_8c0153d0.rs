//! 0x8c0153d0 — double-precision frexp-style decompose into sign/words/buffer.
//!
//! Spills fr5/fr4 at [r15+4]/[r15+8], extracts the sign bit into `*r4`
//! (1/-1), copies the exponent halfword to `*r5` (r13), masks it to the
//! 0x7ff0 field shifted down 4, then four FPSCR-masked calls: 0x8c014da4
//! stores [r13] big-endian at r11=r15, memmove (0x8c010e84, n=3) copies
//! r11+1 -> r14, 0x8c014da4 stores [r12-6] at r11, memmove (n=4) copies
//! r11 -> r14+3. Then shifts the r14 buffer via 0x8c014c1c (n=3, len 7):
//! positive exponent sets bit7 of `*r14` and exits; else clears bit7,
//! bumps the exponent, and loops shift+decrement until bit7 sets.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for r in [14, 13, 12, 11] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[r]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.r[2] = cpu.r[15];
    cpu.r[12] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[13] = cpu.r[5];
    cpu.r[12] = cpu.r[12].wrapping_add(4);
    cpu.r[5] = mem.read_u32(0x8c0154c4).unwrap_or(0); // 0x8000
    cpu.r[12] = cpu.r[12].wrapping_add(6);
    cpu.r[3] = cpu.r[15];
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[5]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[4]);
    cpu.r[2] = mem.read_u16(cpu.r[12]).unwrap_or(0) as u32; // mov.w @r12,r2
    cpu.r[2] &= 0xffff; // extu.w r2,r2
    cpu.r[2] &= cpu.r[5];
    cpu.set_t(cpu.r[2] == cpu.r[5]); // cmp/eq r5,r2
    cpu.r[14] = cpu.r[6]; // bf.s delay slot: always runs
    if cpu.t() {
        cpu.r[3] = (-1i32) as u32; // bra delay slot
    } else {
        cpu.r[3] = 1;
    }
    let _ = mem.write_u8(cpu.r[4], cpu.r[3] as u8); // mov.b r3,@r4
    words(cpu, mem);
}

fn words(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c015406
    cpu.r[5] = cpu.r[12];
    cpu.r[1] = mem.read_u16(cpu.r[12]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r12,r1
    cpu.r[5] = cpu.r[5].wrapping_sub(2);
    cpu.r[2] = mem.read_u16(0x8c0154c0).unwrap_or(0) as i16 as i32 as u32; // 0x7ff0
    cpu.r[11] = cpu.r[15];
    let _ = mem.write_u16(cpu.r[13], cpu.r[1] as u16); // mov.w r1,@r13
    cpu.r[1] = (-4i32) as u32;
    cpu.r[3] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r13,r3
    cpu.r[3] &= cpu.r[2];
    // shad r1,r3 (r1 = -4: arithmetic >>4)
    cpu.r[3] = ((cpu.r[3] as i32) >> 4) as u32;
    cpu.r[1] = cpu.fpscr; // sts fpscr,r1
    let _ = mem.write_u16(cpu.r[13], cpu.r[3] as u16); // mov.w r3,@r13
    cpu.r[3] = mem.read_u32(0x8c0154c8).unwrap_or(0); // 0xffe7ffff
    cpu.r[1] &= cpu.r[3];
    cpu.r[3] = mem.read_u32(0x8c0154cc).unwrap_or(0); // 0x8c014da4
    cpu.fpscr = cpu.r[1]; // lds r1,fpscr
    cpu.r[4] = cpu.r[11]; // jsr delay slot
    cpu.pr = 0x8c01542a;
    crate::registry::p_8c014da4::run(cpu, mem);
    // jsr #2: memmove n=3, r4 = r14, r5 = r11+1
    cpu.r[3] = cpu.fpscr;
    cpu.r[2] = mem.read_u32(0x8c0154c8).unwrap_or(0);
    cpu.r[5] = cpu.r[11];
    cpu.r[5] = cpu.r[5].wrapping_add(1);
    cpu.r[3] &= cpu.r[2];
    cpu.r[2] = mem.read_u32(0x8c0154d0).unwrap_or(0); // 0x8c010e84
    cpu.fpscr = cpu.r[3]; // lds r3,fpscr
    cpu.r[6] = 3;
    cpu.r[4] = cpu.r[14]; // jsr delay slot
    cpu.pr = 0x8c01543e;
    crate::registry::p_8c010e84::run(cpu, mem);
    // jsr #3: 0x8c014da4, r4 = r11, r5 = r12-6
    cpu.r[2] = cpu.fpscr;
    cpu.r[3] = mem.read_u32(0x8c0154c8).unwrap_or(0);
    cpu.r[5] = cpu.r[12];
    cpu.r[5] = cpu.r[5].wrapping_sub(6);
    cpu.r[2] &= cpu.r[3];
    cpu.r[3] = mem.read_u32(0x8c0154cc).unwrap_or(0);
    cpu.fpscr = cpu.r[2]; // jsr delay slot: lds r2,fpscr
    cpu.r[4] = cpu.r[11];
    cpu.pr = 0x8c015450;
    // NOTE: delay slot runs BEFORE the jsr: mov r11,r4 is the delay insn
    crate::registry::p_8c014da4::run(cpu, mem);
    // jsr #4: memmove n=4, r4 = r14+3, r5 = r11
    cpu.r[3] = cpu.fpscr;
    cpu.r[2] = mem.read_u32(0x8c0154c8).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = 4;
    cpu.r[5] = cpu.r[11];
    cpu.r[3] &= cpu.r[2];
    cpu.r[2] = mem.read_u32(0x8c0154d0).unwrap_or(0);
    cpu.fpscr = cpu.r[3];
    cpu.r[4] = cpu.r[4].wrapping_add(3); // jsr delay slot
    cpu.pr = 0x8c015464;
    crate::registry::p_8c010e84::run(cpu, mem);
    // jsr #5: 0x8c014c1c, r4 = r14, r5 = 7, r6 = 3
    cpu.r[3] = cpu.fpscr;
    cpu.r[2] = mem.read_u32(0x8c0154c8).unwrap_or(0);
    cpu.r[6] = 3;
    cpu.r[12] = mem.read_u32(0x8c0154d4).unwrap_or(0); // 0x8c014c1c
    cpu.r[5] = 7;
    cpu.r[3] &= cpu.r[2];
    cpu.r[4] = cpu.r[14];
    cpu.fpscr = cpu.r[3]; // jsr delay slot
    cpu.pr = 0x8c015476;
    crate::registry::p_8c014c1c::run(cpu, mem);
    sign_fix(cpu, mem);
}

fn sign_fix(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c015476
    cpu.r[3] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r13,r3
    cpu.set_t((cpu.r[3] as i32) > 0); // cmp/pl r3
    if cpu.t() {
        cpu.r[0] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b sign-extends
        cpu.r[0] |= 128;
        let _ = mem.write_u8(cpu.r[14], cpu.r[0] as u8); // bra delay slot
        epilogue(cpu, mem);
        return;
    }
    cpu.r[2] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[11] = mem.read_u16(0x8c0154c2).unwrap_or(0) as i16 as i32 as u32; // 0x80
    cpu.r[2] = cpu.r[2].wrapping_add(1);
    let _ = mem.write_u16(cpu.r[13], cpu.r[2] as u16);
    cpu.r[0] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b sign-extends
    cpu.r[0] &= 127;
    let _ = mem.write_u8(cpu.r[14], cpu.r[0] as u8); // bra delay slot
    norm_loop(cpu, mem);
}

fn norm_loop(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0154aa test; body at 0x8c015494 runs BEFORE the first test falls
    // through from sign_fix... no: bra 0x8c0154aa lands ON the test. The
    // bt at 0x8c0154b0 jumps back to the jsr body on bit-clear.
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32;
        cpu.r[3] &= 0xff; // extu.b r3,r3
        cpu.set_t(cpu.r[11] & cpu.r[3] == 0); // tst r11,r3
        if !cpu.t() {
            break;
        }
        cpu.r[2] = cpu.fpscr;
        cpu.r[3] = mem.read_u32(0x8c0154c8).unwrap_or(0);
        cpu.r[6] = 1;
        cpu.r[5] = 7;
        cpu.r[4] = cpu.r[14];
        cpu.r[2] &= cpu.r[3];
        cpu.fpscr = cpu.r[2]; // jsr delay slot
        cpu.pr = 0x8c0154a4;
        crate::registry::p_8c014c1c::run(cpu, mem);
        cpu.r[2] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[2] = cpu.r[2].wrapping_sub(1);
        let _ = mem.write_u16(cpu.r[13], cpu.r[2] as u16);
    }
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0154b2
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
