//! 0x8c1bb2a8 — drop the two handles inside *p, then maybe free p.
//!
//! ```text
//! 8c1bb2a8  mov.l r14,@-r15        push r14
//! 8c1bb2aa  mov.l r13,@-r15        push r13
//! 8c1bb2ac  mov   r5,r14           r14 = flag (short in the low half)
//! 8c1bb2ae  sts.l pr,@-r15         push pr
//! 8c1bb2b0  mov   r4,r13           r13 = p
//! 8c1bb2b2  tst   r13,r13
//! 8c1bb2b4  bt    0x8c1bb2d2       p == NULL: return it
//! 8c1bb2b6  mov.l 0x8c1bb2dc,r0    r0 = 0x8c018bd8 (release)
//! 8c1bb2b8  add   #4,r4            release(&p->second)
//! 8c1bb2ba  jsr   @r0
//! 8c1bb2bc  mov   #-1,r5           delay: count = -1
//! 8c1bb2be  mov.l 0x8c1bb2dc,r0
//! 8c1bb2c0  mov   r13,r4           release(p)
//! 8c1bb2c2  jsr   @r0
//! 8c1bb2c4  mov   #-1,r5           delay: count = -1
//! 8c1bb2c6  exts.w r14,r0          flag as a signed 16-bit value
//! 8c1bb2c8  cmp/pl r0
//! 8c1bb2ca  bf    0x8c1bb2d2       flag <= 0: keep p
//! 8c1bb2cc  mov.l 0x8c1bb2e0,r0    r0 = 0x8c01032c (free thunk)
//! 8c1bb2ce  jsr   @r0
//! 8c1bb2d0  mov   r13,r4           delay: free(p)
//! 8c1bb2d2  mov   r13,r0
//! 8c1bb2d4  lds.l @r15+,pr
//! 8c1bb2d6  mov.l @r15+,r13
//! 8c1bb2d8  rts
//! 8c1bb2da  mov.l @r15+,r14       delay
//! ```
//!
//! Both release calls pass -1, so they never free the handle itself: they only
//! decrement the referent's refcount and clear `*handle` while references
//! remain. The third call is the only one whose argument is sign-extended from
//! r5: `r5 = 0x00010000` takes the keep path, `r5 = 0x0001ffff` frees.
//!
//! Exit T: `tst` leaves T=1 on the NULL path; every other path is compared
//! against zero-aware callees whose last comparison is false (`cmp/pl r0` when
//! it declines, the free port's bin walk otherwise), so T=0 there.
//!
//! r13/r14/pr are pushed and later popped, so they end at their entry values;
//! r0 ends as r13. The two release calls leave r1-r7 as the callee does.

use sh4_oracle::{Cpu, Memory};

/// 0x8c1bb2dc holds 0x8c018bd8, 0x8c1bb2e0 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c1bb2dc;
const FREE_PTR: u32 = 0x8c1bb2e0;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];
    let flag = cpu.r[5];
    let entry_pr = cpu.pr;
    let entry_r13 = cpu.r[13];
    let entry_r14 = cpu.r[14];

    // 0x8c1bb2b2: tst r13,r13 ; bt 0x8c1bb2d2
    cpu.set_t(p == 0);
    if p != 0 {
        // 0x8c1bb2b6: mov.l 0x8c1bb2dc,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c1bb2b8: add #4,r4 ; 0x8c1bb2ba: jsr @r0 ; delay: mov #-1,r5
        cpu.r[4] = p.wrapping_add(4);
        cpu.r[5] = 0xffff_ffff;
        cpu.pr = 0x8c1bb2be;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c1bb2be: mov.l 0x8c1bb2dc,r0 ; 0x8c1bb2c0: mov r13,r4
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        cpu.r[4] = p;
        cpu.r[5] = 0xffff_ffff;
        cpu.pr = 0x8c1bb2c6;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c1bb2c6: exts.w r14,r0 ; 0x8c1bb2c8: cmp/pl r0 ; 0x8c1bb2ca: bf
        cpu.r[0] = flag as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        if (cpu.r[0] as i32) > 0 {
            // 0x8c1bb2cc: mov.l 0x8c1bb2e0,r0 ; 0x8c1bb2ce: jsr @r0
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE_THUNK);
            cpu.r[4] = p; // delay: mov r13,r4
            cpu.pr = 0x8c1bb2d2;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c1bb2d2: mov r13,r0 ; 0x8c1bb2d4-0x8c1bb2da: pop pr, r13, r14
    cpu.r[13] = entry_r13;
    cpu.r[14] = entry_r14;
    cpu.pr = entry_pr;
    cpu.r[0] = p;
    cpu.set_t(p == 0);
}
