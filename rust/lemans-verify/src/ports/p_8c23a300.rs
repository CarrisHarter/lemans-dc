//! 0x8c23a300 — extract_five_fields.
//!
//! Extracts five 2-bit fields from `r4` into `[r5]` (bits 0-1), `[r6]` (4-5),
//! `[r7]` (8-9), `[[r15+4]]` (12-13) and `[[r15+8]]` (16-17); returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    let v = cpu.r[4];
    cpu.r[3] = v;
    cpu.r[14] = 3;
    cpu.r[2] = v;
    cpu.r[3] = cpu.r[3] & cpu.r[14];
    cpu.r[1] = v;
    cpu.r[2] = cpu.r[2] >> 2;
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    cpu.r[3] = v;
    cpu.r[2] = cpu.r[2] >> 2;
    cpu.r[3] = cpu.r[3] >> 8;
    cpu.r[2] = cpu.r[2] & cpu.r[14];
    cpu.r[3] = cpu.r[3] >> 2;
    let _ = mem.write_u32(cpu.r[6], cpu.r[2]);
    cpu.r[3] = cpu.r[3] >> 2;
    cpu.r[3] = cpu.r[3] & cpu.r[14];
    cpu.r[1] = cpu.r[1] >> 8;
    cpu.r[4] = cpu.r[4] >> 16;
    cpu.r[4] = cpu.r[4] & cpu.r[14];
    cpu.r[1] = cpu.r[1] & cpu.r[14];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[7], cpu.r[1]);
    cpu.r[2] = mem.read_u32(cpu.r[15] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[2], cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[15] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[2], cpu.r[4]);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
