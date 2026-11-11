//! 0x8c14fe68 — r0 = *(r4+4) + 12; rts delay r0 = *(r0+4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0); // rts delay
}
