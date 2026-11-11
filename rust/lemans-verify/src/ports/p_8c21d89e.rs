//! 0x8c21d89e — r3=0xff800030; r2=0x300; r4=sext16(*r3); r0=extu16(r4); rts delay r0&=r2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0xff800030;
    cpu.r[2] = 0x300;
    cpu.r[4] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as u32;
    cpu.r[0] = cpu.r[4] & 0xffff;
    cpu.r[0] &= cpu.r[2]; // rts delay
}
