//! 0x8c0b274c — merge: r5 (3 words) → r4+16, r6 (4 words) → r4, fr4 → r4+28, fr5 → r4+32.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[7] = cpu.r[4].wrapping_add(16);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[4].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[7], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[7] + 4, cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[7] + 8, cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[6] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[6] + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits()); // fmov fr4,@r0 (r0=r4+28)
    cpu.r[4] = cpu.r[4].wrapping_add(32);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[5].to_bits()); // rts delay: fmov fr5,@r4
}
