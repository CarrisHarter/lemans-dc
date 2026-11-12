//! 0x8c1831b8 - compare two table entries at an index, returning 0 when equal.
//!
//! The index starts as `r5`; when `[r13+28]` is non-null, 0x8c166d98's result is
//! added first. The index is scaled by 4 and used against the tables at
//! `[r13+12]` and `[r13+16]`. `subc rX,rX` followed by `add #1` turns the
//! `cmp/eq` T into `1 - T`, so r0 is 0 when the entries match and 1 otherwise.
//! T ends as the equality flag.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(28)).unwrap_or(0); // mov.l @(28,r13),r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c183208).unwrap_or(0x8c166d98); // mov.l 0x8c183208,r0
        cpu.pr = 0x8c1831d0; // jsr delay is nop
        crate::registry::p_8c166d98::run(cpu, mem);
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]); // add r0,r14
        cpu.r[14] = cpu.r[14].wrapping_shl(2); // shll2 r14
        cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r13),r3
        cpu.r[0] = cpu.r[14]; // mov r14,r0
        cpu.r[1] = mem.read_u32(cpu.r[13].wrapping_add(16)).unwrap_or(0); // mov.l @(16,r13),r1
        cpu.r[3] = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r3),r3
        cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r1),r0
        cpu.set_t(cpu.r[0] == cpu.r[3]); // cmp/eq r0,r3
        let t = cpu.t() as u32; // subc r12,r12 -> -T, T unchanged
        cpu.r[12] = 0u32.wrapping_sub(t);
        cpu.r[12] = cpu.r[12].wrapping_add(1); // add #1,r12
        cpu.r[0] = cpu.r[12]; // bra delay: mov r12,r0
    } else {
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r13),r4
        cpu.r[14] = cpu.r[14].wrapping_shl(2); // shll2 r14
        cpu.r[0] = cpu.r[14]; // mov r14,r0
        cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(16)).unwrap_or(0); // mov.l @(16,r13),r3
        cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r4
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r3),r0
        cpu.set_t(cpu.r[0] == cpu.r[4]); // cmp/eq r0,r4
        let t = cpu.t() as u32; // subc r1,r1 -> -T
        cpu.r[1] = 0u32.wrapping_sub(t);
        cpu.r[1] = cpu.r[1].wrapping_add(1); // add #1,r1
        cpu.r[0] = cpu.r[1]; // mov r1,r0
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
