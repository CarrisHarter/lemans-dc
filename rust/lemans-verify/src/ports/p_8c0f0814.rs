//! 0x8c0f0814 — *(r4+0x90) = 0, *(r4+0x94) = 0 (rts delay); r1 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x90;
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1]);
    cpu.r[0] = 0x94;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1]); // rts delay slot
}
