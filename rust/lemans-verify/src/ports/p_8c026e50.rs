//! 0x8c026e50 — call_and_clear_flag.
//!
//! Calls 0x8c050c58(`[r4+44]`, 0) then clears `byte[r4+80]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[4] = mem.read_u32(cpu.r[14] + 44).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c026e6c).unwrap_or(0);
    cpu.r[5] = 0; // delay slot of `jsr`
    crate::registry::p_8c050c58::run(cpu, mem);
    cpu.r[0] = 80;
    cpu.r[1] = 0;
    let _ = mem.write_u8(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
