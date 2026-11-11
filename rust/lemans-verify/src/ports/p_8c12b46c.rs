//! 0x8c12b46c — `return *param + 0x38;`
//!
//! Original: `mov.l @r4,r0; rts; add #56,r0`. The +56 happens in the rts
//! delay slot, after the return value is already in r0. No other register is
//! touched; T is preserved (`mov.l`/`add #imm` leave it alone).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = cpu.r[0].wrapping_add(56); // rts delay slot: add #56,r0
}
