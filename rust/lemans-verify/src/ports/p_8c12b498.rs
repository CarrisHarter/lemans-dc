//! 0x8c12b498 — double read at `*param + 0x1c` (Ghidra's fr1 is really fr0).
//!
//! Original: `mov.l @r4,r0; add #28,r0; rts; fmov @r0,fr0`. The pointer is
//! dereferenced, bumped by 28 *before* rts, and the delay slot loads fr0 from
//! the resulting address. Hidden side effects: r0 ends as *param + 28 (not a
//! plain load), fr0 := raw bits at that address.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = cpu.r[0].wrapping_add(28); // add #28,r0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // rts delay slot: fmov @r0,fr0
}
