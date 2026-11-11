//! 0x8c09d598 — init_node_1.
//!
//! Builds a four-word node: the global at `[0x8C09_D5C0]` at `[r4]`, zero at
//! `[r4+4]`, the byte from `[r5]` at `[r4+8]`, and `r4+4` at `[r4+12]` (the
//! zero written there first is overwritten). `r0` ends as the node, `r1` as
//! `node+4`, `r3` as `node+8`, `r4` as 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let node = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8C09_D5C0).unwrap_or(0);
    cpu.r[4] = 0;
    cpu.r[6] = node;
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = node;
    cpu.r[1] = node;
    let _ = mem.write_u32(node, cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    let _ = mem.write_u32(node + 4, 0);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
    let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8);
    let _ = mem.write_u32(node + 12, 0);
    cpu.r[0] = node;
    let _ = mem.write_u32(node + 12, cpu.r[1]);
}
