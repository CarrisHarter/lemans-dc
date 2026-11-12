//! 0x8c14ae28 — combine three fields of `r4` into one 32-bit value.
//!
//! ```text
//! r0 = 48 * *(u32 *)(r4 + 0x2c)
//!    + low32(0x570 * *(u32 *)(r4 + 0x1c))     // signed mul.l, MACL kept
//!    + 4 * *(u32 *)(r4 + 8)
//! ```
//!
//! Every step is wrapping 32-bit arithmetic, and the address bases themselves
//! come from the wrapped loads. `mul.l` (signed) writes MACH:MACL, and only
//! MACL is read back, so a negative field just contributes its low product
//! word; MACH keeps the sign extension and is part of the returned state.
//!
//! `shll r0` at 0x8c14ae36 is the last flag-setting instruction, so T ends as
//! bit 31 of `*(u32 *)(r4 + 0x2c)`; the following `shll2`s leave T alone.
//! `r4`, `r2`, `r6`..`r15`, PR and the system registers are untouched. The
//! literal at 0x8c14ae4c is read from the loaded image so a spec that seeds
//! that address sees the same operand the original does.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14ae28: mov.l @(28,r4),r5
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(0x1c)).unwrap();
    // 0x8c14ae2a: mov.l 0x8c14ae4c,r0  (0x00000570)
    cpu.r[0] = mem.read_u32(0x8C14_AE4C).unwrap();
    // 0x8c14ae2c: mov.l @(44,r4),r1
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(0x2c)).unwrap();
    // 0x8c14ae2e: mul.l r0,r5
    let p = (cpu.r[0] as i32 as i64).wrapping_mul(cpu.r[5] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    // 0x8c14ae30: sts macl,r5
    cpu.r[5] = cpu.macl;
    // 0x8c14ae32: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c14ae34: mov.l @(8,r4),r3
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap();
    // 0x8c14ae36: shll r0
    let t = cpu.r[0] >> 31 != 0;
    cpu.r[0] = cpu.r[0] << 1;
    cpu.set_t(t);
    // 0x8c14ae38: add r1,r0  (r0 = 3 * field2c)
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c14ae3a: mov #0,r1
    cpu.r[1] = 0;
    // 0x8c14ae3c: shll2 r0
    cpu.set_t(cpu.r[0] >> 30 & 1 != 0);
    cpu.r[0] = cpu.r[0] << 2;
    // 0x8c14ae3e: add r5,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]);
    // 0x8c14ae40: shll2 r3
    cpu.r[3] = cpu.r[3] << 2;
    // 0x8c14ae42: add r3,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    // 0x8c14ae44: shll2 r0 - the last T-writing instruction
    cpu.set_t(cpu.r[0] >> 30 & 1 != 0);
    cpu.r[0] = cpu.r[0] << 2;
    // 0x8c14ae46: rts, delay slot 0x8c14ae48: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
}
