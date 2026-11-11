//! 0x8c05cb24 — insert node `param_2` (r5) into the list at `param_1`:
//! prepend when `param_3` (r6) == 0, else append at the tail.
//!
//! ```text
//! tst   r6,r6
//! bf    0x8c05cb30        ; immediate
//! mov.l @(12,r4),r0       ; prepend path
//! mov.l r0,@(8,r5)        ; node->next = old head
//! rts
//! mov.l r5,@(12,r4)       ; delay slot: head = node
//! 0x8c05cb30:
//! mov.l @(12,r4),r0
//! tst   r0,r0
//! bf    0x8c05cb3c        ; immediate
//! rts
//! mov.l r5,@(12,r4)       ; delay slot: empty list -> head = node
//! 0x8c05cb3a:
//! mov   r1,r0
//! 0x8c05cb3c:
//! mov.l @(8,r0),r1
//! tst   r1,r1
//! bf    0x8c05cb3a        ; immediate
//! rts
//! mov.l r5,@(8,r0)        ; delay slot: tail->next = node
//! ```
//! Hidden side effects the C does not show: on prepend r0 keeps the old head
//! value; on the walk r0 ends as the *last node* and r1 as 0; T ends 1 on all
//! three paths (tst of r6==0, of head==0, of next==0).

use sh4_oracle::{Cpu, Memory};

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

#[inline]
fn wr(mem: &mut Memory, addr: u32, v: u32) {
    let _ = mem.write_u32(addr, v);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    if cpu.r[6] == 0 {
        cpu.set_t(true); // tst r6,r6 with r6 == 0
        let head = rd(mem, r4.wrapping_add(12)); // 0x8c05cb28
        cpu.r[0] = head;
        wr(mem, r5.wrapping_add(8), head); // 0x8c05cb2a
        wr(mem, r4.wrapping_add(12), r5); // rts delay slot 0x8c05cb2e
        return;
    }
    cpu.set_t(false); // tst r6,r6 with r6 != 0
    let mut node = rd(mem, r4.wrapping_add(12)); // 0x8c05cb30
    cpu.r[0] = node;
    if node == 0 {
        cpu.set_t(true); // tst r0,r0 with r0 == 0
        wr(mem, r4.wrapping_add(12), r5); // rts delay slot 0x8c05cb38
        return;
    }
    loop {
        let next = rd(mem, node.wrapping_add(8)); // 0x8c05cb3c
        cpu.r[1] = next;
        if next == 0 {
            cpu.set_t(true); // tst r1,r1 with r1 == 0
            wr(mem, node.wrapping_add(8), r5); // rts delay slot 0x8c05cb44
            return;
        }
        node = next; // 0x8c05cb3a: mov r1,r0
        cpu.r[0] = node;
    }
}
