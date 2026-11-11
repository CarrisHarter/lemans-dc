//! 0x8c2268b4 — r3=0x8c2c65c8; *r3=r4; rts delay r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x8c2c65c8;
    let _ = mem.write_u32(cpu.r[3], cpu.r[4]);
    cpu.r[0] = cpu.r[4]; // rts delay
}
