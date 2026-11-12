//! 0x8c02c488 — return the f32 at `r4 + 0xec + 0xac*r5`.
//!
//! ```text
//! mov.l 0x8c02c498,r0   ! r0 = 0xac
//! mul.l r0,r5           ! MACH:MACL = 0xac * r5
//! sts   macl,r5         ! r5 = low word
//! mov.l 0x8c02c49c,r0   ! r0 = 0xec
//! add   r0,r4           ! r4 += 0xec
//! add   r5,r4           ! r4 += macl
//! rts
//! fmov  @r4,fr0         ! delay slot: fr0 = f32[r4]
//! ```
//!
//! Both literals are 4-byte words at 0x8c02c498/0x8c02c49c (the disassembler
//! renders their bytes as instructions, the `! ac`/`! ec` comments give the
//! values). `mul.l` is a signed 32x32 product: both `mach` and `macl` are
//! overwritten, and `r5` keeps the low word. `r0` ends as 0xec.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x0000_00ac;
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[5] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[5] = cpu.macl;
    cpu.r[0] = 0x0000_00ec;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    // Delay slot of `rts`: load 4 bytes into fr0.
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
}
