//! 0x8c1cbda8 — r5==1 → r0 = *(r4+32); else r0 = *(r4+36).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 1);
        if cpu.r[0] == 1 {
            cpu.r[0] = mem.read_u32(cpu.r[4] + 32).unwrap_or(0);
            return;
        }
        cpu.set_t(cpu.r[0] == 2);
    }
    cpu.r[0] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0);
}
