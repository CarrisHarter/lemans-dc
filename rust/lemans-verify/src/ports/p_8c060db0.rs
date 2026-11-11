//! 0x8c060db0 — copy 16 bytes (4 mov.l) from [r5] to [[r4]].
//!
//! Leftovers the C does not show: r3 = dst = [r4], and the loads leave
//! r1 = mem32[r5+8], r0 = mem32[r5+12]; the last store ([dst+12]) is the
//! rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r3 (dst)
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r1
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r5),r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r3)
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r5),r1
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r5),r0
    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r3)
    let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[0]); // rts delay slot
}
