//! 0x8c03943c — `return *(undefined4 *)(param_1 + 0x24);` — the whole port is
//! the `rts` delay slot.
//!
//! ```text
//! rts
//! mov.l @(36,r4),r0           ; delay slot: r0 = [r4 + 0x24]
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap(); // delay slot of `rts`
}
