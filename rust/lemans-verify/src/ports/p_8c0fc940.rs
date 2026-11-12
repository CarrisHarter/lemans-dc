//! 0x8c0fc940 — release a reference on a refcounted object behind a handle.
//!
//! Instruction-for-instruction the same body as 0x8c018bd8/0x8c09d204 (only
//! the literal-pool addresses differ): decrement the refcount in the object,
//! destroy the object when the count reaches zero, then optionally free the
//! handle itself when the sign-extended low half of the count argument is
//! positive. Returns the handle.
//!
//! ```text
//! 8c0fc940  mov.l r14,@-r15        push r14
//! 8c0fc942  mov.l r13,@-r15        push r13
//! 8c0fc944  mov   r5,r14           r14 = count arg (short in low half)
//! 8c0fc946  mov.l r12,@-r15        push r12
//! 8c0fc948  mov   r4,r13           r13 = handle
//! 8c0fc94a  sts.l pr,@-r15         push pr
//! 8c0fc94c  tst   r13,r13
//! 8c0fc94e  bt    0x8c0fc9a8       handle == NULL: return it
//! 8c0fc950  tst   r13,r13          redundant (r13 is nonzero here)
//! 8c0fc952  bt    0x8c0fc99c
//! 8c0fc954  mov.l @r13,r1          r1 = *handle (object)
//! 8c0fc956  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c0fc958  tst   r0,r0
//! 8c0fc95a  bf    0x8c0fc96a       nonzero: skip the lazy init
//! 8c0fc95c  mov.l @(8,r1),r0       reload (still zero)
//! 8c0fc95e  tst   r0,r0
//! 8c0fc960  bt    0x8c0fc966
//! 8c0fc962  bra   0x8c0fc968       dead: r0 is zero here
//! 8c0fc964  nop
//! 8c0fc966  mov   #1,r0            init refcount to 1
//! 8c0fc968  mov.l r0,@(8,r1)       object->refcount = r0
//! 8c0fc96a  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c0fc96c  add   #-1,r0           --refcount
//! 8c0fc96e  tst   r0,r0
//! 8c0fc970  bf.s  0x8c0fc978       new count != 0: r1 = 0
//! 8c0fc972  mov.l r0,@(8,r1)       delay: object->refcount = new count
//! 8c0fc974  bra   0x8c0fc97a       new count == 0: keep the pointer
//! 8c0fc976  nop
//! 8c0fc978  mov   #0,r1
//! 8c0fc97a  mov.l r1,@r13          *handle = r1
//! 8c0fc97c  mov.l @r13,r12         r12 = *handle
//! 8c0fc97e  tst   r12,r12
//! 8c0fc980  bt    0x8c0fc99c       cleared handle: nothing to destroy
//! 8c0fc982  tst   r12,r12          dead (r12 is nonzero)
//! 8c0fc984  bt    0x8c0fc996
//! 8c0fc986  mov.l 0x8c0fc9b4,r0    r0 = 0x8c01032c (free thunk)
//! 8c0fc988  jsr   @r0
//! 8c0fc98a  mov.l @(12,r12),r4     delay: free(object->inner)
//! 8c0fc98c  mov.l 0x8c0fc9b8,r0    r0 = 0x8c018e14 (guarded free)
//! 8c0fc98e  add   #4,r12           r12 = object + 4
//! 8c0fc990  mov   r12,r4
//! 8c0fc992  jsr   @r0
//! 8c0fc994  mov   #-1,r5           delay: count -1, so the guard declines
//! 8c0fc996  mov.l 0x8c0fc9b4,r0    r0 = 0x8c01032c (free thunk)
//! 8c0fc998  jsr   @r0
//! 8c0fc99a  mov.l @r13,r4          delay: free(*handle) = object
//! 8c0fc99c  exts.w r14,r0          r0 = sext16(count arg)
//! 8c0fc99e  cmp/pl r0
//! 8c0fc9a0  bf    0x8c0fc9a8       count <= 0: keep the handle
//! 8c0fc9a2  mov.l 0x8c0fc9b4,r0
//! 8c0fc9a4  jsr   @r0
//! 8c0fc9a6  mov   r13,r4           delay: free(handle)
//! 8c0fc9a8  mov   r13,r0           return the handle
//! 8c0fc9aa  lds.l @r15+,pr
//! 8c0fc9ac  mov.l @r15+,r12
//! 8c0fc9ae  mov.l @r15+,r13
//! 8c0fc9b0  rts
//! 8c0fc9b2  mov.l @r15+,r14       delay
//! ```
//!
//! The `bf.s` at 0x8c0fc970 branches when the *new* count is nonzero, so the
//! handle is cleared (`*handle = 0`) when a reference remains, and kept (so the
//! object can be destroyed) when the count reaches zero. Only `exts.w` makes
//! the final `free(handle)` depend on the low 16 bits of r5; e.g. r5 =
//! 0x00010000 counts as zero and r5 = 0xffff0001 as positive.
//!
//! Both callees are ported: the free thunk at 0x8c01032c (tail-jump to
//! 0x8c159fc0) and the guarded-free wrapper at 0x8c018e14, called with -1 so
//! its `cmp/pl` declines; the wrapper's only effect is the caller-saved
//! registers it leaves behind. pr is set for every `jsr` and restored from the
//! frame in the epilogue, so it ends as the seed value.

