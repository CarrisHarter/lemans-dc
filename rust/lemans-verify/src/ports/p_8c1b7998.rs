//! 0x8c1b7998 — sorted insert of node `r6` into the list whose sentinel is
//! `r5` (links: +4 prev, +8 next, +0 float key), ascending by key.
//!
//! Original:
//! ```text
//! mov.l @(8,r5),r0 ; tst r0,r0 ; bf.s 0x8c1b79aa ; mov r5,r1   (delay)
//! ; empty list: r6->prev=r5, r6->next=0, rts ; r5->next=r6 (delay), r0=0
//! 0x8c1b79aa: bra 0x8c1b79c0 ; fmov @r6,fr1                  (delay)
//! 0x8c1b79ae: mov r5,r1 ; mov.l @(8,r5),r5 ; tst r5,r5 ; bf 0x8c1b79c0
//! ; end of list: r6->prev=r1, r0=0, r6->next=0, rts ; r1->next=r6 (delay)
//! 0x8c1b79c0: fmov @r5,fr0 ; fcmp/gt fr1,fr0 ; bf 0x8c1b79ae
//! ; key(r5) > key(r6): r6->prev=r1, r6->next=r5, r5->prev=r6,
//! ;   rts ; r1->next=r6 (delay)   — r0 keeps the head loaded at entry
//! ```
//! T ends 1 on every path. `r5` walks to the node before which `r6` lands
//! (0 on the tail-insert path); `r1` is the predecessor (sentinel when the
//! list was empty or the insert lands before the head).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sentinel = cpu.r[5];
    let node = cpu.r[6];
    let head = mem.read_u32(sentinel.wrapping_add(8)).unwrap_or(0);
    cpu.r[0] = head;
    cpu.set_t(head == 0);
    if head == 0 {
        // bf.s not taken; its delay slot still ran: r1 = r5.
        cpu.r[1] = sentinel;
        mem.write_u32(node + 4, sentinel).ok();
        cpu.r[0] = 0;
        mem.write_u32(node + 8, 0).ok();
        // rts delay slot
        mem.write_u32(sentinel + 8, node).ok();
        cpu.set_t(true);
        return;
    }
    // bf.s taken (delay: r1 = r5), then bra with delay fmov @r6,fr1.
    cpu.r[1] = sentinel;
    cpu.fr[1] = mem.read_f32(node).unwrap_or(0.0);
    let mut r5 = head;
    loop {
        // 0x8c1b79c0: compare current key against the new node's key.
        let fr0 = mem.read_f32(r5).unwrap_or(0.0);
        cpu.fr[0] = fr0;
        let t = fr0 > cpu.fr[1];
        cpu.set_t(t);
        if !t {
            // 0x8c1b79ae: advance towards the tail.
            cpu.r[1] = r5;
            r5 = mem.read_u32(r5.wrapping_add(8)).unwrap_or(0);
            cpu.set_t(r5 == 0);
            if r5 != 0 {
                continue;
            }
            // Tail insert (0x8c1b79b6): r5 is already 0 in r5's final state.
            mem.write_u32(node + 4, cpu.r[1]).ok();
            cpu.r[0] = 0;
            mem.write_u32(node + 8, 0).ok();
            let r1 = cpu.r[1];
            // rts delay slot
            mem.write_u32(r1 + 8, node).ok();
            cpu.r[5] = 0;
            cpu.set_t(true);
            return;
        }
        // Insert before r5 (0x8c1b79c6).
        let r1 = cpu.r[1];
        mem.write_u32(node + 4, r1).ok();
        mem.write_u32(node + 8, r5).ok();
        mem.write_u32(r5 + 4, node).ok();
        // rts delay slot
        mem.write_u32(r1 + 8, node).ok();
        cpu.r[5] = r5;
        return; // T already 1 from fcmp/gt; r0 keeps the head value
    }
}
