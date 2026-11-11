//! 0x8c011d1c — Horner-style evaluation over six words from the table at
//! 0x8c1cd040 with `fr4^2` as the multiplier, result in `fr0`/`fr4`.
//!
//! Original:
//! ```text
//! fmov fr4,fr5 ; fmul fr4,fr5          ; fr5 = fr4*fr4
//! mov.l (0x8c1cd040),r5 ; mov #6,r4
//! fmov @r5+,fr4                        ; fr4 = table[0], r5 += 4 (pre-loop)
//! loop: fmov @r5+,fr3 ; add #-1,r4 ; fmov fr5,fr0 ; cmp/pl r4
//!       fmac fr0,fr4,fr3 ; bt.s loop ; fmov fr3,fr4
//! rts ; fmov fr4,fr0
//! ```
//! The loop body starts at 0x8c011d26: each pass loads a fresh `fr3`,
//! fmacs it with the previous accumulator held in `fr4`, then the `bt.s`
//! delay slot `fmov fr3,fr4` (which runs taken or not) carries the
//! accumulator. Side effects the C hides: `r4` ends 0, `r5` ends table+28,
//! `fr0 = fr4 = acc`, `fr5` keeps `fr4_input^2`, T ends 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // fmov fr4,fr5 ; fmul fr4,fr5
    cpu.fr[5] = cpu.fr[4];
    cpu.fr[5] = cpu.fr[5] * cpu.fr[4];
    // mov.l 0x8c011d38,r5 ; mov #6,r4
    cpu.r[5] = 0x8c1c_d040;
    cpu.r[4] = 6;
    // fmov @r5+,fr4 (pre-loop)
    cpu.fr[4] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    loop {
        // loop (0x8c011d26): fmov @r5+,fr3 ; add #-1,r4 ; fmov fr5,fr0
        cpu.fr[3] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        cpu.r[4] = cpu.r[4].wrapping_sub(1);
        cpu.fr[0] = cpu.fr[5];
        // cmp/pl r4 ; fmac fr0,fr4,fr3
        cpu.set_t((cpu.r[4] as i32) > 0);
        cpu.fr[3] = cpu.fr[0].mul_add(cpu.fr[4], cpu.fr[3]);
        // bt.s loop ; fmov fr3,fr4 (delay slot runs either way)
        cpu.fr[4] = cpu.fr[3];
        if (cpu.r[4] as i32) > 0 {
            continue;
        }
        break;
    }
    // rts ; fmov fr4,fr0 (delay slot)
    cpu.fr[0] = cpu.fr[4];
}
