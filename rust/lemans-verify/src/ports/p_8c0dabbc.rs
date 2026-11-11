//! 0x8c0dabbc — r0 = *(r4+0x5d8) (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 0x5d8).unwrap_or(0); // rts delay slot
}
