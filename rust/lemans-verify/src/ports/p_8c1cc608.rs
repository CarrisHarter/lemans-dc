//! 0x8c1cc608 — store_to_global_return_self.
//!
//! `r0=[0x8c1cc610] ; [r0]=r4 ; rts ; r0=r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1cc610).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
