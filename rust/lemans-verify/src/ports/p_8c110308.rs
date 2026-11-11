//! 0x8c110308 — `*(u32*)(r4+0x14) = r5; return 1;`
//!
//! ```text
//! mov.l r5,@(20,r4)   ; the only store
//! rts
//! mov   #1,r0         ; delay slot: return value
//! ```
//! Hidden side effect: r0 = 1 comes from the `rts` delay slot, not from a
//! visible `mov` before the return.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4].wrapping_add(0x14), cpu.r[5]); // mov.l r5,@(20,r4)
    cpu.r[0] = 1; // delay slot of rts
}
