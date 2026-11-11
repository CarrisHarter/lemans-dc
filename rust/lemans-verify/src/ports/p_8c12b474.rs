//! 0x8c12b474 — same as 0x8c12b3f8 with array base (*r4)+44.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(44);
    cpu.r[1] = mem.read_u32(cpu.r[0] + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0);
    cpu.r[3] = cpu.r[1];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    loop {
        cpu.set_t(cpu.r[3] == cpu.r[1]);
        if cpu.r[3] == cpu.r[1] {
            cpu.r[0] = 0;
            return;
        }
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[5]);
        if cpu.r[0] == cpu.r[5] {
            cpu.r[0] = 1;
            return;
        }
        cpu.r[1] = cpu.r[1].wrapping_add(4);
    }
}
