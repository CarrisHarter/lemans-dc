//! 0x8c216bd4 — r0 = ((0xf0 & sext8(r4)) >>a 4); r0 = r0*5 << 1; rts delay r0 += 15 & sext8(r4).
//! `and`/`add`/`shll2`/`shad` leave T alone; the trailing `shll r0` sets it
//! from the shifted-out bit 31 (always clear here, so the function exits T=0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = 0xf0;
    cpu.r[5] = cpu.r[4] as i8 as u32;
    cpu.r[3] = 0xfffffffc;
    cpu.r[0] &= cpu.r[5];
    cpu.r[0] = ((cpu.r[0] as i32) >> 4) as u32; // shad -4
    cpu.r[3] = cpu.r[0];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[2] = 15;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.r[2] &= cpu.r[5];
    cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0 sets T from the old bit 31
    cpu.r[0] = cpu.r[0] << 1;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]); // rts delay
}
