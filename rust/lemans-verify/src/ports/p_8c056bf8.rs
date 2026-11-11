//! 0x8c056bf8 — `*param = PTR_0x8c056c00; return param;`
//!
//! ```text
//! mov.l 0x8c056c00,r0   ; r0 = 0x8c1e8f88 (literal pool)
//! mov.l r0,@r4
//! rts ; mov r4,r0       ; delay slot: return value
//! ```
//! Side effects the C does not show: the store happens before `rts`, and
//! r0 ends as the argument (r4), not the pool value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let v = mem.read_u32(0x8c056c00).unwrap_or(0); // mov.l 0x8c056c00,r0
    let _ = mem.write_u32(cpu.r[4], v); // mov.l r0,@r4
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`: mov r4,r0
}
