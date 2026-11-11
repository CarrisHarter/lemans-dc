//! 0x8c011cf8 — 5-iteration FPU dot product: fr6 = fr4^2, then
//! fr3 accumulates fr6 * table pairs at 0x8c1cd028; returns fr0 = fr4.
//!
//! Original side effects: fr6 = fr4*fr4, r5 walks the table, r4 counts down
//! to 0, T = (r4 > 0) per iteration, fr5/fr3 are loop residue, final
//! fr0 = fr4, fr6 = fr5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[6] = cpu.fr[4] * cpu.fr[4]; // fmov fr4,fr6 ; fmul fr4,fr6
    cpu.r[5] = 0x8c1cd028; // mov.l literal
    cpu.r[4] = 5;
    cpu.fr[5] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap_or(0));
    cpu.r[5] = cpu.r[5].wrapping_add(4); // pre-loop fmov @r5+,fr5 (0x8c011d00)
    loop {
        cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap_or(0));
        cpu.r[5] = cpu.r[5].wrapping_add(4); // loop head 0x8c011d02: fmov @r5+,fr3
        cpu.r[4] = cpu.r[4].wrapping_sub(1);
        cpu.fr[0] = cpu.fr[6];
        cpu.set_t((cpu.r[4] as i32) > 0); // cmp/pl r4
        cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[5], cpu.fr[3]); // fmac fr0,fr5,fr3
        cpu.fr[5] = cpu.fr[3]; // bt.s delay slot: fmov fr3,fr5
        if (cpu.r[4] as i32) > 0 {
            continue;
        }
        break;
    }
    cpu.fr[0] = cpu.fr[4];
    cpu.fr[6] = cpu.fr[5];
}
