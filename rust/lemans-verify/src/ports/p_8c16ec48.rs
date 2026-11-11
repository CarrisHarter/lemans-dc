//! 0x8c16ec48 — r4+=92; r3=*r4; byte[r3+0x80]=r5; r0=*r4+0x84; rts delay *r0=fr4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = 0x80;
    cpu.r[1] = 0x84;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[5] as u8);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits()); // rts delay
}
