//! 0x8c0c8cfc — release a refcounted handle, then optionally free the handle
//! slot itself when a 16-bit flag is positive.
//!
//! ```text
//! 8c0c8cfc  mov.l r14,@-r15        push r14
//! 8c0c8cfe  mov.l r13,@-r15        push r13
//! 8c0c8d00  mov   r5,r14           r14 = flag argument (short in the low half)
//! 8c0c8d02  sts.l pr,@-r15         push pr
//! 8c0c8d04  mov   r4,r13           r13 = handle
//! 8c0c8d06  tst   r13,r13
//! 8c0c8d08  bt    0x8c0c8d1c       handle == NULL: nothing to release
//! 8c0c8d0a  mov.l 0x8c0c8d28,r0    r0 = 0x8c018bd8 (release one reference)
//! 8c0c8d0c  jsr   @r0
//! 8c0c8d0e  mov   #-1,r5           delay: count = -1
//! 8c0c8d10  exts.w r14,r0          r0 = sext16(flag)
//! 8c0c8d12  cmp/pl r0
//! 8c0c8d14  bf    0x8c0c8d1c       flag <= 0: keep the handle slot
//! 8c0c8d16  mov.l 0x8c0c8d2c,r0    r0 = 0x8c01032c (free thunk)
//! 8c0c8d18  jsr   @r0
//! 8c0c8d1a  mov   r13,r4           delay: free(handle)
//! 8c0c8d1c  mov   r13,r0           return the handle
//! 8c0c8d1e  lds.l @r15+,pr
//! 8c0c8d20  mov.l @r15+,r13
//! 8c0c8d22  rts
//! 8c0c8d24  mov.l @r15+,r14       delay
//! ```
//!
//! The flag is a `short`: only its low 16 bits count, so 0xffff0001 is
//! positive (both the release and the free run) while 0x00010000 is zero (the
//! free is skipped). The release callee is always invoked with count -1, which
//! decrements the object's refcount and destroys the object when it reaches
//! zero but never frees the handle slot itself; the trailing free is the only
//! thing that frees the slot. T at exit is `cmp/pl`'s for the non-NULL middle
//! path and the free callee's after a free call; the NULL path leaves `tst`'s
//! T set. r13, r14 and pr are restored from the frame, so they end as seeded.

use sh4_oracle::{Cpu, Memory};

/// 0x8c0c8d28 holds 0x8c018bd8, 0x8c0c8d2c holds the free thunk 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c0c8d28;
const FREE_THUNK_PTR: u32 = 0x8c0c8d2c;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0c8cfc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0c8cfe: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0c8d00: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0c8d02: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0c8d04: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0c8d06: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0c8d08: bt 0x8c0c8d1c
    if cpu.r[13] != 0 {
        // 0x8c0c8d0a: mov.l 0x8c0c8d28,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c0c8d0c: jsr @r0 ; 0x8c0c8d0e: mov #-1,r5 (delay)
        cpu.pr = 0x8c0c8d10;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c0c8d10: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0c8d12: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0c8d14: bf 0x8c0c8d1c
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0c8d16: mov.l 0x8c0c8d2c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0c8d18: jsr @r0 ; 0x8c0c8d1a: mov r13,r4 (delay)
            cpu.pr = 0x8c0c8d1c;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0c8d1c: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0c8d1e: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c8d20: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c8d22: rts ; 0x8c0c8d24: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
