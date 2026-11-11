//! 0x8c0cbba0 — *(r4+64)=fr4, *(r4+68)=fr5, fr5 *= fr5, *(r4+72)=fr5 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = cpu.r[4].wrapping_add(64);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[1], cpu.fr[4].to_bits());
    cpu.r[0] = cpu.r[0].wrapping_add(68);
    let _ = mem.write_u32(cpu.r[0], cpu.fr[5].to_bits());
    cpu.r[4] = cpu.r[4].wrapping_add(72);
    cpu.fr[5] = cpu.fr[5] * cpu.fr[5]; // fmul fr5,fr5
    let _ = mem.write_u32(cpu.r[4], cpu.fr[5].to_bits()); // rts delay slot
}
