//! 0x8c164430 — r0 = (*r5)*8 + 0x8c1dd008 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = 0x8c1dd008;
    cpu.r[0] = (cpu.r[0] << 2) << 1; // shll2 + shll = *8
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // rts delay slot
}
