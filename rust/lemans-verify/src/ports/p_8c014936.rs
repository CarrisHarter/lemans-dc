//! 0x8c014936 — double-precision classify + normalize + big-endian word store.
//!
//! Spills fr5/fr4 as a double at [r15+20]/[r15+24], classifies via 0x8c012260.
//! Nonzero class: writes a mode flag (0x4b8/0x44c) to 0x8c23d8b8, stores the
//! class at [r15+20]/[r15+24], reloads fr1/fr0, returns. Zero class: sets
//! double mode in FPSCR (or 0x80000), returns 0 words if fr4 word is zero,
//! else extracts the exponent nibble; nonzero exponent short-circuits to the
//! 0xa86 fixup, zero exponent runs the 0x8c014da4 store loop (FPSCR masked
//! with 0xffe7ffff around each jsr; jsr delay slot is always `lds r3,fpscr`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for r in [14, 13, 12, 11, 10, 9, 8] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[r]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(28);
    cpu.r[1] = cpu.r[15];
    cpu.r[2] = cpu.r[15];
    cpu.r[8] = mem.read_u32(0x8c014a34).unwrap_or(0); // 0x800f
    cpu.r[1] = cpu.r[1].wrapping_add(24);
    cpu.r[3] = mem.read_u32(0x8c014a38).unwrap_or(0); // 0x8c012260
    cpu.r[2] = cpu.r[2].wrapping_add(20);
    cpu.r[0] = 20;
    let _ = mem.write_f32(cpu.r[2], cpu.fr[5]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[4]);
    cpu.r[14] = cpu.r[4];
    cpu.fr[5] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0); // jsr delay slot
    cpu.pr = 0x8c014960;
    crate::registry::p_8c012260::run(cpu, mem);
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    cpu.r[4] = cpu.r[0]; // bt.s delay slot: always runs
    if cpu.t() {
        double_path(cpu, mem);
        return;
    }
    class_nonzero(cpu, mem);
}

fn class_nonzero(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014966
    cpu.r[0] = cpu.r[4];
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    cpu.r[0] = cpu.r[4]; // bf.s delay slot: always runs
    if cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c014a3c).unwrap_or(0); // 0x8c23d8b8
        cpu.r[2] = mem.read_u16(0x8c014a18).unwrap_or(0) as i16 as i32 as u32; // 0x4b8
        let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    }
    cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
    cpu.r[4] = (-1i32) as u32; // bf.s delay slot: always runs
    if cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c014a3c).unwrap_or(0);
        cpu.r[3] = mem.read_u16(0x8c014a1a).unwrap_or(0) as i16 as i32 as u32; // 0x44c
        let _ = mem.write_u32(cpu.r[2], cpu.r[3]);
    }
    cpu.r[3] = cpu.r[15];
    cpu.r[2] = cpu.r[15];
    cpu.r[3] = cpu.r[3].wrapping_add(20);
    cpu.r[2] = cpu.r[2].wrapping_add(20);
    let _ = mem.write_u32(cpu.r[3], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(4), cpu.r[4]); // bra delay slot
    tail_load(cpu, mem);
}

fn tail_load(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014a98
    cpu.r[0] = 20;
    cpu.fr[1] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[0] = 24;
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014aa0
    cpu.r[15] = cpu.r[15].wrapping_add(28);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for r in [8, 9, 10, 11, 12, 13] {
        cpu.r[r] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn double_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01498e
    cpu.r[0] = 0x8c014a40; // mova
    cpu.r[2] = mem.read_u32(0x8c014a24).unwrap_or(0); // 0x80000
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = 20;
    cpu.r[3] = cpu.fpscr; // sts fpscr,r3
    cpu.fr[1] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[0] = 24;
    cpu.fr[3] = cpu.fr[2];
    cpu.r[3] |= cpu.r[2];
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.fpscr = cpu.r[3]; // lds r3,fpscr
    cpu.set_t(cpu.fr[0] == cpu.fr[2]); // fcmp/eq fr2,fr0
    if cpu.t() {
        cpu.r[0] = 0x8c014a40; // mova
        cpu.r[3] = 0;
        let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[1] = cpu.fr[0]; // bra delay slot
        epilogue(cpu, mem);
        return;
    }
    exponent_path(cpu, mem);
}

fn exponent_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0149b4
    cpu.r[13] = cpu.r[15];
    cpu.r[13] = cpu.r[13].wrapping_add(20);
    cpu.r[4] = cpu.r[13];
    cpu.r[4] = cpu.r[4].wrapping_add(6);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[4] = mem.read_u16(cpu.r[4]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r4,r4
    cpu.r[2] = mem.read_u16(0x8c014a1c).unwrap_or(0) as i16 as i32 as u32; // 0x7ff
    cpu.r[4] &= 0xffff; // extu.w r4,r4
    cpu.r[3] = mem.read_u16(0x8c014a1e).unwrap_or(0) as i16 as i32 as u32; // 0xfc02
    cpu.r[4] >>= 2; // shlr2
    cpu.r[1] = mem.read_u16(0x8c014a20).unwrap_or(0) as i16 as i32 as u32; // 0x3fe
    cpu.r[4] >>= 2; // shlr2
    cpu.r[4] &= cpu.r[2];
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[3]);
    cpu.r[0] = cpu.r[4];
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[4]); // bf.s delay slot: always runs
    if !cpu.t() {
        fixup_short(cpu, mem);
        return;
    }
    long_path(cpu, mem);
}

