//! 0x8c13d084 — r4 += 92; r0 = *r4; if zero → r0 = 0 (rts delay), else r0 = *(r0+12).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.r[0] = 0; // rts delay slot
        return;
    }
    cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0);
}
