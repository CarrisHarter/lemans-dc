//! 0x8c0616bc — r0 := *(u32*)(0x268 + r4) (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x268u32.wrapping_add(cpu.r[4])).unwrap_or(0);
}
