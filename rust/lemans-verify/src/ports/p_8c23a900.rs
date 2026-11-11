//! 0x8c23a900 — struct init at r4; rts delay w(r4+30)=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x10000000;
    cpu.r[1] = cpu.r[4];
    let b = cpu.r[1];
    let _ = mem.write_u32(b + 48, cpu.r[5]);
    let _ = mem.write_u32(b + 16, cpu.r[6]);
    let _ = mem.write_u32(b + 20, cpu.r[3]);
    cpu.r[3] = 9;
    let _ = mem.write_u32(b + 12, cpu.r[7]);
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(b + 8, cpu.r[2]);
    let _ = mem.write_u32(b + 4, cpu.r[3]);
    cpu.r[0] = mem.read_u16(cpu.r[15] + 4).unwrap_or(0) as u32;
    cpu.r[3] = 0x8000;
    cpu.r[0] |= cpu.r[3];
    let _ = mem.write_u16(b + 28, cpu.r[0] as u16);
    cpu.r[0] = 0;
    let _ = mem.write_u16(b + 30, cpu.r[0] as u16); // rts delay
}
