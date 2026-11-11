//! 0x8c237640 — r3=0x8c2c9db8; rts delay r0=*r3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x8c2c9db8;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0); // rts delay
}
