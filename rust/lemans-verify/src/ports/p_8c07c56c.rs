//! 0x8c07c56c — initialise an object header and release its embedded handle.
//!
//! ```text
//! 8c07c56c  mov.l r14,@-r15        push r14
//! 8c07c56e  mov.l r13,@-r15        push r13
//! 8c07c570  mov   r5,r14           r14 = tag
//! 8c07c572  sts.l pr,@-r15         push pr
//! 8c07c574  mov   r4,r13           r13 = this
//! 8c07c576  tst   r13,r13
//! 8c07c578  bt    0x8c07c5b4       this == NULL: return it
//! 8c07c57a  mov.l 0x8c07c5c0,r3    r3 = 0x8c1e981c
//! 8c07c57c  mov   r13,r1
//! 8c07c57e  add   #36,r1           r1 = this + 36
//! 8c07c580  mov.l r3,@(4,r13)      this->field4 = 0x8c1e981c
//! 8c07c582  add   #24,r4           r4 = this + 24 (embedded handle)
//! 8c07c584  mov.l @r13,r0          r0 = this->inner
//! 8c07c586  add   #52,r3           r3 = 0x8c1e9850
//! 8c07c588  mov   #-1,r5           count = -1
//! 8c07c58a  mov.l r3,@r0           this->inner->field0 = 0x8c1e9850
//! 8c07c58c  mov.l @r13,r3          r3 = this->inner
//! 8c07c58e  mov.l 0x8c07c5c4,r0    r0 = 0x8c018bd8 (release)
//! 8c07c590  sub   r3,r1            r1 = this + 36 - this->inner
//! 8c07c592  jsr   @r0
//! 8c07c594  mov.l r1,@(24,r3)      delay: this->inner->field24 = r1
//! 8c07c596  exts.w r14,r0
//! 8c07c598  tst   r0,r0
//! 8c07c59a  bt    0x8c07c5a8       tag low 16 == 0: no sentinel
//! 8c07c59c  mov   r13,r0
//! 8c07c59e  add   #36,r0           r0 = this + 36
//! 8c07c5a0  tst   r0,r0
//! 8c07c5a2  bt    0x8c07c5a8       dead for any real this
//! 8c07c5a4  mov.l 0x8c07c5c8,r0    r0 = 0x8c1e9ad4
//! 8c07c5a6  mov.l r0,@(36,r13)     this->field36 = 0x8c1e9ad4
//! 8c07c5a8  exts.w r14,r0
//! 8c07c5aa  cmp/pl r0
//! 8c07c5ac  bf    0x8c07c5b4       tag <= 0: keep this
//! 8c07c5ae  mov.l 0x8c07c5cc,r0    r0 = 0x8c01032c (free thunk)
//! 8c07c5b0  jsr   @r0
//! 8c07c5b2  mov   r13,r4           delay: free(this)
//! 8c07c5b4  mov   r13,r0           return this
//! 8c07c5b6  lds.l @r15+,pr
//! 8c07c5b8  mov.l @r15+,r13
//! 8c07c5ba  rts
//! 8c07c5bc  mov.l @r15+,r14        delay
//! ```
//!
//! The delay slot of the first `jsr` writes `(this + 36) - this->inner` to
//! `this->inner + 24` before the release routine runs; the release routine
//! receives the embedded handle at `this + 24` and count -1. The tag only
//! affects the trailer: its low 16 bits decide the sentinel store and the
//! sign-extended low 16 bits decide `free(this)`. Every pushed register ends
//! at its entry value, and pr is restored by the epilogue.

use sh4_oracle::{Cpu, Memory};

/// Literal pools the routine reads: vtable, release, trailer, free thunk.
const VTABLE_LITERAL: u32 = 0x8c07c5c0;
const RELEASE_LITERAL: u32 = 0x8c07c5c4;
const TRAILER_LITERAL: u32 = 0x8c07c5c8;
const FREE_LITERAL: u32 = 0x8c07c5cc;

const VTABLE: u32 = 0x8c1e981c;
const RELEASE: u32 = 0x8c018bd8;
const TRAILER: u32 = 0x8c1e9ad4;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07c56c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07c56e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07c570: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07c572: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07c574: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07c576: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c07c578: bt 0x8c07c5b4
    if cpu.r[13] != 0 {
        // 0x8c07c57a: mov.l 0x8c07c5c0,r3
        let mut r3 = mem.read_u32(VTABLE_LITERAL).unwrap_or(VTABLE);
        // 0x8c07c57c: mov r13,r1
        let mut r1 = cpu.r[13];
        // 0x8c07c57e: add #36,r1
        r1 = r1.wrapping_add(36);
        // 0x8c07c580: mov.l r3,@(4,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), r3);
        // 0x8c07c582: add #24,r4
        cpu.r[4] = cpu.r[4].wrapping_add(24);
        // 0x8c07c584: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c07c586: add #52,r3
        r3 = r3.wrapping_add(52);
        // 0x8c07c588: mov #-1,r5
        cpu.r[5] = 0xffff_ffff;
        // 0x8c07c58a: mov.l r3,@r0
        let _ = mem.write_u32(cpu.r[0], r3);
        // 0x8c07c58c: mov.l @r13,r3
        r3 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c07c58e: mov.l 0x8c07c5c4,r0
        cpu.r[0] = mem.read_u32(RELEASE_LITERAL).unwrap_or(RELEASE);
        // 0x8c07c590: sub r3,r1
        r1 = r1.wrapping_sub(r3);
        cpu.r[1] = r1;
        cpu.r[3] = r3;
        // 0x8c07c592: jsr @r0 ; 0x8c07c594: mov.l r1,@(24,r3) (delay)
        cpu.pr = 0x8c07c596;
        let _ = mem.write_u32(r3.wrapping_add(24), r1);
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c07c596: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07c598: tst r0,r0
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c07c59a: bt 0x8c07c5a8
        if cpu.r[0] != 0 {
            // 0x8c07c59c: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c07c59e: add #36,r0
            cpu.r[0] = cpu.r[0].wrapping_add(36);
            // 0x8c07c5a0: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c07c5a2: bt 0x8c07c5a8
            if cpu.r[0] != 0 {
                // 0x8c07c5a4: mov.l 0x8c07c5c8,r0
                cpu.r[0] = mem.read_u32(TRAILER_LITERAL).unwrap_or(TRAILER);
                // 0x8c07c5a6: mov.l r0,@(36,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(36), cpu.r[0]);
            }
        }
        // 0x8c07c5a8: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07c5aa: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07c5ac: bf 0x8c07c5b4
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07c5ae: mov.l 0x8c07c5cc,r0
            cpu.r[0] = mem.read_u32(FREE_LITERAL).unwrap_or(FREE_THUNK);
            // 0x8c07c5b0: jsr @r0 ; 0x8c07c5b2: mov r13,r4 (delay)
            cpu.pr = 0x8c07c5b4;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07c5b4: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c07c5b6: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07c5b8: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07c5ba: rts ; 0x8c07c5bc: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
