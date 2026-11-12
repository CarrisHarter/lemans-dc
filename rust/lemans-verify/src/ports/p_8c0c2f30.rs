//! 0x8c0c2f30 — free a circular list's nodes, keeping the head node.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
//! mov   r4,r13                 ; r13 = param (pointer to the head pointer)
//! mov.l @r13,r0                ; r0 = *param (head)
//! bra   0x8c0c2f46
//! mov.l @r0,r14                ; delay: r14 = head->next
//! 0x8c0c2f46: mov.l @r13,r0
//! cmp/eq r0,r14                ; T = (cursor == head)
//! bf    0x8c0c2f3e
//! 0x8c0c2f3e: mov r14,r4
//! mov.l 0x8c0c2f5c,r0          ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0
//! mov.l @r14,r14               ; delay: advance cursor BEFORE the call
//! 0x8c0c2f4c: mov.l r0,@r0     ; *head = head
//! mov.l @r13,r0                ; reload *param
//! mov.l r0,@(4,r0)             ; head->prev = head
//! lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! Every node after the head is handed to `free`; the head itself survives and
//! is rewritten as a self-loop. The prologue-pushed r13/r14/pr come back from
//! the stack in the epilogue, so their entry values survive whatever the body
//! did to them. r0 is loaded once by `0x8c0c2f46` before the final comparison,
//! written through `mov.l r0,@r0`, and loaded again for the `+4` store; the
//! last flag-setting instruction is that compare, so T is set on every exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0c2f30..0x8c0c2f34: mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
    // The epilogue restores these saved values, not the body's assignments.
    let entry_r13 = cpu.r[13];
    let entry_r14 = cpu.r[14];
    let entry_pr = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], entry_r14);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], entry_r13);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], entry_pr);

    // 0x8c0c2f36: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0c2f38: mov.l @r13,r0
    let mut r0 = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c0c2f3a: bra 0x8c0c2f46
    // 0x8c0c2f3c: mov.l @r0,r14 (delay slot runs before the branch target)
    cpu.r[14] = mem.read_u32(r0).unwrap_or(0);

    loop {
        // 0x8c0c2f46: mov.l @r13,r0
        r0 = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0c2f48: cmp/eq r0,r14
        cpu.set_t(cpu.r[14] == r0);
        // 0x8c0c2f4a: bf 0x8c0c2f3e
        if cpu.t() {
            break;
        }
        // 0x8c0c2f3e: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c0c2f40: mov.l 0x8c0c2f5c,r0
        cpu.r[0] = mem.read_u32(0x8c0c2f5c).unwrap_or(0); // 0x8c01032c
        // 0x8c0c2f42: jsr @r0 (pr = return address)
        // 0x8c0c2f44: mov.l @r14,r14 (delay slot: advance before the call)
        cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        cpu.pr = 0x8c0c2f46;
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    // 0x8c0c2f4c: mov.l r0,@r0 — r0 is the head loaded at 0x8c0c2f46.
    let _ = mem.write_u32(r0, r0);
    // 0x8c0c2f4e: mov.l @r13,r0
    let head = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c0c2f50: mov.l r0,@(4,r0)
    let _ = mem.write_u32(head.wrapping_add(4), head);

    // 0x8c0c2f52: lds.l @r15+,pr
    // 0x8c0c2f54: mov.l @r15+,r13
    // 0x8c0c2f56: rts
    // 0x8c0c2f58: mov.l @r15+,r14 (delay slot)
    cpu.r[0] = head;
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(entry_pr);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(entry_r13);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(entry_r14);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
