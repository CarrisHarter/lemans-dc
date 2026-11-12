//! 0x8c09d204 — release a reference on a refcounted handle (second copy).
//!
//! Byte-for-byte the same body as 0x8c018bd8, except for the addresses, the
//! extra redundant null test at 0x8c09d214, and the middle destroy call, which
//! goes through the guarded-free wrapper 0x8c018e14 with count -1 instead of
//! 0x8c018bb4 with count 0.
//!
//! ```text
//! 8c09d204  mov.l r14,@-r15        push r14
//! 8c09d206  mov.l r13,@-r15        push r13
//! 8c09d208  mov   r5,r14           r14 = count arg (short in low half)
//! 8c09d20a  mov.l r12,@-r15        push r12
//! 8c09d20c  mov   r4,r13           r13 = handle
//! 8c09d20e  sts.l pr,@-r15         push pr
//! 8c09d210  tst   r13,r13
//! 8c09d212  bt    0x8c09d26c       handle == NULL: return it
//! 8c09d214  tst   r13,r13          redundant (r13 is nonzero here)
//! 8c09d216  bt    0x8c09d260
//! 8c09d218  mov.l @r13,r1          r1 = *handle (object)
//! 8c09d21a  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c09d21c  tst   r0,r0
//! 8c09d21e  bf    0x8c09d22e       nonzero: skip the lazy init
//! 8c09d220  mov.l @(8,r1),r0       reload (still zero)
//! 8c09d222  tst   r0,r0
//! 8c09d224  bt    0x8c09d22a
//! 8c09d226  bra   0x8c09d22c       dead: r0 is zero here
//! 8c09d228  nop
//! 8c09d22a  mov   #1,r0            init refcount to 1
//! 8c09d22c  mov.l r0,@(8,r1)       object->refcount = r0
//! 8c09d22e  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c09d230  add   #-1,r0           --refcount
//! 8c09d232  tst   r0,r0
//! 8c09d234  bf.s  0x8c09d23c       new count != 0: r1 = 0
//! 8c09d236  mov.l r0,@(8,r1)       delay: object->refcount = new count
//! 8c09d238  bra   0x8c09d23e       new count == 0: keep the pointer
//! 8c09d23a  nop
//! 8c09d23c  mov   #0,r1
//! 8c09d23e  mov.l r1,@r13          *handle = r1
//! 8c09d240  mov.l @r13,r12         r12 = *handle
//! 8c09d242  tst   r12,r12
//! 8c09d244  bt    0x8c09d260       cleared handle: nothing to destroy
//! 8c09d246  tst   r12,r12          dead (r12 is nonzero)
//! 8c09d248  bt    0x8c09d25a
//! 8c09d24a  mov.l 0x8c09d278,r0    r0 = 0x8c01032c (free thunk)
//! 8c09d24c  jsr   @r0
//! 8c09d24e  mov.l @(12,r12),r4     delay: free(object->inner)
//! 8c09d250  mov.l 0x8c09d27c,r0    r0 = 0x8c018e14 (guarded free)
//! 8c09d252  add   #4,r12           r12 = object + 4
//! 8c09d254  mov   r12,r4
//! 8c09d256  jsr   @r0
//! 8c09d258  mov   #-1,r5           delay: count -1, so the guard declines
//! 8c09d25a  mov.l 0x8c09d278,r0    r0 = 0x8c01032c (free thunk)
//! 8c09d25c  jsr   @r0
//! 8c09d25e  mov.l @r13,r4          delay: free(*handle) = object
//! 8c09d260  exts.w r14,r0          r0 = sext16(count arg)
//! 8c09d262  cmp/pl r0
//! 8c09d264  bf    0x8c09d26c       count <= 0: keep the handle
//! 8c09d266  mov.l 0x8c09d278,r0
//! 8c09d268  jsr   @r0
//! 8c09d26a  mov   r13,r4           delay: free(handle)
//! 8c09d26c  mov   r13,r0           return the handle
//! 8c09d26e  lds.l @r15+,pr
//! 8c09d270  mov.l @r15+,r12
//! 8c09d272  mov.l @r15+,r13
//! 8c09d274  rts
//! 8c09d276  mov.l @r15+,r14       delay
//! ```
//!
//! The `bf.s` at 0x8c09d234 branches when the *new* count is nonzero, so the
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

