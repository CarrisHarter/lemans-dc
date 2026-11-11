//! 0x8c0203d4 — call_if_flag_clear.
//!
//! Calls 0x8c017f48 with `r4+8` when `byte[r4+0x108]` is zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = 0x108;
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] & 0xff;
    let t = cpu.r[0] == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = mem.read_u32(0x8c0203f4).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(8); // delay slot of `jsr`
        crate::registry::p_8c017f48::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
