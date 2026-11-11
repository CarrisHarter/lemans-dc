//! 0x8c217e60 — r2=0x8c23d428; r3=*r2; *(r3+48)=r4; rts delay r0=1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x8c23d428;
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3] + 48, cpu.r[4]);
    cpu.r[0] = 1; // rts delay
}
