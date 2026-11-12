//! 0x8c23b9b4 — list insert; rts delay *(r6+8)=r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.r[3] == 0 {
        let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
        let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[6]);
        cpu.r[3] = 0; // bra delay
    } else {
        cpu.r[4] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[5]);
        let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[6]);
        cpu.r[3] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    }
    let _ = mem.write_u32(cpu.r[5] + 8, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[6] + 8, cpu.r[5]); // rts delay
}
