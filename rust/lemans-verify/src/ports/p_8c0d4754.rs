//! 0x8c0d4754 — byte[r4+61]=0, *(r4+80)=1.0f, r0=3 stored at r4+56 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 61;
    cpu.r[3] = 0;
    cpu.r[1] = cpu.r[4];
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), 0);
    cpu.r[1] = cpu.r[1].wrapping_add(80);
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits());
    cpu.r[0] = 3;
    let _ = mem.write_u32(cpu.r[4] + 56, cpu.r[0]); // rts delay slot
}
