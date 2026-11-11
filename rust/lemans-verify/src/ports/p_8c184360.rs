//! 0x8c184360 — fr0 chain: load [r4+0xdc]→*r5, [r4+0xe0]→*r6, [r4+0xe4]→*r7, r4+=0xe8, r0=*r15, rts delay fr0=[r4]→*r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0xdc;
    cpu.r[0] = cpu.r[4].wrapping_add(cpu.r[1]);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
    cpu.r[1] = 0xe0;
    let _ = mem.write_u32(cpu.r[5], cpu.fr[0].to_bits());
    cpu.r[0] = cpu.r[4].wrapping_add(cpu.r[1]);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
    cpu.r[1] = 0xe4;
    let _ = mem.write_u32(cpu.r[6], cpu.fr[0].to_bits());
    cpu.r[0] = cpu.r[4].wrapping_add(cpu.r[1]);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
    cpu.r[0] = 0xe8;
    let _ = mem.write_u32(cpu.r[7], cpu.fr[0].to_bits());
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[4]).unwrap_or(0));
    let _ = mem.write_u32(cpu.r[0], cpu.fr[0].to_bits()); // rts delay slot
}
