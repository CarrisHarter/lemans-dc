//! 0x8c026c90 — unlink a doubly-linked node (list_del with tagged pointers).
//!
//! Node layout: +0 next, +4 prev, +8 back-pointer whose low bit is a flag.
//! If *r5 == r4 then *r5 is set to next; node->next = next->prev; the
//! prev-side slot at next->prev+8 and the +8 field of next are re-tagged
//! with each other's flag bits; the back-pointer at ([r4+8] & ~1) is
//! redirected to the node (at +0 or +4 depending on whether it already
//! points at r4); finally node+8 receives the new next OR the old flag.
//!
//! Side effects the C hides: r6 (next) ends ORed with the old flag bit of
//! [r4+8]; r3 ends 0xfffffffe (the and-mask literal is loaded
//! unconditionally at 0x8c026cb2); r1 ends ([r4+8]>>1)<<1; r0 ends
//! [r4+8]&1; T is the final cmp/eq of [([r4+8]&~1)] against r4 — the
//! shlr/shll T bits are dead by then, and that cmp/eq selects which
//! back-pointer slot gets written.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let node = cpu.r[4];
    let next = mem.read_u32(node).unwrap_or(0); // bf.s delay slot 0x8c026c96
    let head = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t(head == node); // 0x8c026c92 cmp/eq r4,r0
    if head == node {
        let _ = mem.write_u32(cpu.r[5], next); // 0x8c026c98
    }
    cpu.r[6] = next;
    let prev = mem.read_u32(next.wrapping_add(4)).unwrap_or(0); // 0x8c026c9a @(4,r6),r0
    cpu.r[0] = prev;
    let _ = mem.write_u32(node, prev); // 0x8c026c9c: node->next = next->prev
    let back = prev; // 0x8c026c9e reloads the same field into r3
    if back != 0 {
        let slot = back.wrapping_add(8);
        let flag = mem.read_u32(slot).unwrap_or(0) & 1;
        cpu.r[0] = flag;
        cpu.r[1] = node | flag;
        let _ = mem.write_u32(slot, cpu.r[1]);
    }
    // 0x8c026cb0..0x8c026cbc: next+8 = ([r4+8] & ~1) | ([next+8] & 1)
    let nflag = mem.read_u32(next.wrapping_add(8)).unwrap_or(0) & 1;
    let mut r1 = mem.read_u32(node.wrapping_add(8)).unwrap_or(0) & 0xffff_fffe;
    r1 |= nflag;
    cpu.r[0] = nflag;
    cpu.r[1] = r1;
    let _ = mem.write_u32(next.wrapping_add(8), r1);
    cpu.r[3] = 0xffff_fffe; // 0x8c026cb2 literal pool load
    // 0x8c026cbe..0x8c026cc2: shlr r1 ; shll r1 — r1 = [r4+8] & ~1
    let bp = mem.read_u32(node.wrapping_add(8)).unwrap_or(0) & 0xffff_fffe;
    cpu.r[1] = bp;
    cpu.r[0] = mem.read_u32(bp).unwrap_or(0);
    cpu.set_t(cpu.r[0] == node); // 0x8c026cc6 cmp/eq r0,r4
    if cpu.r[0] == node {
        let _ = mem.write_u32(bp, next); // bra delay slot 0x8c026ccc
    } else {
        let _ = mem.write_u32(bp.wrapping_add(4), next); // 0x8c026cce
    }
    let _ = mem.write_u32(next.wrapping_add(4), node); // 0x8c026cd0
    let flag = mem.read_u32(node.wrapping_add(8)).unwrap_or(0) & 1;
    cpu.r[0] = flag;
    cpu.r[6] = next | flag;
    let _ = mem.write_u32(node.wrapping_add(8), cpu.r[6]); // rts delay slot 0x8c026cda
}
