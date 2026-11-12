//! 0x8c0d0020 — clamp |fr4| into [0, 100.0] and store it at [*r4 + 52].
//!
//! Calls 0x8c0b2780 (which puts |fr4| in fr0), then a two-step clamp with
//! `fldi0` (0.0) and the `mova` constant 100.0: negative → 0.0, above 100.0 →
//! 100.0, otherwise the value itself. The clamped float is stored 4 bytes at
//! `[*r14 + 52]`; `r0` ends as that address, r14/pr are restored, T is the last
//! `fcmp/gt`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c0d0050).unwrap_or(0x8c0b2780); // mov.l 0x8c0d0050,r0
    cpu.pr = 0x8c0d002c;
    crate::registry::p_8c0b2780::run(cpu, mem); // jsr @r0 ; nop

    let fr0 = cpu.fr[0];
    let mut fr1 = 0.0f32; // fldi0 fr1
    cpu.set_t(fr1 > fr0); // fcmp/gt fr0,fr1
    if !(fr1 > fr0) {
        // not negative: compare against the mova constant 100.0
        let limit = f32::from_bits(mem.read_u32(0x8c0d0054).unwrap_or(0x42c80000));
        fr1 = limit; // fmov @r0,fr1
        cpu.set_t(fr0 > fr1); // fcmp/gt fr1,fr0
        if !(fr0 > fr1) {
            fr1 = fr0; // fmov fr0,fr1
        }
    }
    cpu.fr[1] = fr1;

    cpu.r[0] = mem.read_u32(object).unwrap_or(0); // mov.l @r14,r0
    cpu.r[0] = cpu.r[0].wrapping_add(52); // add #52,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // fmov fr1,@r0
}
