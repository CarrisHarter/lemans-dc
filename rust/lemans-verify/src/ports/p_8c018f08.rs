//! 0x8c018f08 — link and zero two blocks.
//!
//! Publishes `r4` at `0x8c23da88`, links `r4+0x100` at `[r4+0x1108]` and `r4`
//! itself at `[r4+0x1100]`, stores `r4+0x100` at `[r4+0x1104]`, then memsets
//! (0x8c010f88) `0x1104` bytes at `r4+0x100` and `0x1000` bytes at `r4+0x100`
//! with 0. Returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c018f40).unwrap_or(0);
    cpu.r[3] = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c018f44).unwrap_or(0);
    cpu.r[5] = 0;
    let _ = mem.write_u32(cpu.r[0], cpu.r[14]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]);
    cpu.r[0] = mem.read_u32(0x8c018f48).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[3]);
    cpu.r[0] = mem.read_u32(0x8c018f4c).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[14]);
    cpu.r[1] = mem.read_u32(0x8c018f50).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c018f54).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[3]); // delay slot of `jsr`
    crate::registry::p_8c010f88::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c018f44).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c018f58).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c018f50).unwrap_or(0);
    cpu.r[5] = 0; // delay slot of `jsr`
    crate::registry::p_8c010f88::run(cpu, mem);
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
