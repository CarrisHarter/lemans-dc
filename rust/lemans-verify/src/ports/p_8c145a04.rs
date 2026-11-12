//! 0x8c145a04 — free every non-sentinel node of a circular list, then reset
//! the sentinel's links to point at itself.
//!
//! ```text
//! mov.l r14,@-r15
//! mov.l r13,@-r15
//! sts.l pr,@-r15
//! mov   r4,r13            ; r13 = param (points at the sentinel word)
//! mov.l @r13,r0
//! bra   0x8c145a1a
//! mov.l @r0,r14           ; delay: r14 = (*param)->next
//! 0x8c145a12: mov r14,r4
//! mov.l 0x8c145a30,r0     ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0
//! mov.l @r14,r14          ; delay: successor, read before the free
//! 0x8c145a1a: mov.l @r13,r0
//! cmp/eq r0,r14
//! bf    0x8c145a12
//! mov.l r0,@r0
//! mov.l @r13,r0
//! mov.l r0,@(4,r0)
//! lds.l @r15+,pr
//! mov.l @r15+,r13
//! rts
//! mov.l @r15+,r14         ; delay
//! ```
//!
//! `*param` is the sentinel node and is never freed: the walk stops when the
//! successor pointer has come back to it. The `jsr`'s delay slot loads the
//! successor before the callee can rewrite the block, so the traversal follows
//! the pre-free list. 0x8c01032c is a tail-call thunk to `free` (0x8c159fc0);
//! it returns through our `pr`. The exit `cmp/eq` is the last flag-setting
//! instruction on every path, so T ends 1. The epilogue restores `pr`, `r13`
//! and `r14` to their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c145a04: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c145a06: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c145a08: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c145a0a: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c145a0c: mov.l @r13,r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c145a10: mov.l @r0,r14 (delay slot of the bra at 0x8c145a0e)
    cpu.r[14] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    loop {
        // 0x8c145a1a: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c145a1c: cmp/eq r0,r14
        cpu.set_t(cpu.r[0] == cpu.r[14]);
        // 0x8c145a1e: bf 0x8c145a12
        if cpu.r[0] == cpu.r[14] {
            break;
        }
        // 0x8c145a12: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c145a14: mov.l 0x8c145a30,r0
        cpu.r[0] = mem.read_u32(0x8c145a30).unwrap_or(0x8c01032c);
        // 0x8c145a16: jsr @r0
        cpu.pr = 0x8c145a1a;
        // 0x8c145a18: mov.l @r14,r14 (delay slot; successor read pre-call)
        cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c145a20: mov.l r0,@r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[0]);
    // 0x8c145a22: mov.l @r13,r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c145a24: mov.l r0,@(4,r0)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[0]);
    // 0x8c145a26: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c145a28: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c145a2a: rts ; 0x8c145a2c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
