//! 0x8c0e28c0 — r0 = *(*[r4+8]+4) + 0x240 (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[1] = 0x240;
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // rts delay slot
}
