//! 8c1379c8 — dispatch with count at r4+0xbc, table at r4+0xc0, OOB global 0x8c208170.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0xbc;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.set_t((cpu.r[5] as i32) >= (cpu.r[0] as i32));
    if (cpu.r[5] as i32) >= (cpu.r[0] as i32) {
        cpu.r[0] = 0x8c208170;
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        return;
    }
    cpu.r[0] = 0xc0;
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0);
}
