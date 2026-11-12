//! 0x8c16e200 - zero 0x3fc bytes at `r4+24`, then clear the flag at `[[r4+0x430]+8]`.
//!
//! memset gets `r4 = r14+24`, `r5 = 0`, `r6 = 0x3fc`; it clobbers r3/r7/T, and the
//! port mirrors that through the registry call. The final clear writes `[r1+8] = 0`
//! with `r1 = [r14+0x430]` and leaves `r0 = 0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[6] = mem.read_u32(0x8c16e220).unwrap_or(0x3fc); // mov.l 0x8c16e220,r6
    cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
    cpu.r[0] = mem.read_u32(0x8c16e224).unwrap_or(0x8c010f88); // mov.l 0x8c16e224,r0
    cpu.r[5] = 0; // jsr delay: mov #0,r5
    cpu.pr = 0x8c16e210;
    crate::registry::p_8c010f88::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c16e228).unwrap_or(0x430); // mov.l 0x8c16e228,r0
    cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r14),r1
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[0]); // mov.l r0,@(8,r1)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
