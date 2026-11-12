//! 0x8c05f8cc — release a list node, then free it if the signed 16-bit
//! threshold in r5 is positive.
//!
//! ```text
//! 8c05f8cc  mov.l r14,@-r15        push r14
//! 8c05f8ce  mov.l r13,@-r15        push r13
//! 8c05f8d0  mov   r5,r14           r14 = threshold argument
//! 8c05f8d2  sts.l pr,@-r15         push pr
//! 8c05f8d4  mov   r4,r13           r13 = node
//! 8c05f8d6  tst   r13,r13
//! 8c05f8d8  bt    0x8c05f8ec       node == NULL: return it
//! 8c05f8da  mov.l 0x8c05f8f8,r0    r0 = 0x8c018bd8 (release)
//! 8c05f8dc  jsr   @r0
//! 8c05f8de  mov   #-1,r5           delay: release(node, -1)
//! 8c05f8e0  exts.w r14,r0          r0 = sext16(threshold)
//! 8c05f8e2  cmp/pl r0
//! 8c05f8e4  bf    0x8c05f8ec       threshold <= 0: skip the free
//! 8c05f8e6  mov.l 0x8c05f8fc,r0    r0 = 0x8c01032c (free thunk)
//! 8c05f8e8  jsr   @r0
//! 8c05f8ea  mov   r13,r4           delay: free(node)
//! 8c05f8ec  mov   r13,r0           return the node
//! 8c05f8ee  lds.l @r15+,pr
//! 8c05f8f0  mov.l @r15+,r13
//! 8c05f8f2  rts
//! 8c05f8f4  mov.l @r15+,r14       delay
//! ```
//!
//! Literal pool: 0x8c05f8f8 = 0x8c018bd8, 0x8c05f8fc = 0x8c01032c.
//!
//! The first callee is always called with r4 unchanged (the node) and r5 set
//! to -1 by the delay slot, so a real caller passing r5 = -1 (all four call
//! sites do) never reaches the second call. Only the low 16 bits of r5 matter:
//! `exts.w` sign-extends them, so 0x0000ffff is non-positive while
//! 0xffff0001 is positive. T on the NULL path is `tst` (set), on the
//! non-positive path `cmp/pl` (clear), and after a call it is whatever the
//! callee left. The epilogue restores pr, r13 and r14 from the frame, so r0 is
//! the only register whose result matters beyond the callee-visible work.

use sh4_oracle::{Cpu, Memory};

/// 0x8c05f8f8 holds 0x8c018bd8, 0x8c05f8fc holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c05f8f8;
const FREE_THUNK_PTR: u32 = 0x8c05f8fc;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05f8cc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05f8ce: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05f8d0: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05f8d2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05f8d4: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05f8d6: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05f8d8: bt 0x8c05f8ec
    if cpu.r[13] != 0 {
        // 0x8c05f8da: mov.l 0x8c05f8f8,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c05f8dc: jsr @r0 ; 0x8c05f8de: mov #-1,r5 (delay)
        cpu.pr = 0x8c05f8e0;
        cpu.r[5] = u32::MAX;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c05f8e0: exts.w r14,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c05f8e2: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05f8e4: bf 0x8c05f8ec
        if (cpu.r[0] as i32) > 0 {
            // 0x8c05f8e6: mov.l 0x8c05f8fc,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c05f8e8: jsr @r0 ; 0x8c05f8ea: mov r13,r4 (delay)
            cpu.pr = 0x8c05f8ec;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05f8ec: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05f8ee: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05f8f0: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05f8f2: rts ; 0x8c05f8f4: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
