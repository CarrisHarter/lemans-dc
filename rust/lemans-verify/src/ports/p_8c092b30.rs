//! 0x8c092b30 — r4 += 0x84; fr0 = 0; rts delay *(r4) = fr0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x84;
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[0].to_bits()); // rts delay slot
}
