//! 0x8c14b280 — scale fr4 by pool const, clamp to [0, pool2], ftrc to int,
//! insert shifted into r4 at bit position r5 (clearing via ~r6).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8c14b2b8;
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
    cpu.fr[1] = 0.0; // fldi0 fr1
    cpu.fr[4] = cpu.fr[0] * cpu.fr[4]; // fmul fr0,fr4
    cpu.fpul = cpu.fr[4] as i32 as u32; // ftrc fr4,fpul
    cpu.r[0] = cpu.fpul.wrapping_add(127);
    cpu.fpul = cpu.r[0]; // lds r0,fpul
    cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1: T=(fr1>fr0)
    if cpu.fr[1] > cpu.fr[0] {
        // bra to 0x8c14b2a8 (skip upper clamp)
    } else {
        cpu.r[0] = 0x8c14b2bc;
        cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
        cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0: T=(fr0>fr1)
        if !(cpu.fr[0] > cpu.fr[1]) {
            cpu.fr[1] = cpu.fr[0];
        }
    }
    cpu.fpul = cpu.fr[1] as i32 as u32; // ftrc fr1,fpul
    cpu.r[0] = !cpu.r[6]; // not r6,r0
    cpu.r[4] &= cpu.r[0];
    cpu.r[1] = cpu.fpul; // sts fpul,r1
    let a = cpu.r[5] as i32;
    cpu.r[1] = if a >= 0 { cpu.r[1] << a } else { cpu.r[1] >> (-a) }; // shld r5,r1
    cpu.r[4] |= cpu.r[1];
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
