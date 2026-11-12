//! 0x8c09d280 — destroy a refcounted object stored behind a handle.
//!
//! Nearly the same body as 0x8c09d204, but the handle is a two-slot structure:
//! before touching `*handle` it releases the reference held in `handle[1]` by
//! calling 0x8c018bd8 with count -1, so only the object's own teardown frees
//! anything there.
//!
//! ```text
//! 8c09d280  mov.l r14,@-r15        push r14
//! 8c09d282  mov.l r13,@-r15        push r13
//! 8c09d284  mov   r5,r14           r14 = count arg (short in low half)
//! 8c09d286  mov.l r12,@-r15        push r12
//! 8c09d288  mov   r4,r13           r13 = handle
//! 8c09d28a  sts.l pr,@-r15         push pr
//! 8c09d28c  tst   r13,r13
//! 8c09d28e  bt    0x8c09d2f4       handle == NULL: return 0
//! 8c09d290  add   #4,r4            r4 = &handle[1]
//! 8c09d292  tst   r4,r4
//! 8c09d294  bt    0x8c09d29c       dead unless the handle is 0xfffffffc
//! 8c09d296  mov.l 0x8c09d300,r0    r0 = 0x8c018bd8 (release one ref)
//! 8c09d298  jsr   @r0
//! 8c09d29a  mov   #-1,r5           delay: count -1
//! 8c09d29c  tst   r13,r13          dead (r13 is nonzero here)
//! 8c09d29e  bt    0x8c09d2e8
//! 8c09d2a0  mov.l @r13,r1          r1 = *handle (object)
//! 8c09d2a2  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c09d2a4  tst   r0,r0
//! 8c09d2a6  bf    0x8c09d2b6       nonzero: skip the lazy init
//! 8c09d2a8  mov.l @(8,r1),r0       reload (still zero)
//! 8c09d2aa  tst   r0,r0
//! 8c09d2ac  bt    0x8c09d2b2
//! 8c09d2ae  bra   0x8c09d2b4       dead: r0 is zero here
//! 8c09d2b0  nop
//! 8c09d2b2  mov   #1,r0            init refcount to 1
//! 8c09d2b4  mov.l r0,@(8,r1)       object->refcount = r0
//! 8c09d2b6  mov.l @(8,r1),r0       r0 = object->refcount
//! 8c09d2b8  add   #-1,r0           --refcount
//! 8c09d2ba  tst   r0,r0
//! 8c09d2bc  bf.s  0x8c09d2c4       new count != 0: r1 = 0
//! 8c09d2be  mov.l r0,@(8,r1)       delay: object->refcount = new count
//! 8c09d2c0  bra   0x8c09d2c6       new count == 0: keep the pointer
//! 8c09d2c2  nop
//! 8c09d2c4  mov   #0,r1
//! 8c09d2c6  mov.l r1,@r13          *handle = r1
//! 8c09d2c8  mov.l @r13,r12         r12 = *handle
//! 8c09d2ca  tst   r12,r12
//! 8c09d2cc  bt    0x8c09d2e8       cleared handle: nothing to destroy
//! 8c09d2ce  tst   r12,r12          dead (r12 is nonzero)
//! 8c09d2d0  bt    0x8c09d2e2
//! 8c09d2d2  mov.l 0x8c09d304,r0    r0 = 0x8c01032c (free thunk)
//! 8c09d2d4  jsr   @r0
//! 8c09d2d6  mov.l @(12,r12),r4     delay: free(object->inner)
//! 8c09d2d8  mov.l 0x8c09d308,r0    r0 = 0x8c018e14 (guarded free)
//! 8c09d2da  add   #4,r12           r12 = object + 4
//! 8c09d2dc  mov   r12,r4
//! 8c09d2de  jsr   @r0
//! 8c09d2e0  mov   #-1,r5           delay: count -1, so the guard declines
//! 8c09d2e2  mov.l 0x8c09d304,r0    r0 = 0x8c01032c (free thunk)
//! 8c09d2e4  jsr   @r0
//! 8c09d2e6  mov.l @r13,r4          delay: free(*handle) = object
//! 8c09d2e8  exts.w r14,r0          r0 = sext16(count arg)
//! 8c09d2ea  cmp/pl r0
//! 8c09d2ec  bf    0x8c09d2f4       count <= 0: keep the handle
//! 8c09d2ee  mov.l 0x8c09d304,r0
//! 8c09d2f0  jsr   @r0
//! 8c09d2f2  mov   r13,r4           delay: free(handle)
//! 8c09d2f4  mov   r13,r0           return the handle (0 on the NULL path)
//! 8c09d2f6  lds.l @r15+,pr
//! 8c09d2f8  mov.l @r15+,r12
//! 8c09d2fa  mov.l @r15+,r13
//! 8c09d2fc  rts
//! 8c09d2fe  mov.l @r15+,r14       delay
//! ```
//!
//! The `bf.s` at 0x8c09d2bc branches when the *new* count is nonzero, so the
//! handle is cleared (`*handle = 0`) when a reference remains, and kept (so the
//! object can be destroyed) when the count reaches zero. Only `exts.w` makes
//! the final `free(handle)` depend on the low 16 bits of r5; e.g. r5 =
//! 0x00010000 counts as zero and r5 = 0xffff0001 as positive.
//!
//! All three callees are ported: the release routine 0x8c018bd8, the free thunk
//! 0x8c01032c (tail-jump to 0x8c159fc0) and the guarded-free wrapper
//! 0x8c018e14, called with -1 so its `cmp/pl` declines; the wrapper's only
//! effect is the caller-saved registers it leaves behind. pr is set for every
//! `jsr` and restored from the frame in the epilogue, so it ends as the seed
//! value. T on exit is the last callee's flag on the paths that call one, and
//! `cmp/pl`'s clear flag otherwise.
//!
//! Two degenerate inputs cannot halt under the oracle, so no spec covers them:
//! `r4 == 0xfffffffc` (the `tst r4,r4` guard skips the release call and the
//! next load reads `*0xfffffffc`) and a handle whose first word is NULL (the
//! release routine always reads `object->refcount` at address 8). Both fault in
//! the original; the port performs the same loads and moves on with zero, so
//! the oracle reports the fault and the case stays invalid.

