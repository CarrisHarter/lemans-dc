//! 0x8c221dc8 — fill_slot_record.
//!
//! Indexes the 128-byte table at `0x8c2bde98` by `r4` and fills `+20..+40` from
//! `r5`/`r6`/`r7` and three stack words, then sets `+4 = 1` in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[3] = mem.read_u32(0x8c221e14).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4] << 1;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 20, cpu.r[5]);
    let _ = mem.write_u32(cpu.r[4] + 24, cpu.r[6]);
    let _ = mem.write_u32(cpu.r[4] + 28, cpu.r[7]);
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 32, cpu.r[2]);
    cpu.r[1] = mem.read_u32(cpu.r[15] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 36, cpu.r[1]);
    cpu.r[1] = 1;
    cpu.r[2] = mem.read_u32(cpu.r[15] + 8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 40, cpu.r[2]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]); // delay slot of `rts`
}
