//! 0x8c0573f4 — copy the 16 bytes at r4 to 0x8c23dd80.
//!
//! ```text
//! mov.l @r4,r1 ; mov.l 0x8c057408,r3 (=0x8c23dd80) ; mov.l @(4,r4),r0
//! mov.l r1,@r3 ; mov.l r0,@(4,r3)
//! mov.l @(8,r4),r1 ; mov.l @(12,r4),r0 ; mov.l r1,@(8,r3)
//! rts ; mov.l r0,@(12,r3)   ; delay slot stores the last dword
//! ```
//! Leftovers the C does not show: r0 = [r4+12], r1 = [r4+8], r3 = 0x8c23dd80.
//! Verified against fixtures/lemans24-ext.bin (the global at 0x8c23dd80 lies
//! past the end of analysis/lemans24.bin).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = mem.read_u32(base).unwrap_or(0); // mov.l @r4,r1
    cpu.r[3] = mem.read_u32(0x8c057408).unwrap_or(0); // mov.l 0x8c057408,r3
    cpu.r[0] = mem.read_u32(base.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r3)
    cpu.r[1] = mem.read_u32(base.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r1
    cpu.r[0] = mem.read_u32(base.wrapping_add(12)).unwrap_or(0); // mov.l @(12,r4),r0
    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r3)
    let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[0]); // rts delay slot
}
