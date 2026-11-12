//! 0x8c020a78 — release the three handles of a refcounted block, then free it.
//!
//! ```text
//! 8c020a78  mov.l r14,@-r15        push r14
//! 8c020a7a  mov.l r13,@-r15        push r13
//! 8c020a7c  mov   r4,r14           r14 = param1 (block / handle array)
//! 8c020a7e  sts.l pr,@-r15         push pr
//! 8c020a80  mov   r5,r13           r13 = param2 (count, short in low half)
//! 8c020a82  tst   r14,r14
//! 8c020a84  bt    0x8c020aac       param1 == NULL: return it
//! 8c020a86  mov.l 0x8c020ab8,r0    r0 = 0x8c018bd8 (release handle)
//! 8c020a88  add   #8,r4            r4 = param1 + 8
//! 8c020a8a  jsr   @r0              ; 8c020a8c: mov #-1,r5 (delay)
//! 8c020a8e  mov.l 0x8c020ab8,r0
//! 8c020a90  mov   r14,r4
//! 8c020a92  add   #4,r4            r4 = param1 + 4
//! 8c020a94  jsr   @r0              ; 8c020a96: mov #-1,r5 (delay)
//! 8c020a98  mov.l 0x8c020ab8,r0
//! 8c020a9a  mov   r14,r4           r4 = param1
//! 8c020a9c  jsr   @r0              ; 8c020a9e: mov #-1,r5 (delay)
//! 8c020aa0  exts.w r13,r0          r0 = sext16(count)
//! 8c020aa2  cmp/pl r0
//! 8c020aa4  bf    0x8c020aac       count <= 0: keep the block
//! 8c020aa6  mov.l 0x8c020abc,r0    r0 = 0x8c01032c (free thunk)
//! 8c020aa8  jsr   @r0              ; 8c020aaa: mov r14,r4 (delay)
//! 8c020aac  mov   r14,r0           return param1
//! 8c020aae  lds.l @r15+,pr
//! 8c020ab0  mov.l @r15+,r13
//! 8c020ab2  rts
//! 8c020ab4  mov.l @r15+,r14        delay
//! ```
//!
//! The three `0x8c018bd8` calls release the handles at param1+8, param1+4 and
//! param1. Their count argument is `0xffffffff`, so the release's own
//! `free(handle)` tail is always skipped; only the object teardown inside it
//! depends on the refcount it finds. The final `0x8c01032c` free call happens
//! only when the low 16 bits of param2, sign-extended, are positive: the whole
//! `r5` word is otherwise irrelevant (0x00010000 counts as zero, 0xffff0001 as
//! positive).
//!
//! Registers not written by the epilogue keep whatever the last callee left
//! there, so the calls are delegated to the ported routines rather than
//! approximated. The epilogue always returns param1 in r0; pr comes back from
//! the frame (the seed value), and T is the last flag set: the `tst` on the
//! NULL path, the `cmp/pl` on the no-free path, or the free routine's final
//! compare on the free path.

use sh4_oracle::{Cpu, Memory};

/// Literal-pool slot holding 0x8c018bd8, and the routine behind it.
const RELEASE_SLOT: u32 = 0x8c020ab8;
const RELEASE: u32 = 0x8c018bd8;
/// Literal-pool slot holding 0x8c01032c (the free thunk).
const FREE_SLOT: u32 = 0x8c020abc;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c020a78: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c020a7a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c020a7c: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c020a7e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c020a80: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c020a82: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c020a84: bt 0x8c020aac
    if cpu.r[14] != 0 {
        // 0x8c020a86: mov.l 0x8c020ab8,r0
        cpu.r[0] = mem.read_u32(RELEASE_SLOT).unwrap_or(RELEASE);
        // 0x8c020a88: add #8,r4
        cpu.r[4] = cpu.r[4].wrapping_add(8);
        // 0x8c020a8a: jsr @r0 ; 0x8c020a8c: mov #-1,r5 (delay)
        cpu.pr = 0x8c020a8e;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c020a8e: mov.l 0x8c020ab8,r0
        cpu.r[0] = mem.read_u32(RELEASE_SLOT).unwrap_or(RELEASE);
        // 0x8c020a90: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c020a92: add #4,r4
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        // 0x8c020a94: jsr @r0 ; 0x8c020a96: mov #-1,r5 (delay)
        cpu.pr = 0x8c020a98;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c020a98: mov.l 0x8c020ab8,r0
        cpu.r[0] = mem.read_u32(RELEASE_SLOT).unwrap_or(RELEASE);
        // 0x8c020a9a: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c020a9c: jsr @r0 ; 0x8c020a9e: mov #-1,r5 (delay)
        cpu.pr = 0x8c020aa0;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c020aa0: exts.w r13,r0
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32;
        // 0x8c020aa2: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c020aa4: bf 0x8c020aac
        if (cpu.r[0] as i32) > 0 {
            // 0x8c020aa6: mov.l 0x8c020abc,r0
            cpu.r[0] = mem.read_u32(FREE_SLOT).unwrap_or(FREE_THUNK);
            // 0x8c020aa8: jsr @r0 ; 0x8c020aaa: mov r14,r4 (delay)
            cpu.pr = 0x8c020aac;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c020aac: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c020aae: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c020ab0: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c020ab2: rts ; 0x8c020ab4: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
