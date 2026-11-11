//! 0x8c0daf34 — r4 += 0x5bc; rts delay fr0 = *(r4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x5bc;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0)); // rts delay
}
