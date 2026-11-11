//! 0x8c0b4ed0 — *(r3) = r4[0]+r5[0]; rts delay *(r3+4) = r4[1]+r5[1]; r3 += 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let a0 = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0));
    cpu.r[1] = cpu.r[4].wrapping_add(4);
    let b0 = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap_or(0));
    cpu.r[0] = cpu.r[5].wrapping_add(4);
    cpu.fr[1] = a0 + b0; // fadd fr0,fr1
    let a1 = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0));
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
    let _ = mem.write_u32(cpu.r[3], cpu.fr[1].to_bits());
    cpu.fr[2] = a1 + cpu.fr[0];
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[3], cpu.fr[2].to_bits()); // rts delay slot
}
