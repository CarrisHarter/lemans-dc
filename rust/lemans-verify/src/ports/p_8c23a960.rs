//! 0x8c23a960 — init struct at r4: +16=r6, +20=r5|0x11000000, +12=r7, +8=0, +4=8, w+28=0x8000, w+30=0 (rts delay); r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x11000000;
    cpu.r[1] = cpu.r[4];
    cpu.r[0] = 0x8000;
    cpu.r[4] = 0;
    cpu.r[2] = 8;
    let b = cpu.r[1];
    let _ = mem.write_u32(b + 16, cpu.r[6]);
    cpu.r[5] |= cpu.r[3];
    let _ = mem.write_u32(b + 20, cpu.r[5]);
    let _ = mem.write_u32(b + 12, cpu.r[7]);
    let _ = mem.write_u32(b + 8, cpu.r[4]);
    let _ = mem.write_u32(b + 4, cpu.r[2]);
    let _ = mem.write_u16(b + 28, cpu.r[0] as u16);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u16(b + 30, cpu.r[0] as u16); // rts delay
}
