//! 0x8c056c04 — copy the two dwords at 0x8c23dd60 to [r4, r4+4].
//!
//! ```text
//! mov.l 0x8c056c10,r0   ; r0 = 0x8c23dd60 (literal pool)
//! mov.l @r0,r1
//! mov.l @(4,r0),r0
//! mov.l r1,@r4
//! rts ; mov.l r0,@(4,r4)  ; delay slot stores the second dword
//! ```
//! Side effects the C does not show: r1 keeps the first dword, r0 ends as
//! the second dword, and the [r4+4] store is the `rts` delay slot.
//! Verified against fixtures/lemans24-ext.bin (globals at 0x8c23dd60 lie
//! past the end of analysis/lemans24.bin).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c056c10).unwrap_or(0); // 0x8c23dd60
    cpu.r[1] = mem.read_u32(g).unwrap_or(0); // mov.l @r0,r1
    cpu.r[0] = mem.read_u32(g.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r0),r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // mov.l r1,@r4
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]); // rts delay slot
}
