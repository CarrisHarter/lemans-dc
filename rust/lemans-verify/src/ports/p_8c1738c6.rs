//! 0x8c1738c6 — table_entry_nonzero.
//!
//! Indexes a 64-entry table at `0x8c2b5260+0xb4` by `([0x8c2b5260+0xb0]+1)&63` and
//! returns 1 when the entry is non-zero (`tst`+`movt`+`xor #1`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u16(0x8c1738e8).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[4] = mem.read_u32(0x8c173908).unwrap_or(0);
    cpu.r[3] = mem.read_u16(0x8c1738ea).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    cpu.r[0] = cpu.r[0].wrapping_add(1);
    cpu.r[0] = cpu.r[0] & 63;
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    cpu.r[0] = cpu.t() as u32;
    cpu.r[0] = cpu.r[0] ^ 1; // delay slot of `rts`
}
