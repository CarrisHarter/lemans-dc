//! 0x8c0b4fd4 — r0 = r4+4; fr0 = 0; *(r0) = fr0; rts delay *(r4) = fr0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(4);
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[0].to_bits());
    let _ = mem.write_u32(cpu.r[4], cpu.fr[0].to_bits()); // rts delay slot
}
