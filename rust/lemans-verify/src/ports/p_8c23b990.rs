//! 0x8c23b990 — list insert; bare rts;nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    if cpu.r[2] == 0 {
        cpu.r[7] = 0;
        let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[7]);
        let _ = mem.write_u32(cpu.r[6] + 8, cpu.r[7]);
        let _ = mem.write_u32(cpu.r[4], cpu.r[6]);
        let _ = mem.write_u32(cpu.r[5], cpu.r[6]); // bra delay
    } else {
        cpu.r[5] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[2]);
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6] + 8, cpu.r[3]);
        let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[6]);
        let _ = mem.write_u32(cpu.r[4], cpu.r[6]);
    }
}
