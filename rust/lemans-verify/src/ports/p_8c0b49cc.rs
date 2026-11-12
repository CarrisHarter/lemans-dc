//! 0x8c0b49cc — lazy one-shot initializer plus a 3x3 vector transform.
//!
//! `record = r4`, `vec = r5`. The caller's first stack argument (the word at
//! the entry `sp`) is the output pointer. If `record->flag` (the byte at
//! `record+0`) is zero, 0x8c0b48e8 initialises the 0x54-byte record; the
//! redundant `mov.b`/`extu.b`/`extu.b` sequence makes the zero test an unsigned
//! byte compare. Then 0x8c0b38b8 transforms `vec - *(float *)(record+0x48)`
//! through the 3x3 matrix at `record+0x24`, writing three floats to the output
//! pointer.
//!
//! Call sites: 0x8c0b49e2 `jsr @r0` (r0 = [0x8c0b4a00] = 0x8c0b48e8, `nop`
//! delay, pr = 0x8c0b49e6) and 0x8c0b49f0 `jsr @r0` (r0 = [0x8c0b4a04] =
//! 0x8c0b38b8, pr = 0x8c0b49f4, delay slot `mov r13,r4` after `add #36,r13` and
//! `mov r14,r5`). The output pointer is pushed at 0x8c0b49e8 and popped at
//! 0x8c0b49f4.
//!
//! `r13`/`r14`/`pr` are saved on the stack and restored, so they end at their
//! seeded values; `r15` is balanced by the epilogue (`add #4` for the pushed
//! pointer, then the three `mov.l @r15+`/`lds.l @r15+` pops). `r0`..`r7` and
//! `fr0`..`fr8` keep the residue of 0x8c0b38b8. T is never touched by that
//! transform, so it ends as the initialiser left it on the flag-zero path and
//! as the `tst` left it (clear, since the flag was nonzero) on the skip path.
//! All memory effects are the callee stores, plus `record->flag = 1` on the
//! initialising path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0b49cc mov.l r14,@-r15 ; 8c0b49ce mov.l r13,@-r15 ;
    // 8c0b49d0 mov r5,r14 ; 8c0b49d2 sts.l pr,@-r15 ; 8c0b49d4 mov r4,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];

    // 8c0b49d6..8c0b49dc — r0 = (u8)record->flag (mov.b sign-extends, the two
    // extu.b clear it); T = (r0 == 0); bf 0x8c0b49e6 skips the initialiser when
    // the flag is already set.
    cpu.r[0] = mem.read_u8(cpu.r[13]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] == 0 {
        // 8c0b49e0..8c0b49e4 — 0x8c0b48e8(record); delay slot is `nop`.
        cpu.r[0] = mem.read_u32(0x8c0b4a00).unwrap_or(0);
        cpu.pr = 0x8c0b49e6;
        crate::registry::p_8c0b48e8::run(cpu, mem);
    }

    // 8c0b49e6 mov.l @(12,r15),r0 — entry-sp stack argument (r15 is still the
    // prologue value because the initialiser balances its frame).
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    // 8c0b49e8 mov.l r0,@-r15 — push the output pointer.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);

    // 8c0b49ea..8c0b49f2 — r0 = [0x8c0b4a04]; r13 += 36; r5 = r14;
    // jsr @r0 (pr = 0x8c0b49f4); delay slot `mov r13,r4`.
    cpu.r[0] = mem.read_u32(0x8c0b4a04).unwrap_or(0);
    cpu.r[13] = cpu.r[13].wrapping_add(36);
    cpu.r[5] = cpu.r[14];
    cpu.pr = 0x8c0b49f4;
    cpu.r[4] = cpu.r[13]; // delay slot of `jsr`
    crate::registry::p_8c0b38b8::run(cpu, mem);

    // 8c0b49f4 add #4,r15 ; 8c0b49f6 lds.l @r15+,pr ;
    // 8c0b49f8 mov.l @r15+,r13 ; 8c0b49fa rts ;
    // 8c0b49fc mov.l @r15+,r14 (delay slot).
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
