//! 0x8c10606c — r0 = *r4 + 52; rts delay fr0 = *(r0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(52);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // rts delay
}
