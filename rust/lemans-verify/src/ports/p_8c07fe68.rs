//! 0x8c07fe68 — release a handle, then free the handle when asked.
//!
//! ```text
//! 8c07fe68  mov.l r14,@-r15        push r14
//! 8c07fe6a  mov.l r13,@-r15        push r13
//! 8c07fe6c  mov   r5,r14           r14 = count arg (short in low half)
//! 8c07fe6e  sts.l pr,@-r15         push pr
//! 8c07fe70  mov   r4,r13           r13 = handle
//! 8c07fe72  tst   r13,r13
//! 8c07fe74  bt    0x8c07fe88       NULL handle: return it
//! 8c07fe76  mov.l 0x8c07fe94,r0    r0 = 0x8c018bd8 (release)
//! 8c07fe78  jsr   @r0
//! 8c07fe7a  mov   #-1,r5           delay: release(handle, -1)
//! 8c07fe7c  exts.w r14,r0          only the low half of the count matters
//! 8c07fe7e  cmp/pl r0
//! 8c07fe80  bf    0x8c07fe88       count <= 0: keep the handle
//! 8c07fe82  mov.l 0x8c07fe98,r0    r0 = 0x8c01032c (free thunk)
//! 8c07fe84  jsr   @r0
//! 8c07fe86  mov   r13,r4           delay: free(handle)
//! 8c07fe88  mov   r13,r0           return the handle
//! 8c07fe8a  lds.l @r15+,pr
//! 8c07fe8c  mov.l @r15+,r13
//! 8c07fe8e  rts
//! 8c07fe90  mov.l @r15+,r14        delay
//! ```
//!
//! A NULL handle returns immediately with T set by the entry `tst`. Otherwise
//! the handle is released with count -1 first; that call's r4 is the untouched
//! seeded register, and it always gets the -1 delay slot. The release leaves
//! its own residue in r0-r7, then `exts.w`/`cmp/pl` test only the low 16 bits
//! of the original r5: a positive short hands the handle itself to the free
//! thunk at 0x8c01032c, which tail-jumps to the free routine. When the short is
//! zero or negative the branch skips the free and T is left clear by `cmp/pl`.
//! On the free path the released routine's residue and flags reach the
//! epilogue; the epilogue only rewrites r0 with the handle.
//!
//! Every `jsr` here writes pr with its return address; the callees save and
//! restore it, and the epilogue reloads the seeded pr from the frame.

use sh4_oracle::{Cpu, Memory};

/// 0x8c07fe94 holds 0x8c018bd8, 0x8c07fe98 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c07fe94;
const FREE_THUNK_PTR: u32 = 0x8c07fe98;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07fe68: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07fe6a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07fe6c: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07fe6e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07fe70: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07fe72: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c07fe74: bt 0x8c07fe88
    if cpu.r[13] != 0 {
        // 0x8c07fe76: mov.l 0x8c07fe94,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c07fe78: jsr @r0 ; 0x8c07fe7a: mov #-1,r5 (delay)
        cpu.pr = 0x8c07fe7c;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c07fe7c: exts.w r14,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07fe7e: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07fe80: bf 0x8c07fe88
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07fe82: mov.l 0x8c07fe98,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c07fe84: jsr @r0 ; 0x8c07fe86: mov r13,r4 (delay)
            cpu.pr = 0x8c07fe88;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07fe88: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c07fe8a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07fe8c: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07fe8e: rts ; 0x8c07fe90: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
