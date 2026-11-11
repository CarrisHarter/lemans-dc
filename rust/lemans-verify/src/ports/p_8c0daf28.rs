//! 0x8c0daf28 — r4 += 0x5bc; rts delay *(r4) = fr4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x5bc;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // rts delay slot
}
