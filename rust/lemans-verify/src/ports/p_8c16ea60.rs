//! 0x8c16ea60 — if *(r4+8) & 1: r4 += 92, fr0 = *(float*)(*(r4)+96) (rts delay); else fr0 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[0] &= 1;
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.fr[0] = 0.0; // fldi0 fr0
        return;
    }
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(96);
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // rts delay
}
