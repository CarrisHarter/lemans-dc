//! 0x8c1075c4 — r0 = 64; rts delay *(r4+64) = r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 64;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[5]); // rts delay
}
