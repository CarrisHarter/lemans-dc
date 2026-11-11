//! 0x8c1c3a2c — r0 = *r4 + 8; r4 += 12; fr0 = *(float*)r4; rts delay *(r0) = fr0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0));
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[0], cpu.fr[0].to_bits()); // rts delay slot
}
