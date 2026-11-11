//! 0x8c1cc380 — clear_and_copy_flag.
//!
//! Reads the byte at `0x8c2b8738`, clears `byte[0x8c215d80]`, and stores the
//! byte at `0x8c2b8740` in the `rts` delay slot. Note `r4` is overwritten by
//! the pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1cc394).unwrap_or(0);
    cpu.r[3] = 0;
    cpu.r[4] = mem.read_u32(0x8c1cc398).unwrap_or(0);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[1] = mem.read_u32(0x8c1cc39c).unwrap_or(0);
    cpu.r[0] = cpu.r[0] & 0xff;
    let _ = mem.write_u8(cpu.r[4], cpu.r[3] as u8);
    let _ = mem.write_u8(cpu.r[1], cpu.r[0] as u8); // delay slot of `rts`
}
