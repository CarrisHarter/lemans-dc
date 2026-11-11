//! 0x8c017844 — push a 3-word node onto the singly linked list whose head
//! pointer lives at the literal cell 0x8c01785c: node[0] = old head,
//! node[1] = r5, node[2] = r4, head = node; returns r4.
//!
//! The head pointer is loaded from memory at runtime (0x8c23da58 in the
//! ROM, redirected by the spec). Hidden side effects: r0 = r4 (return),
//! r1 keeps the old head value, r3 keeps the head-pointer cell address,
//! and the store order is node[0], node[1], node[2], then head.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r3 = mem.read_u32(0x8c01785c).unwrap_or(0);
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let r1 = mem.read_u32(r3).unwrap_or(0); // mov.l @r3,r1
    let node = cpu.r[6];
    let _ = mem.write_u32(node, r1); // mov.l r1,@r6
    let _ = mem.write_u32(node.wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r6)
    let _ = mem.write_u32(node.wrapping_add(8), cpu.r[4]); // mov.l r4,@(8,r6)
    let _ = mem.write_u32(r3, node); // mov.l r6,@r3
    cpu.r[1] = r1;
    cpu.r[3] = r3;
}
