//! 0x8c21e0c8 — conditional_append.
//!
//! When bit 3 of `[0x8c2bc96c]` is clear and `[0x8c2bc978] < [0x8c2bc97c]`, stores
//! `r4` at the cursor and advances it by 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c21e0e8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let t = (cpu.r[0] & 8) == 0;
    cpu.set_t(t);
    if t {
        return; // `bt` when bit 3 is clear
    }
    cpu.r[3] = mem.read_u32(0x8c21e0f0).unwrap_or(0);
    cpu.r[6] = mem.read_u32(0x8c21e0ec).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    let hs = cpu.r[5] >= cpu.r[1];
    cpu.set_t(hs);
    if hs {
        return;
    }
    let _ = mem.write_u32(cpu.r[5], cpu.r[4]);
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[6], cpu.r[1]);
}
