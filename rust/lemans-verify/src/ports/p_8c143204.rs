//! 0x8c143204 — *(r4+20) = fr4; r4 += 24; *(r4) = fr5 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(20);
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits());
    cpu.r[4] = cpu.r[4].wrapping_add(24);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[5].to_bits()); // rts delay slot
}
