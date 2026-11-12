//! 0x8c239820 — store a 8-byte pair into a global array, or defer when r5 is null.
//!
//! `slot = 0x8c2c8a88 + (r4 << 3)`; `[slot] = r5`, `[slot+4] = r6`, then r0 = 0.
//! A null r5 tail-jumps to 0x8c23b0c0 with the arguments untouched (`jmp @r2`
//! leaves pr alone, so the callee returns to our caller).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        cpu.r[2] = mem.read_u32(0x8c239840).unwrap_or(0x8c23b0c0); // mov.l 0x8c239840,r2
        crate::registry::p_8c23b0c0::run(cpu, mem); // jmp @r2
        return;
    }
    cpu.r[7] = cpu.r[4]; // mov r4,r7
    cpu.r[0] = mem.read_u32(0x8c239844).unwrap_or(0x8c2c8a88); // mov.l 0x8c239844,r0
    cpu.r[7] = cpu.r[7].wrapping_shl(2); // shll2 r7
    cpu.r[7] = cpu.r[7].wrapping_shl(1); // shll r7
    cpu.r[3] = cpu.r[0]; // mov r0,r3
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[7]), cpu.r[5]); // mov.l r5,@(r0,r7)
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]); // add r3,r7
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[6]); // mov.l r6,@(4,r7)
    cpu.r[0] = 0; // rts delay: mov #0,r0
}
