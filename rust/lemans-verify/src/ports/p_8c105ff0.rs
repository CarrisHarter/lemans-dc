//! 0x8c105ff0 — index an array of 12-byte records and tail-call 0x8c18b9c8.
//!
//! `base = [r4+8]`, `r1 = r5*3` (`shll` sets T from bit 31 of r5), `r1 <<= 2`
//! → 12*r5, `r4 = base + 12*r5`, `r5 = r6`; the literal 0x8c18b9c8 is the jmp
//! target, so the callee's `rts` returns to our caller and pr is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let index = cpu.r[5];
    let arg = cpu.r[6];
    let mut r1 = index;
    r1 <<= 1; // shll r1 — sets T from bit 31
    cpu.set_t(index & 0x8000_0000 != 0);
    r1 = r1.wrapping_add(index); // add r5,r1
    let base = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r0
    r1 <<= 2; // shll2 r1
    cpu.r[5] = arg; // mov r6,r5
    r1 = r1.wrapping_add(base); // add r0,r1
    cpu.r[0] = mem.read_u32(0x8c106004).unwrap_or(0x8c18b9c8); // mov.l 0x8c106004,r0
    cpu.r[1] = r1; // r1 holds 12*index + base at the jump
    cpu.r[4] = r1; // delay slot: mov r1,r4
    crate::registry::p_8c18b9c8::run(cpu, mem); // jmp @r0
}
