//! 0x8c156f8c — init_node.
//!
//! Builds a four-word node at `r4`: `[0]=*[0x8c209eb0]`, `[4]=0`, `[8]=byte[r5]`,
//! `[12]=r4+4`. `r0` ends as the node and the stack is restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[0] = mem.read_u32(0x8c156fb4).unwrap_or(0);
    cpu.r[4] = 0;
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[6];
    cpu.r[1] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
    let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8);
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[4]);
    cpu.r[0] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[1]);
    cpu.r[15] = cpu.r[15].wrapping_add(4); // delay slot of `rts`
}
