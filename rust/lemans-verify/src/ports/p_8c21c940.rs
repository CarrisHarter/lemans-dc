//! 0x8c21c940 — pack two source bytes and six optional samples into `[r4+8]`.
//!
//! `r5` is indirect: the bytes live at `[[r5+8]+8]`. `r7` starts as
//! `0xffff0000 | b0 | (b1 << 8)`. Bit 17 tracks `b3` against the bytes at
//! `0x8c2bab88` / `0x8c2bab89`, and a 0 or 1 is stored at `0x8c2bab8a + r6`.
//! Bit 16 does the same for `b2` and `0x8c2baba2 + r6`. A zero word at
//! `0x8c2bab84` inverts `r7`. Bits 16..21 of `[r4+4]` then copy `b2`, `b3`
//! and `b4..b7 - 128` into the halfwords at `[r4+24]` onward; a clear bit
//! stores 0. `r12`–`r14` are restored. `r5` is left pointing at the byte
//! block.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

fn ldb(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

fn stb(mem: &mut Memory, addr: u32, value: u32) {
    let _ = mem.write_u8(addr, value as u8);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c21c940: mov.l r14,@-r15
    push(cpu, mem, cpu.r[14]);
    // 0x8c21c942: mov.l r13,@-r15
    push(cpu, mem, cpu.r[13]);
    // 0x8c21c944: mov.l 0x8c21c9b4,r3  ! 0xffff0000
    cpu.r[3] = mem.read_u32(0x8c21c9b4).unwrap_or(0);
    // 0x8c21c946: mov.l 0x8c21c998,r14 ! 0x8c2bab8a
    cpu.r[14] = mem.read_u32(0x8c21c998).unwrap_or(0);
    // 0x8c21c948: mov.l r12,@-r15
    push(cpu, mem, cpu.r[12]);
    // 0x8c21c94a: mov.l @(8,r5),r5 / add #8,r5
    cpu.r[5] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    // 0x8c21c94e: mov.b @(1,r5),r0
    cpu.r[0] = ldb(mem, cpu.r[5].wrapping_add(1));
    // 0x8c21c950: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c21c952: mov.b @r5,r7
    cpu.r[7] = ldb(mem, cpu.r[5]);
    // 0x8c21c954: add #3,r13
    cpu.r[13] = cpu.r[13].wrapping_add(3);
    // 0x8c21c956: extu.b r0,r0
    cpu.r[0] &= 0xff;
    // 0x8c21c958: mov.b @r13,r2
    cpu.r[2] = ldb(mem, cpu.r[13]);
    // 0x8c21c95a: shll8 r0
    cpu.r[0] <<= 8;
    // 0x8c21c95c: extu.b r7,r7 / or r0,r7
    cpu.r[7] &= 0xff;
    cpu.r[7] |= cpu.r[0];
    // 0x8c21c960: mov.l 0x8c21c9a8,r0 / or r3,r7 / mov.b @r0,r1
    cpu.r[0] = mem.read_u32(0x8c21c9a8).unwrap_or(0);
    cpu.r[7] |= cpu.r[3];
    cpu.r[1] = ldb(mem, cpu.r[0]);
    // 0x8c21c966: cmp/hs r1,r2  (T = r2 >= r1) / bf.s ; delay add r6,r14
    cpu.set_t(cpu.r[2] >= cpu.r[1]);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[6]);
    if cpu.t() {
        // 0x8c21c96c: clear bit 17 and store 1
        cpu.r[1] = mem.read_u32(0x8c21c9b8).unwrap_or(0);
        cpu.r[3] = 1;
        cpu.r[7] &= cpu.r[1];
        stb(mem, cpu.r[14], cpu.r[3]);
    } else {
        // 0x8c21c9bc: compare against the second threshold
        cpu.r[3] = mem.read_u32(0x8c21caa4).unwrap_or(0);
        cpu.r[1] = ldb(mem, cpu.r[13]);
        cpu.r[2] = ldb(mem, cpu.r[3]);
        // cmp/hi r2,r1: T = r1 > r2
        cpu.set_t(cpu.r[1] > cpu.r[2]);
        if !cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c21caa8).unwrap_or(0);
            cpu.r[2] = 0;
            cpu.r[7] |= cpu.r[1];
            stb(mem, cpu.r[14], cpu.r[2]);
        } else {
            cpu.r[1] = ldb(mem, cpu.r[14]);
            cpu.set_t(cpu.r[1] == 0);
            if cpu.r[1] != 0 {
                cpu.r[1] = mem.read_u32(0x8c21caac).unwrap_or(0);
                cpu.r[7] &= cpu.r[1];
            }
        }
    }
    // 0x8c21c9da: same shape for b2 / bit 16 / the other flag byte
    cpu.r[1] = mem.read_u32(0x8c21cab4).unwrap_or(0);
    cpu.r[12] = cpu.r[5].wrapping_add(2);
    cpu.r[14] = mem.read_u32(0x8c21cab0).unwrap_or(0);
    cpu.r[2] = ldb(mem, cpu.r[1]);
    cpu.r[3] = ldb(mem, cpu.r[12]);
    cpu.set_t(cpu.r[3] >= cpu.r[2]);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[6]);
    if cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c21cab8).unwrap_or(0);
        cpu.r[7] &= cpu.r[2];
        cpu.r[3] = 1;
        stb(mem, cpu.r[14], cpu.r[3]);
    } else {
        cpu.r[0] = mem.read_u32(0x8c21caa4).unwrap_or(0);
        cpu.r[2] = ldb(mem, cpu.r[12]);
        cpu.r[3] = ldb(mem, cpu.r[0]);
        cpu.set_t(cpu.r[2] > cpu.r[3]);
        if !cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c21cabc).unwrap_or(0);
            cpu.r[3] = 0;
            cpu.r[7] |= cpu.r[2];
            stb(mem, cpu.r[14], cpu.r[3]);
        } else {
            cpu.r[2] = ldb(mem, cpu.r[14]);
            cpu.set_t(cpu.r[2] == 0);
            if cpu.r[2] != 0 {
                cpu.r[2] = mem.read_u32(0x8c21cab8).unwrap_or(0);
                cpu.r[7] &= cpu.r[2];
            }
        }
    }
    // 0x8c21ca12: invert r7 when the control word is zero
    cpu.r[1] = mem.read_u32(0x8c21cac0).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0);
    if cpu.r[3] == 0 {
        cpu.r[3] = (-1i32) as u32;
        cpu.r[7] ^= cpu.r[3];
    }
    // 0x8c21ca1e: store the packed word, then the six gated halfwords
    cpu.r[3] = mem.read_u32(0x8c21cabc).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[7]);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[0] & cpu.r[3] == 0);
    if cpu.t() {
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = ldb(mem, cpu.r[12]) & 0xff;
    }
    let _ = mem.write_u16(cpu.r[4].wrapping_add(24), cpu.r[0] as u16);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c21caa8).unwrap_or(0);
    cpu.set_t(cpu.r[0] & cpu.r[3] == 0);
    if cpu.t() {
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = ldb(mem, cpu.r[13]) & 0xff;
    }
    let _ = mem.write_u16(cpu.r[4].wrapping_add(26), cpu.r[0] as u16);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c21cac4).unwrap_or(0);
    cpu.set_t(cpu.r[0] & cpu.r[3] == 0);
    if cpu.t() {
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = (ldb(mem, cpu.r[5].wrapping_add(4)) & 0xff).wrapping_add((-128i32) as u32);
    }
    let _ = mem.write_u16(cpu.r[4].wrapping_add(28), cpu.r[0] as u16);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c21cac8).unwrap_or(0);
    cpu.set_t(cpu.r[0] & cpu.r[3] == 0);
    if cpu.t() {
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = (ldb(mem, cpu.r[5].wrapping_add(5)) & 0xff).wrapping_add((-128i32) as u32);
    }
    let _ = mem.write_u16(cpu.r[4].wrapping_add(30), cpu.r[0] as u16);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c21cacc).unwrap_or(0);
    cpu.set_t(cpu.r[2] & cpu.r[3] == 0);
    if cpu.t() {
        cpu.r[2] = 0;
    } else {
        cpu.r[0] = ldb(mem, cpu.r[5].wrapping_add(6)) & 0xff;
        cpu.r[2] = cpu.r[0].wrapping_add((-128i32) as u32);
    }
    cpu.r[3] = mem.read_u32(0x8c21cad0).unwrap_or(0);
    cpu.r[0] = 32;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[2] as u16);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[1] & cpu.r[3] == 0);
    if cpu.t() {
        cpu.r[3] = 0;
    } else {
        cpu.r[0] = ldb(mem, cpu.r[5].wrapping_add(7)) & 0xff;
        cpu.r[3] = cpu.r[0].wrapping_add((-128i32) as u32);
    }
    cpu.r[0] = 34;
    let _ = mem.write_u16(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[3] as u16);
    // 0x8c21ca9c: pop r12, r13 ; rts ; delay pop r14
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
