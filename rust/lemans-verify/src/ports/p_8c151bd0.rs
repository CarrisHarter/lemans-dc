//! 0x8c151bd0 — *(r4+88) = fr4; r4 += 92; *(r4) = 1.0/fr4 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(88);
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits());
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.fr[0] = cpu.fr[0] / cpu.fr[4]; // fdiv fr4,fr0
    let _ = mem.write_u32(cpu.r[4], cpu.fr[0].to_bits()); // rts delay slot
}
