//! 0x8c0d98a0 — store two image globals into r4[0] (the second store wins)
//! and r5 into r4[1]; returns r4.
//!
//! r0 is loaded twice from the literal pool (0x8c0d98b0 -> 0x8c1e8b2c, then
//! 0x8c0d98b4 -> 0x8c1ec3f0) and stored as-is; only the second value
//! survives at r4[0]. The rts delay slot leaves r0 = r4. No T-bit instruction runs.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let v1 = mem.read_u32(0x8c0d98b0).unwrap_or(0); // mov.l literal -> 0x8c1e8b2c
    let _ = mem.write_u32(r4, v1); // mov.l r0,@r4
    let v2 = mem.read_u32(0x8c0d98b4).unwrap_or(0); // mov.l literal -> 0x8c1ec3f0
    let _ = mem.write_u32(r4, v2); // mov.l r0,@r4 (overwrites)
    let _ = mem.write_u32(r4.wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r4)
    cpu.r[0] = r4; // rts delay slot: mov r4,r0
}
