//! 0x8c105f70 — r0 = r5*20 + *(r4+4) (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] & 0x8000_0000 != 0); // shll r0: T = bit 31 shifted out
    cpu.r[0] = cpu.r[5] << 1; // shll r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]);
    cpu.r[0] = cpu.r[0] << 2; // shll2 r0  => r5*20
    cpu.r[1] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // rts delay slot
}
