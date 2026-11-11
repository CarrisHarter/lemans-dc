//! 0x8c236d40 — pack_five_fields.
//!
//! Packs five 2-bit fields into one word: `0x100000 | (v0<<12) | (r6<<4) | (r7<<8) | (v4<<16) | (r5 & 3)`,
//! where `v0`/`v4` are the stack arguments.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = 3;
    cpu.r[6] = cpu.r[6] & cpu.r[1];
    cpu.r[3] = cpu.r[3] & cpu.r[1];
    cpu.r[3] = cpu.r[3] << 8;
    cpu.r[6] = cpu.r[6] << 2;
    cpu.r[7] = cpu.r[7] & cpu.r[1];
    cpu.r[3] = cpu.r[3] << 2;
    cpu.r[6] = cpu.r[6] << 2;
    cpu.r[7] = cpu.r[7] << 8;
    cpu.r[3] = cpu.r[3] << 2;
    cpu.r[6] = cpu.r[6] | cpu.r[7];
    cpu.r[6] = cpu.r[6] | cpu.r[3];
    cpu.r[3] = mem.read_u32(cpu.r[15] + 4).unwrap_or(0);
    cpu.r[5] = cpu.r[5] & cpu.r[1];
    cpu.r[3] = cpu.r[3] & cpu.r[1];
    cpu.r[3] = cpu.r[3] << 16;
    cpu.r[6] = cpu.r[6] | cpu.r[3];
    cpu.r[6] = cpu.r[6] | cpu.r[5];
    cpu.r[5] = mem.read_u32(0x8c236d80).unwrap_or(0);
    cpu.r[5] = cpu.r[5] | cpu.r[6];
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
}
