//! 0x8c0252cc — walk a binary search tree, store the result through [r15].
//!
//! From *(r4+4), each step compares *r5 (the key) with node[3]: key <
//! node[3] goes left (node[0]), otherwise the node is recorded in r3 and
//! the walk goes right (node[4]). After the walk, r3 is stored if
//! key <= r3[3], otherwise r4+4 — through the pointer at [r15], which is
//! loaded into r0 (the C signature's third argument actually arrives on
//! the stack).
//!
//! Side effects the C hides: r3 = 0 is set in the bt.s delay slot even when
//! the root is null; r4 advances by 4 ONLY on the r4+4 store path; r6 ends
//! 0; r1/r0 hold the last loop loads, or the final best[3]/key pair, or
//! just the key and [r15] when best is null; T is the final cmp/gt
//! (key > best[3]) or 0 when best is null.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut node = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[6] = node;
    cpu.r[3] = 0; // bt.s delay slot 0x8c0252d2 runs either way
    cpu.set_t(node == 0); // 0x8c0252ce tst r6,r6
    while node != 0 {
        cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0); // key
        cpu.r[0] = mem.read_u32(node.wrapping_add(12)).unwrap_or(0);
        let t = (cpu.r[0] as i32) > (cpu.r[1] as i32); // cmp/gt: node[3] > key
        cpu.set_t(t);
        if t {
            node = mem.read_u32(node).unwrap_or(0); // bra delay slot 0x8c0252de
        } else {
            cpu.r[3] = node; // 0x8c0252e0
            node = mem.read_u32(node.wrapping_add(4)).unwrap_or(0); // 0x8c0252e2
        }
        cpu.r[6] = node;
        cpu.set_t(node == 0); // 0x8c0252e4 tst r6,r6
    }
    let best = cpu.r[3];
    cpu.set_t(best == 0); // 0x8c0252e8 tst r3,r3
    if best != 0 {
        cpu.r[1] = mem.read_u32(best.wrapping_add(12)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        let key_gt = (cpu.r[0] as i32) > (cpu.r[1] as i32); // key > best[3]
        cpu.set_t(key_gt);
        if !key_gt {
            // bf 0x8c0252fc: store best, r4 untouched
            let out = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[0] = out;
            let _ = mem.write_u32(out, best);
            return;
        }
    }
    // 0x8c0252f4: add #4,r4 precedes the rts delay-slot store of r4
    let out = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = out;
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(out, cpu.r[4]);
}
