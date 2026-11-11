//! 0x8c1060d4 — r0 = *r4 + 16; rts delay fr0 = *(r0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(16);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // rts delay
}
