//! 0x8c0dfb04 — *(r4+0x190) = 0 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x190;
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1]);
}