use sh4_oracle::{Cpu, Memory};

/// 0x8c09d300 holds 0x8c018bd8, 0x8c09d304 holds 0x8c01032c, 0x8c09d308 holds
/// 0x8c018e14.
const RELEASE_PTR: u32 = 0x8c09d300;
const FREE_THUNK_PTR: u32 = 0x8c09d304;
const GUARDED_FREE_PTR: u32 = 0x8c09d308;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;
const GUARDED_FREE: u32 = 0x8c018e14;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c09d280: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c09d282: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c09d284: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c09d286: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c09d288: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c09d28a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c09d28c: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c09d28e: bt 0x8c09d2f4
    if cpu.r[13] != 0 {
        // 0x8c09d290: add #4,r4
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        // 0x8c09d292: tst r4,r4
        cpu.set_t(cpu.r[4] == 0);
        // 0x8c09d294: bt 0x8c09d29c (only for a handle at 0xfffffffc)
        if cpu.r[4] != 0 {
            // 0x8c09d296: mov.l 0x8c09d300,r0
            cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
            // 0x8c09d298: jsr @r0 ; 0x8c09d29a: mov #-1,r5 (delay)
            cpu.pr = 0x8c09d29c;
            cpu.r[5] = 0xffff_ffff;
            crate::registry::p_8c018bd8::run(cpu, mem);
        }
        // 0x8c09d29c: tst r13,r13 ; 0x8c09d29e: bt 0x8c09d2e8 (dead: r13 != 0)
        // 0x8c09d2a0: mov.l @r13,r1
        let mut r1 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[1] = r1;
        // 0x8c09d2a2: mov.l @(8,r1),r0
        let mut r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c09d2a4: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c09d2a6: bf 0x8c09d2b6
        if r0 == 0 {
            // 0x8c09d2a8: mov.l @(8,r1),r0
            r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
            // 0x8c09d2aa: tst r0,r0
            cpu.set_t(r0 == 0);
            // 0x8c09d2ac: bt 0x8c09d2b2
            if r0 == 0 {
                // 0x8c09d2b2: mov #1,r0
                r0 = 1;
            }
            // 0x8c09d2b4: mov.l r0,@(8,r1)
            let _ = mem.write_u32(r1.wrapping_add(8), r0);
        }
        // 0x8c09d2b6: mov.l @(8,r1),r0
        r0 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
        // 0x8c09d2b8: add #-1,r0
        r0 = r0.wrapping_sub(1);
        // 0x8c09d2ba: tst r0,r0
        cpu.set_t(r0 == 0);
        // 0x8c09d2bc: bf.s 0x8c09d2c4 ; 0x8c09d2be: mov.l r0,@(8,r1) (delay)
        let _ = mem.write_u32(r1.wrapping_add(8), r0);
        if r0 != 0 {
            // 0x8c09d2c4: mov #0,r1
            r1 = 0;
        }
        // 0x8c09d2c6: mov.l r1,@r13
        let _ = mem.write_u32(cpu.r[13], r1);
        cpu.r[1] = r1;

        // 0x8c09d2c8: mov.l @r13,r12
        let mut r12 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[12] = r12;
        // 0x8c09d2ca: tst r12,r12
        cpu.set_t(r12 == 0);
        // 0x8c09d2cc: bt 0x8c09d2e8
        if r12 != 0 {
            // 0x8c09d2ce/0x8c09d2d0: tst r12,r12 ; bt (unreachable, r12 != 0)
            // 0x8c09d2d2: mov.l 0x8c09d304,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c09d2d4: jsr @r0 ; 0x8c09d2d6: mov.l @(12,r12),r4 (delay)
            cpu.pr = 0x8c09d2d8;
            cpu.r[4] = mem.read_u32(r12.wrapping_add(12)).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
            // 0x8c09d2d8: mov.l 0x8c09d308,r0
            cpu.r[0] = mem.read_u32(GUARDED_FREE_PTR).unwrap_or(GUARDED_FREE);
            // 0x8c09d2da: add #4,r12
            r12 = r12.wrapping_add(4);
            cpu.r[12] = r12;
            // 0x8c09d2dc: mov r12,r4
            cpu.r[4] = r12;
            // 0x8c09d2de: jsr @r0 ; 0x8c09d2e0: mov #-1,r5 (delay)
            cpu.pr = 0x8c09d2e2;
            cpu.r[5] = 0xffff_ffff;
            crate::registry::p_8c018e14::run(cpu, mem);
            // 0x8c09d2e2: mov.l 0x8c09d304,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c09d2e4: jsr @r0 ; 0x8c09d2e6: mov.l @r13,r4 (delay)
            cpu.pr = 0x8c09d2e8;
            cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c09d2e8: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c09d2ea: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c09d2ec: bf 0x8c09d2f4
        if (cpu.r[0] as i32) > 0 {
            // 0x8c09d2ee: mov.l 0x8c09d304,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c09d2f0: jsr @r0 ; 0x8c09d2f2: mov r13,r4 (delay)
            cpu.pr = 0x8c09d2f4;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c09d2f4: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c09d2f6: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d2f8: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d2fa: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d2fc: rts ; 0x8c09d2fe: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