fn fixup_short(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014a86
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = mem.read_u16(0x8c014b56).unwrap_or(0) as i16 as i32 as u32; // 0x3fe0
    cpu.r[2] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r3,r2
    cpu.r[2] &= cpu.r[8];
    let _ = mem.write_u16(cpu.r[3], cpu.r[2] as u16);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[2] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[2] |= cpu.r[4];
    let _ = mem.write_u16(cpu.r[3], cpu.r[2] as u16);
    tail_load(cpu, mem);
}

fn long_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0149da (bf.s not taken: r0 == 0)
    cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[5] = cpu.r[13];
    cpu.r[10] = cpu.r[15];
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.r[3] = cpu.r[3].wrapping_add(1);
    cpu.r[12] = mem.read_u32(0x8c014a44).unwrap_or(0); // 0x8c014da4
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    cpu.r[10] = cpu.r[10].wrapping_add(12);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[10];
    cpu.r[0] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r3,r0
    cpu.r[0] &= cpu.r[8];
    let _ = mem.write_u16(cpu.r[3], cpu.r[0] as u16);
    // jsr @r12 #1 (r4 = r10, r5 = r13+4)
    cpu.r[3] = cpu.fpscr;
    cpu.r[0] = mem.read_u32(0x8c014a30).unwrap_or(0); // 0xffe7ffff
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    cpu.r[3] &= cpu.r[0];
    cpu.fpscr = cpu.r[3]; // jsr delay slot
    cpu.pr = 0x8c014a00;
    crate::registry::p_8c014da4::run(cpu, mem);
    // jsr @r12 #2 (r4 = r10+4, r5 = r13)
    cpu.r[3] = cpu.fpscr;
    cpu.r[2] = mem.read_u32(0x8c014a30).unwrap_or(0);
    cpu.r[4] = cpu.r[10];
    cpu.r[5] = cpu.r[13];
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.r[3] &= cpu.r[2];
    cpu.fpscr = cpu.r[3]; // jsr delay slot
    cpu.pr = 0x8c014a10;
    crate::registry::p_8c014da4::run(cpu, mem);
    cpu.r[11] = cpu.r[10];
    cpu.r[11] = cpu.r[11].wrapping_add(1);
    cpu.r[9] = 16; // bra delay slot
    norm_loop(cpu, mem);
}

fn norm_loop(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014a5e: while bit16 clear, rotate via 0x8c014bd4; when set, run
    // the jsr pair ONCE then fall out (no loop-back after jsr #4).
    loop {
        cpu.r[2] = mem.read_u8(cpu.r[11]).unwrap_or(0) as u32; // mov.b @r11,r2
        cpu.r[2] &= 0xff; // extu.b r2,r2
        cpu.set_t(cpu.r[9] & cpu.r[2] == 0); // tst r9,r2
        if !cpu.t() {
            break; // bt not taken: bit set -> jsr pair below
        }
        // 0x8c014a48 rotate block, then falls back to the tst
        cpu.r[3] = cpu.fpscr; // sts fpscr,r3
        cpu.r[2] = mem.read_u32(0x8c014b60).unwrap_or(0); // 0xffe7ffff
        cpu.r[5] = 7;
        cpu.r[3] &= cpu.r[2];
        cpu.r[2] = mem.read_u32(0x8c014b64).unwrap_or(0); // 0x8c014bd4
        cpu.fpscr = cpu.r[3]; // lds r3,fpscr
        cpu.r[4] = cpu.r[11]; // jsr delay slot: mov r11,r4
        cpu.pr = 0x8c014a58;
        crate::registry::p_8c014bd4::run(cpu, mem);
        cpu.r[3] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_sub(1);
        let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    }
    // jsr @r12 #3 (r4 = [r15+4], r5 = r10)
    cpu.r[3] = cpu.fpscr;
    cpu.r[2] = mem.read_u32(0x8c014b60).unwrap_or(0); // 0xffe7ffff
    cpu.r[5] = cpu.r[10];
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[10]);
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] &= cpu.r[2];
    cpu.fpscr = cpu.r[3]; // jsr delay slot
    cpu.pr = 0x8c014a76;
    crate::registry::p_8c014da4::run(cpu, mem);
    // jsr @r12 #4 (r4 = r13, r5 = [r15+8]+4)
    cpu.r[3] = cpu.fpscr;
    cpu.r[2] = mem.read_u32(0x8c014b60).unwrap_or(0);
    cpu.r[4] = cpu.r[13];
    cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.r[3] &= cpu.r[2];
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fpscr = cpu.r[3]; // jsr delay slot
    cpu.pr = 0x8c014a86;
    crate::registry::p_8c014da4::run(cpu, mem);
    fixup_short(cpu, mem);
}
