//! 0x8c043a80 — destroy a circular list, then re-arm its head sentinel.
//!
//! ```text
//! 8c043a80  mov.l r14,@-r15
//! 8c043a82  mov.l r13,@-r15
//! 8c043a84  mov.l r12,@-r15
//! 8c043a86  mov   r4,r13           r13 = param_1 (pointer to the head field)
//! 8c043a88  mov.l r11,@-r15
//! 8c043a8a  sts.l pr,@-r15
//! 8c043a8c  mov.l @r13,r0          r0 = *param_1 (head)
//! 8c043a8e  mov   #-1,r14          r14 = 0xffffffff
//! 8c043a90  bra   0x8c043aa8
//! 8c043a92  mov.l @r0,r12          delay: r12 = head->next
//! 8c043a94  mov   r12,r11          loop: r11 = node
//! 8c043a96  mov   r11,r4
//! 8c043a98  mov.l 0x8c043ac0,r0    r0 = 0x8c018bd8 (release)
//! 8c043a9a  mov   r14,r5           r5 = -1
//! 8c043a9c  mov.l @r12,r12         r12 = node->next (before the call)
//! 8c043a9e  jsr   @r0
//! 8c043aa0  add   #8,r4            delay: release(node + 8, -1)
//! 8c043aa2  mov.l 0x8c043ac4,r0    r0 = 0x8c01032c (free thunk)
//! 8c043aa4  jsr   @r0
//! 8c043aa6  mov   r11,r4           delay: free(node)
//! 8c043aa8  mov.l @r13,r0          test: r0 = *param_1 (reloaded every pass)
//! 8c043aaa  cmp/eq r0,r12
//! 8c043aac  bf    0x8c043a94      node != head: keep destroying
//! 8c043aae  mov.l r0,@r0           head->next = head
//! 8c043ab0  mov.l @r13,r0
//! 8c043ab2  mov.l r0,@(4,r0)       head->prev = head
//! 8c043ab4  lds.l @r15+,pr
//! 8c043ab6  mov.l @r15+,r11
//! 8c043ab8  mov.l @r15+,r12
//! 8c043aba  mov.l @r15+,r13
//! 8c043abc  rts
//! 8c043abe  mov.l @r15+,r14        delay
//! ```
//!
//! The head word at `*param_1` is reloaded for every comparison, so a callee
//! that rewrites the head field changes when the walk stops; the port reloads
//! it at the same point as the original. `r12` is advanced past `node->next`
//! *before* the release call, so freeing the node cannot disturb the walk.
//! Both calls are `jsr`, so their delay slots run unconditionally: `r4` is
//! `node + 8` for the release even though `mov r11,r4` put `node` there first,
//! and `node` for the free.
//!
//! Entries leave `r0` as the reloaded head, `T` set by the final `cmp/eq` (1 on
//! every exit), the caller-saved registers holding the last callee's residue,
//! and `r11`–`r14`/`pr` restored from the frame.

use sh4_oracle::{Cpu, Memory};

/// 0x8c043ac0 holds 0x8c018bd8, 0x8c043ac4 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c043ac0;
const FREE_THUNK_PTR: u32 = 0x8c043ac4;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c043a80: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c043a82: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c043a84: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c043a86: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c043a88: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c043a8a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c043a8c: mov.l @r13,r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c043a8e: mov #-1,r14
    cpu.r[14] = 0xffff_ffff;
    // 0x8c043a90: bra 0x8c043aa8 ; 0x8c043a92: mov.l @r0,r12 (delay)
    cpu.r[12] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    loop {
        // 0x8c043aa8: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c043aaa: cmp/eq r0,r12
        cpu.set_t(cpu.r[0] == cpu.r[12]);
        if cpu.r[0] == cpu.r[12] {
            // 0x8c043aac: bf 0x8c043a94 (not taken)
            break;
        }
        // 0x8c043a94: mov r12,r11
        cpu.r[11] = cpu.r[12];
        // 0x8c043a96: mov r11,r4
        cpu.r[4] = cpu.r[11];
        // 0x8c043a98: mov.l 0x8c043ac0,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c043a9a: mov r14,r5
        cpu.r[5] = cpu.r[14];
        // 0x8c043a9c: mov.l @r12,r12
        cpu.r[12] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        // 0x8c043a9e: jsr @r0 ; 0x8c043aa0: add #8,r4 (delay)
        cpu.pr = 0x8c043aa2;
        cpu.r[4] = cpu.r[4].wrapping_add(8);
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c043aa2: mov.l 0x8c043ac4,r0
        cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
        // 0x8c043aa4: jsr @r0 ; 0x8c043aa6: mov r11,r4 (delay)
        cpu.pr = 0x8c043aa8;
        cpu.r[4] = cpu.r[11];
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c043aae: mov.l r0,@r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[0]);
    // 0x8c043ab0: mov.l @r13,r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c043ab2: mov.l r0,@(4,r0)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[0]);
    // 0x8c043ab4: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043ab6: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043ab8: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043aba: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043abc: rts ; 0x8c043abe: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
