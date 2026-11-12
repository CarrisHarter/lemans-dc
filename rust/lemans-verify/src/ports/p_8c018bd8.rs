//! 0x8c018bd8 — release a reference on a refcounted handle.
//!
//! ```text
//! 8c018bd8  mov.l r14,@-r15        push r14
//! 8c018bda  mov.l r13,@-r15        push r13
//! 8c018bdc  mov   r5,r14           r14 = count arg (short in low half)
//! 8c018bde  mov.l r12,@-r15        push r12
//! 8c018be0  mov   r4,r13           r13 = handle
//! 8c018be2  sts.l pr,@-r15         push pr
//! 8c018be4  tst   r13,r13
//! 8c018be6  bt    0x8c018c40       handle == NULL: return it
//! 8c018be8  mov.l @r13,r1          r1 = *handle (object)
//! 8c018bea  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c018bec  tst   r0,r0
//! 8c018bee  bf    0x8c018bfe       nonzero: skip the lazy init
//! 8c018bf0  mov.l @(8,r1),r0       reload (still zero)
//! 8c018bf2  tst   r0,r0
//! 8c018bf4  bt    0x8c018bfa
//! 8c018bf6  bra   0x8c018bfc       dead: r0 is zero here
//! 8c018bf8  nop
//! 8c018bfa  mov   #1,r0            init refcount to 1
//! 8c018bfc  mov.l r0,@(8,r1)       object->refcount = r0
//! 8c018bfe  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c018c00  add   #-1,r0           --refcount
//! 8c018c02  tst   r0,r0
//! 8c018c04  bf.s  0x8c018c0c       new count != 0: r1 = 0
//! 8c018c06  mov.l r0,@(8,r1)       delay: object->refcount = new count
//! 8c018c08  bra   0x8c018c0e       new count == 0: keep the pointer
//! 8c018c0a  nop
//! 8c018c0c  mov   #0,r1
//! 8c018c0e  mov.l r1,@r13          *handle = r1
//! 8c018c10  mov.l @r13,r12         r12 = *handle
//! 8c018c12  tst   r12,r12
//! 8c018c14  bt    0x8c018c34       cleared handle: nothing to destroy
//! 8c018c16  tst   r12,r12          dead (r12 is nonzero)
//! 8c018c18  bt    0x8c018c2e
//! 8c018c1a  mov.l 0x8c018c4c,r0    r0 = 0x8c01032c (free thunk)
//! 8c018c1c  jsr   @r0
//! 8c018c1e  mov.l @(12,r12),r4     delay: free(object->inner)
//! 8c018c20  add   #4,r12           r12 = object + 4
//! 8c018c22  tst   r12,r12
//! 8c018c24  bt    0x8c018c2e       dead for any real object
//! 8c018c26  mov.l 0x8c018c50,r0    r0 = 0x8c018bb4 (guarded free)
//! 8c018c28  mov   r12,r4
//! 8c018c2a  jsr   @r0
//! 8c018c2c  mov   #0,r5            delay: count 0, so the guard declines
//! 8c018c2e  mov.l 0x8c018c4c,r0    r0 = 0x8c01032c (free thunk)
//! 8c018c30  jsr   @r0
//! 8c018c32  mov.l @r13,r4          delay: free(*handle) = object
//! 8c018c34  exts.w r14,r0          r0 = sext16(count arg)
//! 8c018c36  cmp/pl r0
//! 8c018c38  bf    0x8c018c40       count <= 0: keep the handle
//! 8c018c3a  mov.l 0x8c018c4c,r0
//! 8c018c3c  jsr   @r0
//! 8c018c3e  mov   r13,r4           delay: free(handle)
//! 8c018c40  mov   r13,r0           return the handle
//! 8c018c42  lds.l @r15+,pr
//! 8c018c44  mov.l @r15+,r12
//! 8c018c46  mov.l @r15+,r13
//! 8c018c48  rts
//! 8c018c4a  mov.l @r15+,r14       delay
//! ```
//!
//! The `bf.s` at 0x8c018c04 branches when the *new* count is nonzero, so the
//! handle is cleared (`*handle = 0`) when a reference remains, and kept (so the
//! object can be destroyed) when the count reaches zero. Only `exts.w` makes
//! the final `free(handle)` depend on the low 16 bits of r5; e.g. r5 =
//! 0x00010000 counts as zero and r5 = 0xffff0001 as positive.
//!
//! The three callees are the free thunk at 0x8c01032c (which tail-jumps to
//! free at 0x8c159fc0) and the guarded-free wrapper at 0x8c018bb4, called with
//! a zero count, so it never frees anything: its only visible effect is
//! clobbering the caller-saved registers the real body leaves behind. pr is
//! set for every `jsr` and is restored from the frame in the epilogue, so it
//! ends as the seed value.

