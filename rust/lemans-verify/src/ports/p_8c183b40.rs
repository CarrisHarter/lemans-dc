//! 0x8c183b40 — copy 4 words from r5 to r4; r6 = r5+20, r7 = r4+20, r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[6] = cpu.r[5].wrapping_add(20);
    cpu.r[7] = cpu.r[4].wrapping_add(20);
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[7], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[7] + 4, cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[7] + 8, cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 16).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[1]); // rts delay slot
}
