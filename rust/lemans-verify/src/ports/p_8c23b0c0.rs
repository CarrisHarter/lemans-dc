//! 0x8c23b0c0 — r5=0x8c2c8a80; r6=r4*8; r2=r5+8+r6; r3=*(r5+4); *r2=r3; *(r5+8+r6+4)=0; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = 0x8c2c8a80;
    cpu.r[6] = cpu.r[4] << 3;
    cpu.r[2] = cpu.r[5].wrapping_add(8);
    cpu.r[3] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[6]);
    let _ = mem.write_u32(cpu.r[2], cpu.r[3]);
    cpu.r[2] = cpu.r[5].wrapping_add(8);
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[2]);
    cpu.r[4] = 0;
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]);
    cpu.r[0] = cpu.r[4]; // rts delay
}
