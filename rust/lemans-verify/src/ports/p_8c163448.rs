//! 0x8c163448 — r0 = 92; rts delay byte[r4+92] = r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 92;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[5] as u8); // rts delay
}
