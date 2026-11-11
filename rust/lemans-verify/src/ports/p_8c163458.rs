//! 0x8c163458 — store_byte_indexed_global.
//!
//! `r0=[0x8c163460] ; rts ; byte[r0+r4]=r5` — a byte store into the global
//! table at `0x8c20ad70`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c163460).unwrap_or(0);
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u8(addr, cpu.r[5] as u8); // delay slot of `rts`
}
