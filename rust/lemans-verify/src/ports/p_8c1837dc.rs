//! 0x8c1837dc — struct init, pool 0x8c20df70 single deref.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    cpu.r[6] = s;
    cpu.r[0] = mem.read_u32(0x8c20df70).unwrap_or(0);
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
