//! 0x8c10689c — struct init like 0x8c05de64, pool at 0x8c203ea8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    cpu.r[6] = s;
    cpu.r[0] = mem.read_u32(0x8c203ea8).unwrap_or(0);
    cpu.r[4] = 0;
    cpu.r[3] = cpu.r[6];
    cpu.r[1] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    let b = mem.read_u8(cpu.r[5]).unwrap_or(0);
    cpu.r[0] = b as u32;
    let _ = mem.write_u8(cpu.r[3], b);
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[4]);
    cpu.r[0] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[1]);
}
