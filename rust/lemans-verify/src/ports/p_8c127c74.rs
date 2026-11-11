//! 0x8c127c74 — same shape as 0x8c127c6c with offset 0x4c.
//!
//! ```text
//! add  #76,r4
//! rts
//! fmov fr4,@r4    ; delay slot: 32-bit store of fr4 at r4+0x4c
//! ```
//! Hidden side effects: r4 += 0x4c and the fr4 store runs in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(0x4C); // add #76,r4
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // delay slot: fmov fr4,@r4
}
