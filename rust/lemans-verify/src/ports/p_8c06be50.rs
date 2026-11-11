//! 0x8c06be50 — push a node onto a doubly linked list.
//!
//! ```text
//! mov.l @r5,r1        ; r1 = *head
//! tst   r1,r1
//! bt    0x8c06be5a    ; no back-pointer fixup when the list is empty
//! mov   #72,r0
//! mov.l r4,@(r0,r1)   ; old_head->back = node
//! mov   #72,r0
//! mov   #0,r1
//! mov.l r1,@(r0,r4)   ; node->back = 0
//! mov.l @r5,r1        ; r1 = *head again
//! mov   #68,r0
//! mov.l r1,@(r0,r4)   ; node->next = old_head
//! rts
//! mov.l r4,@r5        ; *head = node  (delay slot)
//! ```
//!
//! Hidden side effects: `r0` ends as 0x44 (the last `mov #68,r0`), and `r1`
//! ends as the old head — the `mov #0,r1` is overwritten by the reload. T is
//! the `tst` result, so it is 1 exactly when the list was empty.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let node = cpu.r[4];
    let head_cell = cpu.r[5];

    let head = mem.read_u32(head_cell).unwrap_or(0);
    cpu.r[1] = head;
    cpu.set_t(head == 0);

    if head != 0 {
        cpu.r[0] = 72;
        let _ = mem.write_u32(head + 0x48, node);
    }

    cpu.r[0] = 72;
    cpu.r[1] = 0;
    let _ = mem.write_u32(node + 0x48, 0);

    let head_again = mem.read_u32(head_cell).unwrap_or(0);
    cpu.r[1] = head_again;
    cpu.r[0] = 68;
    let _ = mem.write_u32(node + 0x44, head_again);

    let _ = mem.write_u32(head_cell, node); // delay slot of `rts`
}
