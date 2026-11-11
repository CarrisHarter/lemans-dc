//! 0x8c21d8c2 — r2=0x8c23d450; r3=0; *r2=r3; rts delay r0=r3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x8c23d450;
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[2], cpu.r[3]);
    cpu.r[0] = cpu.r[3]; // rts delay
}
