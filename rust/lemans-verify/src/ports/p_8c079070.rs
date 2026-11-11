//! 0x8c079070 — *(r4+68) = r5 (rts delay slot); r0 = 68.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 68;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // rts delay slot
}