/// 0x8c09d278 holds 0x8c01032c, 0x8c09d27c holds 0x8c018e14.
const FREE_THUNK_PTR: u32 = 0x8c09d278;
const GUARDED_FREE_PTR: u32 = 0x8c09d27c;
const FREE_THUNK: u32 = 0x8c01032c;
const GUARDED_FREE: u32 = 0x8c018e14;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c09d204: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c09d206: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c09d208: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c09d20a: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c09d20c: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c09d20e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c09d210: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c09d212: bt 0x8c09d26c
    if cpu.r[13] != 0 {
        // 0x8c09d214: tst r13,r13 ; 0x8c09d216: bt 0x8c09d260 (dead: r13 != 0)
        cpu.set_t(false);
        // 0x8c09d218: mov.l @r13,r1
        let mut r1 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[1] = r1;
        // 0x8c09d21a: mov.l @(8,r1),r0
        let mut r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c09d21c: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c09d21e: bf 0x8c09d22e
        if r0 == 0 {
            // 0x8c09d220: mov.l @(8,r1),r0
            r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
            // 0x8c09d222: tst r0,r0
            cpu.set_t(r0 == 0);
            // 0x8c09d224: bt 0x8c09d22a
            if r0 == 0 {
                // 0x8c09d22a: mov #1,r0
                r0 = 1;
            }
            // 0x8c09d22c: mov.l r0,@(8,r1)
            let _ = mem.write_u32(r1.wrapping_add(8), r0);
        }
        // 0x8c09d22e: mov.l @(8,r1),r0
        r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c09d230: add #-1,r0
        r0 = r0.wrapping_sub(1);
        // 0x8c09d232: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c09d234: bf.s 0x8c09d23c ; 0x8c09d236: mov.l r0,@(8,r1) (delay)
        let _ = mem.write_u32(r1.wrapping_add(8), r0);
        if r0 != 0 {
            // 0x8c09d23c: mov #0,r1
            r1 = 0;
        }
        // 0x8c09d23e: mov.l r1,@r13
        let _ = mem.write_u32(cpu.r[13], r1);
        cpu.r[1] = r1;

        // 0x8c09d240: mov.l @r13,r12
        let mut r12 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[12] = r12;
        // 0x8c09d242: tst r12,r12
        cpu.set_t(r12 == 0);
        // 0x8c09d244: bt 0x8c09d260
        if r12 != 0 {
            // 0x8c09d246/0x8c09d248: tst r12,r12 ; bt (unreachable, r12 != 0)
            // 0x8c09d24a: mov.l 0x8c09d278,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c09d24c: jsr @r0 ; 0x8c09d24e: mov.l @(12,r12),r4 (delay)
            cpu.pr = 0x8c09d250;
            cpu.r[4] = mem.read_u32(r12.wrapping_add(12)).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
            // 0x8c09d250: mov.l 0x8c09d27c,r0
            cpu.r[0] = mem.read_u32(GUARDED_FREE_PTR).unwrap_or(GUARDED_FREE);
            // 0x8c09d252: add #4,r12
            r12 = r12.wrapping_add(4);
            cpu.r[12] = r12;
            // 0x8c09d254: mov r12,r4
            cpu.r[4] = r12;
            // 0x8c09d256: jsr @r0 ; 0x8c09d258: mov #-1,r5 (delay)
            cpu.pr = 0x8c09d25a;
            cpu.r[5] = 0xffff_ffff;
            crate::registry::p_8c018e14::run(cpu, mem);
            // 0x8c09d25a: mov.l 0x8c09d278,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c09d25c: jsr @r0 ; 0x8c09d25e: mov.l @r13,r4 (delay)
            cpu.pr = 0x8c09d260;
            cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c09d260: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c09d262: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c09d264: bf 0x8c09d26c
        if (cpu.r[0] as i32) > 0 {
            // 0x8c09d266: mov.l 0x8c09d278,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c09d268: jsr @r0 ; 0x8c09d26a: mov r13,r4 (delay)
            cpu.pr = 0x8c09d26c;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c09d26c: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c09d26e: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d270: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d272: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d274: rts ; 0x8c09d276: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
