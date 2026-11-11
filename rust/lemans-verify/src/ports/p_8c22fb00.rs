//! 0x8c22fb00 — set_slot_or_error.
//!
//! When `r4` is 4-byte aligned, clears bit 0 of `[0x8c2c9d78]`, stores `r4` at
//! `+12`, sets bit 0 again, and returns 0. Otherwise returns `0xeeee`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = (-4i32) as u32;
    let t = (cpu.r[4] & cpu.r[3]) == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[4] = mem.read_u32(0x8c22fb64).unwrap_or(0);
    } else {
        cpu.r[3] = mem.read_u32(0x8c22fb68).unwrap_or(0);
        cpu.r[2] = (-2i32) as u32;
        cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[1] = cpu.r[1] & cpu.r[2];
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        cpu.r[3] = cpu.r[3].wrapping_add(12);
        let _ = mem.write_u32(cpu.r[3], cpu.r[4]);
        cpu.r[1] = mem.read_u32(0x8c22fb68).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[0] = cpu.r[0] | 1;
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        cpu.r[4] = 0;
    }
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
