//! 0x8c0c6904 — linked-list lookup by node value.
//!
//! ```text
//! tst r5,r5 ; bt 0x8c0c6928            ; r5 == 0 → walk-to-last path
//! mov.l @(12,r4),r0 ; cmp/eq r0,r5 ; bf 0x8c0c6912
//! rts ; mov #0,r0 (delay)              ; head == r5 → return 0
//! 0x8c0c6912: tst r0,r0 ; bt 0x8c0c693a ; r1 = mem[r0+8]
//!   cmp/eq r5,r1 ; bt 0x8c0c693a       ; found → rts ; nop
//!   0x8c0c691c: r0 = mem[r0+8] ; r1 = mem[r0+8] ; cmp/eq r5,r1 ; bf 0x8c0c691c
//!   rts ; nop
//! 0x8c0c6928: r0 = mem[r4+12] ; tst r0,r0 ; bt 0x8c0c693a
//!   bra 0x8c0c6934 ; nop
//!   0x8c0c6932: mov r1,r0
//!   0x8c0c6934: r1 = mem[r0+8] ; tst r1,r1 ; bf 0x8c0c6932
//! 0x8c0c693a: rts ; nop
//! ```
//! Hidden side effects: r0 ends as 0 (head match), the predecessor node
//! (search match), 0 (empty list), or the last node (r5 == 0 walk); r1 ends
//! as the seed, r5, the seed, or 0 respectively; every exit leaves T = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        // 0x8c0c6928: walk to the last node
        let mut r0 = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap();
        cpu.r[0] = r0;
        cpu.set_t(r0 == 0); // tst r0,r0
        if r0 == 0 {
            return; // bt 0x8c0c693a ; rts ; nop — r1 keeps its seed
        }
        let mut r1 = mem.read_u32(r0.wrapping_add(8)).unwrap(); // bra → 0x8c0c6934
        loop {
            cpu.r[1] = r1;
            cpu.set_t(r1 == 0); // 0x8c0c6936: tst r1,r1
            if r1 == 0 {
                break; // bf not taken → 0x8c0c693a ; rts ; nop
            }
            r0 = r1; // 0x8c0c6932: mov r1,r0
            r1 = mem.read_u32(r0.wrapping_add(8)).unwrap(); // 0x8c0c6934
        }
        cpu.r[0] = r0;
        cpu.r[1] = r1;
        return;
    }
    // r5 != 0
    let mut r0 = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap();
    cpu.r[0] = r0;
    cpu.set_t(r0 == cpu.r[5]); // cmp/eq r0,r5
    if r0 == cpu.r[5] {
        cpu.r[0] = 0; // rts ; mov #0,r0 (delay)
        return;
    }
    cpu.set_t(r0 == 0); // 0x8c0c6912: tst r0,r0
    if r0 == 0 {
        return; // bt 0x8c0c693a ; rts ; nop — r1 keeps its seed
    }
    let mut r1 = mem.read_u32(r0.wrapping_add(8)).unwrap(); // 0x8c0c6916
    loop {
        cpu.r[1] = r1;
        cpu.set_t(r1 == cpu.r[5]); // cmp/eq r5,r1
        if r1 == cpu.r[5] {
            cpu.r[0] = r0; // bt 0x8c0c693a ; rts ; nop
            return;
        }
        r0 = mem.read_u32(r0.wrapping_add(8)).unwrap(); // 0x8c0c691c
        r1 = mem.read_u32(r0.wrapping_add(8)).unwrap(); // 0x8c0c691e
    }
}
