//! 0x8c146730 — *(r4 + r5*4) = r6; *(r4+0xa8) += 1 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[0] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[6]);
    cpu.r[0] = 0xa8;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1]); // rts delay
}
