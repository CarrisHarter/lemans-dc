//! 0x8c0c01fc — *r4 = -1, byte at r4+4 = 0, words at r4+8/+12 = 0; r0 = r4 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = (-1i32) as u32;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4] + 4, 0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[0]);
    cpu.r[0] = cpu.r[4]; // rts delay slot
}
