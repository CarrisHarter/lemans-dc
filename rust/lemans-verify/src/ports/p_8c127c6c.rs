//! 0x8c127c6c — `fmov fr4,@(r4+0x40)` with r4 pre-incremented by the delay-slot-free
//! `add`.
//!
//! ```text
//! add  #64,r4
//! rts
//! fmov fr4,@r4    ; delay slot: 32-bit store of fr4 at the NEW r4
//! ```
//! Hidden side effects: r4 is bumped by 0x40 (the C reads param_3+0x40 without
//! showing the register change), and the store happens in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(0x40); // add #64,r4
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // delay slot: fmov fr4,@r4
}
