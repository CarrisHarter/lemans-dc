//! 0x8c011d3c — Horner-style evaluation over six pairs from the table at
//! 0x8c1cd060 with `fr4^2` as the multiplier, result returned in `fr0`.
//!
//! Original:
//! ```text
//! fmov fr4,fr6 ; fmul fr4,fr6          ; fr6 = fr4*fr4
//! mov.l (0x8c1cd060),r5 ; mov #6,r4
//! fmov @r5+,fr5                        ; fr5 = table[0], r5 += 4
//! loop: fmov @r5+,fr3 ; add #-1,r4 ; fmov fr6,fr0 ; cmp/pl r4
//!       fmac fr0,fr5,fr3 ; bt.s loop ; fmov fr3,fr5
//! fmov fr4,fr0 ; fmov fr5,fr6 ; rts ; fmul fr6,fr0
//! ```
//! Same shape as 0x8c011cf8 but six iterations and the 0x8c1cd060 table.
//! Side effects the C hides: `r4` ends 0, `r5` ends table+28, `fr5 = fr6 = acc`,
//! `fr0 = fr4*acc`, T ends 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // fmov fr4,fr6 ; fmul fr4,fr6
    cpu.fr[6] = cpu.fr[4];
    cpu.fr[6] = cpu.fr[6] * cpu.fr[4];
    // mov.l 0x8c011d5c,r5 ; mov #6,r4
    cpu.r[5] = 0x8c1c_d060;
    cpu.r[4] = 6;
    // fmov @r5+,fr5
    cpu.fr[5] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    loop {
        // fmov @r5+,fr3 ; add #-1,r4 ; fmov fr6,fr0 ; cmp/pl r4 ; fmac fr0,fr5,fr3
        cpu.fr[3] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        cpu.r[4] = cpu.r[4].wrapping_sub(1);
        cpu.fr[0] = cpu.fr[6];
        cpu.set_t((cpu.r[4] as i32) > 0);
        cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[5], cpu.fr[3]);
        // bt.s loop ; fmov fr3,fr5 (delay slot runs either way)
        cpu.fr[5] = cpu.fr[3];
        if (cpu.r[4] as i32) > 0 {
            continue;
        }
        break;
    }
    // fmov fr4,fr0 ; fmov fr5,fr6 ; rts ; fmul fr6,fr0
    cpu.fr[0] = cpu.fr[4];
    cpu.fr[6] = cpu.fr[5];
    cpu.fr[0] = cpu.fr[6] * cpu.fr[0];
}
