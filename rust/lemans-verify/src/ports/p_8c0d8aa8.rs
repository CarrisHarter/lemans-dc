//! 0x8c0d8aa8 — [r4+4/+8/+12]=0, [r4+16]=-1, [r4+20]=0 (rts delay); r1=0, r0=-1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[0] = (-1i32) as u32;
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4] + 20, cpu.r[1]); // rts delay slot
}
