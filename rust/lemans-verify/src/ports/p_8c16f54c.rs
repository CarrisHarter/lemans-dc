//! 0x8c16f54c — init object: call 0x8c0d8aa8, [r4+24]=0x7fffffff, memset(r4+28, 0, 64).
//!
//! Saves r14/pr, calls 0x8c0d8aa8 with r4 kept (pr=0x8c16f558), stores
//! 0x7fffffff at [arg+24], then calls memset (0x8c010f88) with dst=r14+28,
//! val=0, n=64 (pr=0x8c16f568; clobbers r3/r7/T via the registry port).
//! Restores pr/r14 from the stack.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c16f54c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c16f54e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c16f550: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c16f552: mov.l 0x8c16f570,r0 (0x8c0d8aa8)
    cpu.r[0] = mem.read_u32(0x8c16f570).unwrap_or(0);
    // 0x8c16f554: jsr @r0 ; nop (delay)
    cpu.pr = 0x8c16f558;
    crate::registry::p_8c0d8aa8::run(cpu, mem);
    // 0x8c16f558: mov.l 0x8c16f574,r0 (0x7fffffff)
    cpu.r[0] = mem.read_u32(0x8c16f574).unwrap_or(0);
    // 0x8c16f55a: mov #64,r6 ; 0x8c16f55c: mov #0,r5
    cpu.r[6] = 64;
    cpu.r[5] = 0;
    // 0x8c16f55e: mov.l r0,@(24,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[0]);
    // 0x8c16f560: mov.l 0x8c16f578,r0 (0x8c010f88)
    cpu.r[0] = mem.read_u32(0x8c16f578).unwrap_or(0);
    // 0x8c16f562: add #28,r14
    cpu.r[14] = cpu.r[14].wrapping_add(28);
    // 0x8c16f564: jsr @r0 ; mov r14,r4 (delay)
    cpu.pr = 0x8c16f568;
    cpu.r[4] = cpu.r[14];
    crate::registry::p_8c010f88::run(cpu, mem);
    // 0x8c16f568: lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
