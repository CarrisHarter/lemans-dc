//! 0x8c2259a4 - `[record+40] + 32*r5`, record = `0x8c2c02b8 + 0x148*r4`.
//!
//! The `shll` of `r5*16` sets T from bit 31 of that intermediate. `mul.l`
//! writes both MAC halves. The add is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u16(0x8c225a10).unwrap_or(0x0148) as i16 as i32 as u32;
    cpu.r[0] = cpu.r[5];
    cpu.r[2] = mem.read_u32(0x8c225a14).unwrap_or(0x8c2c02b8);
    cpu.r[0] <<= 2; // shll2
    let product = (cpu.r[3] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r3,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[0] <<= 2; // shll2
    cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
    cpu.r[0] <<= 1;
    cpu.r[4] = cpu.macl;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[2]);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // rts delay
}
