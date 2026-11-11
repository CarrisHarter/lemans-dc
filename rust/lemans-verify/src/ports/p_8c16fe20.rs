//! 0x8c16fe20 — fr4→r4+12; r4+=8; rts delay fr4→r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits());
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // rts delay
}