use sh4_oracle::{Cpu, Memory};

/// 0x8c018c4c holds 0x8c01032c, 0x8c018c50 holds 0x8c018bb4.
const FREE_THUNK_PTR: u32 = 0x8c018c4c;
const GUARDED_FREE_PTR: u32 = 0x8c018c50;
const FREE_THUNK: u32 = 0x8c01032c;
const GUARDED_FREE: u32 = 0x8c018bb4;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c018bd8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c018bda: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c018bdc: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c018bde: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c018be0: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c018be2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c018be4: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c018be6: bt 0x8c018c40
    if cpu.r[13] != 0 {
        // 0x8c018be8: mov.l @r13,r1
        let mut r1 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c018bea: mov.l @(8,r1),r0
        let mut r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c018bec: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c018bee: bf 0x8c018bfe
        if r0 == 0 {
            // 0x8c018bf0: mov.l @(8,r1),r0
            r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
            // 0x8c018bf2: tst r0,r0
            cpu.set_t(r0 == 0);
            // 0x8c018bf4: bt 0x8c018bfa
            if r0 == 0 {
                // 0x8c018bfa: mov #1,r0
                r0 = 1;
            }
            // 0x8c018bfc: mov.l r0,@(8,r1)
            let _ = mem.write_u32(r1.wrapping_add(8), r0);
        }
        // 0x8c018bfe: mov.l @(8,r1),r0
        r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c018c00: add #-1,r0
        r0 = r0.wrapping_sub(1);
        // 0x8c018c02: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c018c04: bf.s 0x8c018c0c ; 0x8c018c06: mov.l r0,@(8,r1) (delay)
        let _ = mem.write_u32(r1.wrapping_add(8), r0);
        if r0 != 0 {
            // 0x8c018c0c: mov #0,r1
            r1 = 0;
        }
        // 0x8c018c0e: mov.l r1,@r13
        let _ = mem.write_u32(cpu.r[13], r1);
        cpu.r[1] = r1;

        // 0x8c018c10: mov.l @r13,r12
        let mut r12 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[12] = r12;
        // 0x8c018c12: tst r12,r12
        cpu.set_t(r12 == 0);
        // 0x8c018c14: bt 0x8c018c34
        if r12 != 0 {
            // 0x8c018c16/0x8c018c18: tst r12,r12 ; bt (unreachable, r12 != 0)
            // 0x8c018c1a: mov.l 0x8c018c4c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c018c1c: jsr @r0 ; 0x8c018c1e: mov.l @(12,r12),r4 (delay)
            cpu.pr = 0x8c018c20;
            cpu.r[4] = mem.read_u32(r12.wrapping_add(12)).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
            // 0x8c018c20: add #4,r12
            r12 = r12.wrapping_add(4);
            cpu.r[12] = r12;
            // 0x8c018c22: tst r12,r12
            cpu.set_t(r12 == 0);
            // 0x8c018c24: bt 0x8c018c2e
            if r12 != 0 {
                // 0x8c018c26: mov.l 0x8c018c50,r0
                cpu.r[0] = mem.read_u32(GUARDED_FREE_PTR).unwrap_or(GUARDED_FREE);
                // 0x8c018c28: mov r12,r4
                cpu.r[4] = r12;
                // 0x8c018c2a: jsr @r0 ; 0x8c018c2c: mov #0,r5 (delay)
                cpu.pr = 0x8c018c2e;
                cpu.r[5] = 0;
                crate::registry::p_8c018bb4::run(cpu, mem);
            }
            // 0x8c018c2e: mov.l 0x8c018c4c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c018c30: jsr @r0 ; 0x8c018c32: mov.l @r13,r4 (delay)
            cpu.pr = 0x8c018c34;
            cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c018c34: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c018c36: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c018c38: bf 0x8c018c40
        if (cpu.r[0] as i32) > 0 {
            // 0x8c018c3a: mov.l 0x8c018c4c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c018c3c: jsr @r0 ; 0x8c018c3e: mov r13,r4 (delay)
            cpu.pr = 0x8c018c40;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c018c40: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c018c42: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c018c44: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c018c46: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c018c48: rts ; 0x8c018c4a: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
