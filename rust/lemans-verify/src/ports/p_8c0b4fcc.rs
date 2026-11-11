//! 0x8c0b4fcc — *(r4) = fr4; r4 += 4; rts delay *(r4) = fr5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits());
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[5].to_bits()); // rts delay slot
}
