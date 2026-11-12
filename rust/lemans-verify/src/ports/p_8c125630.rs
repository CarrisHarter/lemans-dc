//! 0x8c125630 — free every node of a circular list, then re-arm the head
//! sentinel as an empty list.
//!
//! ```text
//! mov.l r14,@-r15
//! mov.l r13,@-r15
//! sts.l pr,@-r15
//! mov   r4,r13        ; r13 = pointer to the head word
//! mov.l @r13,r0       ; r0 = head
//! bra   0x8c125646
//! mov.l @r0,r14       ; delay slot: r14 = head->next (first real node)
//! 0x8c12563e: mov   r14,r4         ; loop: r4 = node
//! mov.l 0x8c12565c,r0 ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0
//! mov.l @r14,r14      ; delay slot: r14 = node->next (before free clobbers it)
//! 0x8c125646: mov.l @r13,r0        ; reload head
//! cmp/eq r0,r14
//! bf    0x8c12563e
//! mov.l r0,@r0        ; head->next = head
//! mov.l @r13,r0
//! mov.l r0,@(4,r0)    ; head->prev = head (the list is doubly linked)
//! lds.l @r15+,pr
//! mov.l @r15+,r13
//! rts
//! mov.l @r15+,r14     ; delay slot
//! ```
//!
//! `r4` is the address of the list head word, not the head node itself:
//! `mov.l @r13,r0` loads the sentinel node pointer and `mov.l @r0,r14` loads
//! the sentinel's `next`, so an empty list (next == self) runs zero iterations.
//! The loop calls the free thunk directly through the literal pool entry at
//! 0x8c12565c with the node in r4; the delay slot reads `node->next` *after*
//! `r4` was copied, so `r4` still names the node being freed while `r14`
//! already holds its successor. The callee preserves r13/r14 (both are
//! callee-saved), so the reloaded head and the carried successor survive each
//! call. The final `cmp/eq` of the exit test is the last flag setter, so T is
//! 1 when the routine returns; the epilogue restores pr, r13 and r14 to their
//! entry values and r0 ends as the head pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c125630: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c125632: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c125634: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c125636: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c125638: mov.l @r13,r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c12563a: bra 0x8c125646 ; 0x8c12563c: mov.l @r0,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[0]).unwrap_or(0);

    let r0;
    loop {
        // 0x8c125646: mov.l @r13,r0
        let head = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c125648: cmp/eq r0,r14
        cpu.set_t(cpu.r[14] == head);
        // 0x8c12564a: bf 0x8c12563e
        if cpu.t() {
            r0 = head;
            break;
        }
        // 0x8c12563e: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c125640: mov.l 0x8c12565c,r0
        cpu.r[0] = mem.read_u32(0x8c12565c).unwrap_or(0x8c01032c);
        // 0x8c125642: jsr @r0 ; 0x8c125644: mov.l @r14,r14 (delay slot)
        cpu.pr = 0x8c125646;
        cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    // 0x8c12564c: mov.l r0,@r0 — head->next = head
    let _ = mem.write_u32(r0, r0);
    // 0x8c12564e: mov.l @r13,r0
    let head = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[0] = head;
    // 0x8c125650: mov.l r0,@(4,r0) — head->prev = head
    let _ = mem.write_u32(head.wrapping_add(4), head);
    // 0x8c125652: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c125654: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c125656: rts ; 0x8c125658: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
