//! 0x8c026ce0 — unlink a node from a doubly linked list.
//!
//! Original (r4 = node, r5 = pointer to the head cell):
//! `mov.l @r5,r0 ; cmp/eq r4,r0 ; bf.s … ; mov.l @(4,r4),r6 ; mov.l r6,@r5`
//! and so on. The decompiled C hides several register side effects:
//! the `bf.s` delay slot always loads `next` into r6, r3 ends as the
//! 0xfffffffe literal (loaded at 0x8c026d02), r1 ends as
//! `node->f8 & 0xfffffffe`, r0 as `node->f8 & 1`, r6 as
//! `next | (node->f8 & 1)`, and the final T bit comes from
//! `cmp/eq r0,r4`, not from the trailing `or`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let node = cpu.r[4];
    let cell = cpu.r[5];

    // `bf.s` delay slot: r6 = node->next always executes.
    let next = mem.read_u32(node + 4).unwrap();
    cpu.r[6] = next;

    if mem.read_u32(cell).unwrap() == node {
        let _ = mem.write_u32(cell, next); // mov.l r6,@r5
    }

    let next_val = mem.read_u32(next).unwrap();
    let _ = mem.write_u32(node + 4, next_val); // mov.l r0,@(4,r4)
    cpu.r[3] = next_val; // mov.l @r6,r3
    if next_val != 0 {
        // tst r3,r3 -> bt skips this block when next->val == 0
        cpu.r[3] = next_val.wrapping_add(8); // add #8,r3
        let p = cpu.r[3];
        cpu.r[0] = mem.read_u32(p).unwrap() & 1; // mov.l @r3,r0 ; and #1,r0
        cpu.r[1] = node | cpu.r[0]; // mov r4,r1 ; or r0,r1
        let _ = mem.write_u32(p, cpu.r[1]);
    }

    let next_f8 = mem.read_u32(next + 8).unwrap();
    let node_f8 = mem.read_u32(node + 8).unwrap();
    cpu.r[3] = mem.read_u32(0x8c026d2c).unwrap(); // mov.l 0x8c026d2c,r3 — r3 ends as 0xfffffffe
    cpu.r[0] = next_f8 & 1; // and #1,r0
    cpu.r[1] = node_f8 & 0xFFFF_FFFE; // and r3,r1
    let _ = mem.write_u32(next + 8, cpu.r[0] | cpu.r[1]);

    // shlr r1 ; shll r1 leaves r1 = node->f8 & 0xfffffffe
    let ptr = cpu.r[1];
    let head = mem.read_u32(ptr).unwrap();
    cpu.set_t(head == node); // cmp/eq r0,r4 — the last T-setting instruction
    if head == node {
        let _ = mem.write_u32(ptr, next); // delay slot of `bra`
    } else {
        let _ = mem.write_u32(ptr + 4, next); // mov.l r6,@(4,r1)
    }

    let _ = mem.write_u32(next, node); // mov.l r4,@r6
    cpu.r[0] = node_f8 & 1; // mov.l @(8,r4),r0 ; and #1,r0
    cpu.r[6] = next | cpu.r[0]; // or r0,r6
    let _ = mem.write_u32(node + 8, cpu.r[6]); // delay slot of `rts`
}
