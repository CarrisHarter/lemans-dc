//! 0x8c182648 — zero_byte_return_self.
//!
//! `mov #0,r0 ; byte[r4]=r0 ; rts ; r0=r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4], cpu.r[0] as u8);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
