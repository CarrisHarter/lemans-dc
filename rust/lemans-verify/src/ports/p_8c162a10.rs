//! 0x8c162a10 — insertion sort over `[r4, r5)` stepping by 4.
//!
//! `r13` is the key, the inner loop shifts elements right while the
//! comparator returns nonzero, and the `bt.s` delay store places the key
//! even on outer-loop exit. Outer test is `cmp/hi r14,r10`
//! (`T = r10 > r14` unsigned); inner test is `cmp/hi r9,r11`
//! (`T = r11 > r9` unsigned). The shift block clobbers `r0`.
//! Callee-saved `r9`-`r14` and `pr` are restored; final `T` is always 0.
//!
//! Inner comparator on `r5 = [r12]` (element) vs `r13` (key):
//! if `[r5+20] != [r13+20]`, result is `[r13+20] > [r5+20]` signed
//! (`cmp/gt r0,r1`, i.e. shift while key's +20 is greater); else if
//! `[[r5+12]] != [[r13+12]]`, result is (`[[r13+12]] == 2`); else result
//! is the lexicographic halfword compare at 0x8c085480 over `r13+32`
//! vs `r5+32`. The merge block (`extu.b`/`tst`/`bf`) shifts while the
//! result byte is nonzero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c162a10..8c162a1a: push r14,r13,r12,r11,r10,r9
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    // 8c162a1c: mov r5,r10 ; 8c162a1e: sts.l pr,@-r15
    cpu.r[10] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c162a20: mov r4,r9 ; 8c162a22: mov r9,r14 ; 8c162a24: add #4,r14
    cpu.r[9] = cpu.r[4];
    cpu.r[14] = cpu.r[9];
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    // 8c162a26: cmp/hi r14,r10 ; 8c162a28: bf 0x8c162a7c
    cpu.set_t(cpu.r[10] > cpu.r[14]);
    if cpu.t() {
        loop {
            // 8c162a2a: mov.l @r14,r13 ; 2c: mov r14,r12 ; 2e: mov r14,r11
            cpu.r[13] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[12] = cpu.r[14];
            cpu.r[11] = cpu.r[14];
            // 8c162a30: bra 0x8c162a3c ; 32: add #-4,r12 (delay)
            cpu.r[12] = cpu.r[12].wrapping_sub(4);
            loop {
                // 8c162a3c: cmp/hi r9,r11 ; 3e: bf 0x8c162a74
                cpu.set_t(cpu.r[11] > cpu.r[9]);
                if !cpu.t() {
                    break;
                }
                // 8c162a40: mov.l @r12,r5 ; 42: mov.l @(20,r13),r1
                // 8c162a44: mov.l @(20,r5),r0 ; 46: cmp/eq r0,r1
                cpu.r[5] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                cpu.r[1] = mem.read_u32(cpu.r[13].wrapping_add(20)).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap_or(0);
                cpu.set_t(cpu.r[0] == cpu.r[1]);
                if cpu.t() {
                    // 8c162a4a: mov.l @(12,r5),r1 ; 4c: mov.l @(12,r13),r0
                    // 8c162a4e: mov.l @r1,r1 ; 50: mov.l @r0,r0
                    // 8c162a52: cmp/eq r1,r0
                    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
                    cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
                    cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                    cpu.set_t(cpu.r[1] == cpu.r[0]);
                    if cpu.t() {
                        // 8c162a56: mov.l 0x8c162a8c,r0 — 0x8c085480
                        cpu.r[0] = mem.read_u32(0x8c162a8c).unwrap_or(0);
                        // 8c162a58: mov r13,r4 ; 5a: add #32,r5
                        // 8c162a5c: jsr @r0 ; 5e: add #32,r4 (delay)
                        cpu.r[4] = cpu.r[13];
                        cpu.r[5] = cpu.r[5].wrapping_add(32);
                        cpu.pr = 0x8c162a60;
                        cpu.r[4] = cpu.r[4].wrapping_add(32);
                        crate::registry::p_8c085480::run(cpu, mem);
                        // 8c162a60: bra 0x8c162a6e ; 62: nop
                    } else {
                        // 8c162a64: cmp/eq #2,r0 ; 66: bra 0x8c162a6e
                        // 8c162a68: movt r0 (delay: captures the cmp/eq #2 T)
                        cpu.set_t(cpu.r[0] == 2);
                        cpu.r[0] = cpu.t() as u32;
                    }
                } else {
                    // 8c162a6a: cmp/gt r0,r1 (T = r1 > r0 signed)
                    // 8c162a6c: movt r0
                    cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32));
                    cpu.r[0] = cpu.t() as u32;
                }
                // 8c162a6e: extu.b r0,r0 ; 70: tst r0,r0 ; 72: bf 0x8c162a34
                cpu.r[0] &= 0xff;
                cpu.set_t(cpu.r[0] == 0);
                if cpu.t() {
                    break;
                }
                // 8c162a34: mov.l @r12,r0 ; 36: mov.l r0,@r11
                cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                // 8c162a38: add #-4,r12 ; 3a: add #-4,r11
                cpu.r[12] = cpu.r[12].wrapping_sub(4);
                cpu.r[11] = cpu.r[11].wrapping_sub(4);
            }
            // 8c162a74: add #4,r14 ; 76: cmp/hi r14,r10
            cpu.r[14] = cpu.r[14].wrapping_add(4);
            cpu.set_t(cpu.r[10] > cpu.r[14]);
            // 8c162a78: bt.s 0x8c162a2a ; 7a: mov.l r13,@r11 (delay: both sides)
            let take = cpu.t();
            let _ = mem.write_u32(cpu.r[11], cpu.r[13]);
            if !take {
                break;
            }
        }
    }
    // 8c162a7c: lds.l @r15+,pr ; pop r9,r10,r11,r12,r13 ; rts ; @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
