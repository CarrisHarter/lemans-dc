//! 0x8c105f80 — index by `lookup(*(arg2+4))`, then search a record table.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14 ; sts.l pr,@-r15 ; mov r4,r13
//! r0 = 0x8c10e3dc ; jsr @r0 ; mov.l @(4,r14),r4     (delay slot)
//! r4 = 3*r0 ; r0 = *(u32*)(r13+4) ; r4 = 4*r4 + r0  -> 12*index + base
//! r0 = 0x8c18b8c8 ; jsr @r0 ; mov r14,r5             (delay slot)
//! lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! The first callee (0x8c10e3dc) returns `*(u32*)(*(u32*)(*(arg2+4)) + 0x4c)`;
//! the second (0x8c18b8c8) linear-searches a 20-byte-record table for `arg2`.
//! Both are ported, so the calls go through the registry with pr and the delay
//! slot. Between the calls `shll` sets T from the bit it shifts out (`shll2`
//! and `add` do not touch it), and the second callee's final `cmp` decides the
//! exit T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: r14, r13 then pr; the moves below overwrite r14/r13 afterwards.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[5];
    cpu.r[13] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c105fac).unwrap_or(0); // 0x8c10e3dc
    cpu.pr = 0x8c105f90; // jsr @r0
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // delay slot
    crate::registry::p_8c10e3dc::run(cpu, mem);
    let ret = cpu.r[0];
    cpu.r[4] = ret.wrapping_shl(1); // shll r4
    cpu.set_t(ret >> 31 != 0); // shll sets T from the bit shifted out
    cpu.r[4] = cpu.r[4].wrapping_add(ret); // add r0,r4
    cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_shl(2); // shll2 r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
    cpu.r[0] = mem.read_u32(0x8c105fb0).unwrap_or(0); // 0x8c18b8c8
    cpu.pr = 0x8c105fa2; // jsr @r0
    cpu.r[5] = cpu.r[14]; // delay slot
    crate::registry::p_8c18b8c8::run(cpu, mem);
    // Epilogue: the rts delay slot restores r14.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
