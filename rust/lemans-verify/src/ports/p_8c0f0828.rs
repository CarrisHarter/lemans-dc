//! 0x8c0f0828 — *(r4+0x90) += r5; *(r4+0x94) += r6 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x90;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0).wrapping_add(cpu.r[5]);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1]);
    cpu.r[0] = 0x94;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0).wrapping_add(cpu.r[6]);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1]); // rts delay slot
}
