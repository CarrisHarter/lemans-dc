//! 0x8c05799c — `*(param_1 + 0x4c) = param_2` with r0 = 76 as the offset.
//!
//! ```text
//! mov #76,r0
//! rts ; mov.l r5,@(r0,r4)   ; delay slot: [r4+76] = r5
//! ```
//! Side effects the C does not show: r0 ends as 76 (the displacement), and
//! the store is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 76; // mov #76,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // rts delay slot
}
