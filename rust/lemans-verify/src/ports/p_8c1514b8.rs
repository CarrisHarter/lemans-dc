//! 0x8c1514b8 — fr0 = *r5; r4 += 28; rts delay *(r4) = fr0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap_or(0));
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[0].to_bits()); // rts delay
}