use sh4_oracle::{Cpu, Memory};

/// 0x8c0fc9b4 holds 0x8c01032c, 0x8c0fc9b8 holds 0x8c018e14.
const FREE_THUNK_PTR: u32 = 0x8c0fc9b4;
const GUARDED_FREE_PTR: u32 = 0x8c0fc9b8;
const FREE_THUNK: u32 = 0x8c01032c;
const GUARDED_FREE: u32 = 0x8c018e14;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0fc940: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0fc942: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0fc944: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0fc946: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0fc948: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0fc94a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0fc94c: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0fc94e: bt 0x8c0fc9a8
    if cpu.r[13] != 0 {
        // 0x8c0fc950: tst r13,r13 ; 0x8c0fc952: bt 0x8c0fc99c (dead: r13 != 0)
        cpu.set_t(false);
        // 0x8c0fc954: mov.l @r13,r1
        let mut r1 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[1] = r1;
        // 0x8c0fc956: mov.l @(8,r1),r0
        let mut r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c0fc958: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c0fc95a: bf 0x8c0fc96a
        if r0 == 0 {
            // 0x8c0fc95c: mov.l @(8,r1),r0
            r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
            // 0x8c0fc95e: tst r0,r0
            cpu.set_t(r0 == 0);
            // 0x8c0fc960: bt 0x8c0fc966
            if r0 == 0 {
                // 0x8c0fc966: mov #1,r0
                r0 = 1;
            }
            // 0x8c0fc968: mov.l r0,@(8,r1)
            let _ = mem.write_u32(r1.wrapping_add(8), r0);
        }
        // 0x8c0fc96a: mov.l @(8,r1),r0
        r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c0fc96c: add #-1,r0
        r0 = r0.wrapping_sub(1);
        // 0x8c0fc96e: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c0fc970: bf.s 0x8c0fc978 ; 0x8c0fc972: mov.l r0,@(8,r1) (delay)
        let _ = mem.write_u32(r1.wrapping_add(8), r0);
        if r0 != 0 {
            // 0x8c0fc978: mov #0,r1
            r1 = 0;
        }
        // 0x8c0fc97a: mov.l r1,@r13
        let _ = mem.write_u32(cpu.r[13], r1);
        cpu.r[1] = r1;

        // 0x8c0fc97c: mov.l @r13,r12
        let mut r12 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[12] = r12;
        // 0x8c0fc97e: tst r12,r12
        cpu.set_t(r12 == 0);
        // 0x8c0fc980: bt 0x8c0fc99c
        if r12 != 0 {
            // 0x8c0fc982/0x8c0fc984: tst r12,r12 ; bt (unreachable, r12 != 0)
            // 0x8c0fc986: mov.l 0x8c0fc9b4,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0fc988: jsr @r0 ; 0x8c0fc98a: mov.l @(12,r12),r4 (delay)
            cpu.pr = 0x8c0fc98c;
            cpu.r[4] = mem.read_u32(r12.wrapping_add(12)).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
            // 0x8c0fc98c: mov.l 0x8c0fc9b8,r0
            cpu.r[0] = mem.read_u32(GUARDED_FREE_PTR).unwrap_or(GUARDED_FREE);
            // 0x8c0fc98e: add #4,r12
            r12 = r12.wrapping_add(4);
            cpu.r[12] = r12;
            // 0x8c0fc990: mov r12,r4
            cpu.r[4] = r12;
            // 0x8c0fc992: jsr @r0 ; 0x8c0fc994: mov #-1,r5 (delay)
            cpu.pr = 0x8c0fc996;
            cpu.r[5] = 0xffff_ffff;
            crate::registry::p_8c018e14::run(cpu, mem);
            // 0x8c0fc996: mov.l 0x8c0fc9b4,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0fc998: jsr @r0 ; 0x8c0fc99a: mov.l @r13,r4 (delay)
            cpu.pr = 0x8c0fc99c;
            cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c0fc99c: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0fc99e: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0fc9a0: bf 0x8c0fc9a8
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0fc9a2: mov.l 0x8c0fc9b4,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0fc9a4: jsr @r0 ; 0x8c0fc9a6: mov r13,r4 (delay)
            cpu.pr = 0x8c0fc9a8;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0fc9a8: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0fc9aa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fc9ac: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fc9ae: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fc9b0: rts ; 0x8c0fc9b2: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
