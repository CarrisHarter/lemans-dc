//! 0x8c161a38 — insertion sort over `[r4, r5)` stepping by 4.
//!
//! `r13` is the key, the inner loop shifts larger elements right while the
//! comparator at 0x8c10f2d4 (`r15+28`, key, `[r12]`) returns nonzero, and the
//! `bt.s` delay store places the key even on outer-loop exit. Outer test is
//! `cmp/hi r14,r10` (`T = r10 > r14` unsigned); inner test is
//! `cmp/hi r9,r11` (`T = r11 > r9` unsigned). The shift block clobbers `r0`.
//! Callee-saved `r9`-`r14` and `pr` are restored; final `T` is always 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c161a38..8c161a42: push r14,r13,r12,r11,r10,r9
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
    // 8c161a44: mov r5,r10 ; 8c161a46: sts.l pr,@-r15
    cpu.r[10] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c161a48: mov r4,r9 ; 8c161a4a: mov r9,r14 ; 8c161a4c: add #4,r14
    cpu.r[9] = cpu.r[4];
    cpu.r[14] = cpu.r[9];
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    // 8c161a4e: cmp/hi r14,r10 ; 8c161a50: bf 0x8c161a82
    cpu.set_t(cpu.r[10] > cpu.r[14]);
    if cpu.t() {
        loop {
            // 8c161a52: mov.l @r14,r13 ; 54: mov r14,r12 ; 56: mov r14,r11
            cpu.r[13] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[12] = cpu.r[14];
            cpu.r[11] = cpu.r[14];
            // 8c161a58: bra 0x8c161a64 ; 5a: add #-4,r12 (delay)
            cpu.r[12] = cpu.r[12].wrapping_sub(4);
            loop {
                // 8c161a64: cmp/hi r9,r11 ; 66: bf 0x8c161a7a
                cpu.set_t(cpu.r[11] > cpu.r[9]);
                if !cpu.t() {
                    break;
                }
                // 8c161a68: mov.l @r12,r6 ; 6a: mov r15,r4
                cpu.r[6] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                cpu.r[4] = cpu.r[15];
                // 8c161a6c: mov.l 0x8c161a94,r0 — 0x8c10f2d4
                cpu.r[0] = mem.read_u32(0x8c161a94).unwrap_or(0);
                // 8c161a6e: mov r13,r5 ; 70: jsr @r0 ; 72: add #28,r4 (delay)
                cpu.r[5] = cpu.r[13];
                cpu.pr = 0x8c161a74;
                cpu.r[4] = cpu.r[4].wrapping_add(28);
                crate::registry::p_8c10f2d4::run(cpu, mem);
                // 8c161a74: extu.b r0,r0 ; 76: tst r0,r0 ; 78: bf 0x8c161a5c
                cpu.r[0] &= 0xff;
                cpu.set_t(cpu.r[0] == 0);
                if cpu.t() {
                    break;
                }
                // 8c161a5c: mov.l @r12,r0 ; 5e: mov.l r0,@r11
                cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                // 8c161a60: add #-4,r12 ; 62: add #-4,r11
                cpu.r[12] = cpu.r[12].wrapping_sub(4);
                cpu.r[11] = cpu.r[11].wrapping_sub(4);
            }
            // 8c161a7a: add #4,r14 ; 7c: cmp/hi r14,r10
            cpu.r[14] = cpu.r[14].wrapping_add(4);
            cpu.set_t(cpu.r[10] > cpu.r[14]);
            // 8c161a7e: bt.s 0x8c161a52 ; 80: mov.l r13,@r11 (delay: both sides)
            let take = cpu.t();
            let _ = mem.write_u32(cpu.r[11], cpu.r[13]);
            if !take {
                break;
            }
        }
    }
    // 8c161a82: lds.l @r15+,pr ; pop r9,r10,r11,r12,r13 ; rts ; @r15+,r14
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
