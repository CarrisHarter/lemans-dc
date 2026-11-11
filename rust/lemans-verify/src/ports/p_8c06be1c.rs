//! 0x8c06be1c — unlink a node from a doubly linked list.
//!
//! ```text
//! r3 = node->back
//! if (r3 == 0) *out = node->next
//! else         r3->next = node->next
//! r3 = node->next
//! if (r3 != 0) r3->back = node->back
//! ```
//!
//! Hidden side effects: `r4` is advanced by 72 on the second fixup, `r0` ends
//! as 72 when that fixup ran and as node+0x44 otherwise, `r1` holds the value
//! last stored (node->next on the first path, node->back on the second), and
//! T is the second `tst` — 1 exactly when the node had no successor.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let node = cpu.r[4];
    let out = cpu.r[5];

    let back = mem.read_u32(node + 0x48).unwrap_or(0);
    cpu.r[3] = back;
    cpu.set_t(back == 0);

    if back == 0 {
        cpu.r[0] = node.wrapping_add(0x44);
        let next = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = next;
        let _ = mem.write_u32(out, next);
    } else {
        cpu.r[0] = node.wrapping_add(0x44);
        let next = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[1] = next;
        cpu.r[0] = 68;
        let _ = mem.write_u32(back + 68, next); // delay slot of `bra`
    }

    cpu.r[0] = node.wrapping_add(0x44);
    let next = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = next;
    cpu.set_t(next == 0);

    if next != 0 {
        cpu.r[4] = node.wrapping_add(72);
        cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = 72;
        let _ = mem.write_u32(next + 72, cpu.r[1]);
    }
}
