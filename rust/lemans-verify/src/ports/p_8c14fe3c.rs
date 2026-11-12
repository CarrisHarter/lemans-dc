//! 0x8c14fe3c - fetch element `r5` of the array at `[[r4+4]+12+8]` and load its +12.
//!
//! `n = [[r4+4]+12+4]`; a plain `bf` returns 0 when `n <= r5`. Otherwise the
//! element `[[[r4+4]+12]+8][r5]` goes to 0x8c0f8850, whose r0 is the result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r1
    cpu.r[1] = cpu.r[1].wrapping_add(12); // add #12,r1
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r1),r0
    cpu.set_t(cpu.r[0] > cpu.r[5]); // cmp/hi r5,r0
    if !cpu.t() {
        cpu.r[0] = 0; // mov #0,r0
    } else {
        cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r1),r1
        // The callee 0x8c0f8850 never touches T, so the shll2's carry survives.
        cpu.set_t(cpu.r[5] >> 30 & 1 != 0); // shll2 r5 sets T from bit 30
        cpu.r[5] = cpu.r[5].wrapping_shl(2); // shll2 r5
        cpu.r[0] = cpu.r[5]; // mov r5,r0
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r1),r4
        cpu.r[0] = mem.read_u32(0x8c14fe64).unwrap_or(0x8c0f8850); // mov.l 0x8c14fe64,r0
        cpu.pr = 0x8c14fe56;
        crate::registry::p_8c0f8850::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
