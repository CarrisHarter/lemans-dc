//! 0x8c21e11a — r2=0xffd8000c; r0=-1; r3=*r2; rts delay r0-=r3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0xffd8000c;
    cpu.r[0] = 0xffffffff;
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[3]); // rts delay
}
