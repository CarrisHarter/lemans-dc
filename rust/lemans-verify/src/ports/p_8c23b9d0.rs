//! 0x8c23b9d0 — unlink_node.
//!
//! Unlinks the node described by `r6` from a doubly linked list, writing the
//! back/forward pointers through `r4` and `r5` when the neighbours are null.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[7] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
    let t = cpu.r[7] == 0;
    cpu.set_t(t);
    let _ = mem.write_u32(cpu.r[15], cpu.r[7]);
    cpu.r[3] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15] + 4, cpu.r[3]); // delay slot of `bf.s`
    if t {
        cpu.r[1] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    } else {
        cpu.r[3] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[2] + 8, cpu.r[3]);
    }
    cpu.r[2] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let t2 = cpu.r[2] == 0;
    cpu.set_t(t2);
    if t2 {
        cpu.r[2] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[5], cpu.r[2]);
    } else {
        cpu.r[3] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[15] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[1] + 4, cpu.r[3]);
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8); // delay slot of `rts`
}
