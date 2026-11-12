//! 0x8c22f2e0 — init struct at 0x8c2c6600, link into *(r4+36); rts delay add #4,r15.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[5] = 0x8c2c6600;
    cpu.r[7] = 0;
    cpu.r[3] = 0xb0u32.wrapping_add(cpu.r[5]);
    cpu.r[6] = mem.read_u32(cpu.r[4] + 36).unwrap_or(0);
    cpu.r[1] = cpu.r[5];
    cpu.r[2] = 0xb0u32.wrapping_add(cpu.r[5]);
    let p = cpu.r[5];
    let _ = mem.write_u32(p + 64, cpu.r[7]);
    let _ = mem.write_u32(p + 60, cpu.r[7]);
    let _ = mem.write_u32(p + 56, cpu.r[7]);
    let _ = mem.write_u32(p + 68, cpu.r[7]);
    let _ = mem.write_u32(p + 52, cpu.r[7]);
    let _ = mem.write_u32(p + 76, cpu.r[7]);
    let _ = mem.write_u32(cpu.r[6], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[6] + 16, cpu.r[2]);
    let r3 = cpu.r[5].wrapping_add(112);
    let _ = mem.write_u32(cpu.r[15], r3);
    let r2 = mem.read_u32(cpu.r[15]).unwrap_or(0) & 0xffffffe0;
    let _ = mem.write_u32(cpu.r[6] + 4, r2);
    let r1 = cpu.r[1].wrapping_add(52);
    let _ = mem.write_u32(cpu.r[6] + 8, r1);
    let r2b = 0x150u32.wrapping_add(cpu.r[5]);
    let _ = mem.write_u32(cpu.r[6] + 12, r2b);
    cpu.r[1] = cpu.r[4].wrapping_add(56);
    let r1b = cpu.r[1];
    let _ = mem.write_u32(cpu.r[6] + 24, r1b);
    cpu.r[0] = cpu.r[7]; // mov r7,r0
    cpu.r[2] = r2b; cpu.r[3] = 0xffffffe0;
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay
}
