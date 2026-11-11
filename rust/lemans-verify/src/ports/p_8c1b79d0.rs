//! 0x8c1b79d0 — sorted insert of node `r6` walking `prev` (+4) links from the
//! tail `r5` toward the head sentinel `r4` (+8 holds the head pointer).
//! Node layout: +0 float key, +4 prev, +8 next.
//!
//! Original:
//! ```text
//! mov.l @(4,r5),r0 ; tst r0,r0 ; bf.s 0x8c1b79e4 ; mov r5,r1   (delay)
//! ; empty: (r4+8)=r6, r0=0, (r6+4)=0, (r6+8)=r5, rts ; (r5+4)=r6 (delay)
//! 0x8c1b79e4: bra 0x8c1b79fc ; fmov @r6,fr1                  (delay)
//! 0x8c1b79e8: mov r5,r1 ; mov.l @(4,r5),r5 ; tst r5,r5 ; bf 0x8c1b79fc
//! ; reached head end: (r4+8)=r6, r0=0, (r6+4)=0, (r6+8)=r1,
//! ;   rts ; (r1+4)=r6 (delay)
//! 0x8c1b79fc: fmov @r5,fr0 ; fcmp/gt fr0,fr1 ; bf 0x8c1b79e8
//! ; key(r6) > key(r5): (r6+4)=r5, (r6+8)=r1, (r5+8)=r6,
//! ;   rts ; (r1+4)=r6 (delay)   — r0 keeps the value loaded at entry
//! ```
//! T ends 1 on every path. `r5` ends at the node after which `r6` lands
//! (0 on the head-end path); `r1` is the node on the tail side of the gap.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let tail = cpu.r[5];
    let node = cpu.r[6];
    let first = mem.read_u32(tail.wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = first;
    cpu.set_t(first == 0);
    if first == 0 {
        // bf.s not taken; its delay slot still ran: r1 = r5.
        cpu.r[1] = tail;
        mem.write_u32(r4 + 8, node).ok();
        cpu.r[0] = 0;
        mem.write_u32(node + 4, 0).ok();
        mem.write_u32(node + 8, tail).ok();
        // rts delay slot
        mem.write_u32(tail + 4, node).ok();
        cpu.set_t(true);
        return;
    }
    // bf.s taken (delay: r1 = r5), then bra with delay fmov @r6,fr1.
    cpu.r[1] = tail;
    cpu.fr[1] = mem.read_f32(node).unwrap_or(0.0);
    let mut r5 = tail;
    loop {
        // 0x8c1b79fc: fcmp/gt fr0,fr1 -> T = fr1 > fr0.
        let fr0 = mem.read_f32(r5).unwrap_or(0.0);
        cpu.fr[0] = fr0;
        let t = cpu.fr[1] > fr0;
        cpu.set_t(t);
        if !t {
            // 0x8c1b79e8: advance towards the head.
            cpu.r[1] = r5;
            r5 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
            cpu.set_t(r5 == 0);
            if r5 != 0 {
                continue;
            }
            // Head-end insert (0x8c1b79f0).
            mem.write_u32(r4 + 8, node).ok();
            cpu.r[0] = 0;
            mem.write_u32(node + 4, 0).ok();
            let r1 = cpu.r[1];
            mem.write_u32(node + 8, r1).ok();
            // rts delay slot
            mem.write_u32(r1 + 4, node).ok();
            cpu.r[5] = 0;
            cpu.set_t(true);
            return;
        }
        // Insert after r5 (0x8c1b7a02).
        let r1 = cpu.r[1];
        mem.write_u32(node + 4, r5).ok();
        mem.write_u32(node + 8, r1).ok();
        mem.write_u32(r5 + 8, node).ok();
        // rts delay slot
        mem.write_u32(r1 + 4, node).ok();
        cpu.r[5] = r5;
        return; // T already 1 from fcmp/gt; r0 keeps the entry value
    }
}
