//! 0x8c127c7c — same shape as 0x8c127c6c with offset 0x44.
//!
//! ```text
//! add  #68,r4
//! rts
//! fmov fr4,@r4    ; delay slot: 32-bit store of fr4 at r4+0x44
//! ```
//! Hidden side effects: r4 += 0x44 and the fr4 store runs in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(0x44); // add #68,r4
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // delay slot: fmov fr4,@r4
}
