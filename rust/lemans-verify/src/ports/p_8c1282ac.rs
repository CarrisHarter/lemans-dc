//! 0x8c1282ac — `return *(u32 *)(param + 0x48);`
//!
//! Original: `add #72,r4; rts; mov.l @r4,r0`. The rts delay slot runs
//! unconditionally, so the real side effects the C hides are: r4 is
//! permanently incremented by 72, and the load happens from the shifted
//! pointer. `add #imm` and `mov.l` leave T alone.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(72); // add #72,r4
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // rts delay slot: mov.l @r4,r0
}
