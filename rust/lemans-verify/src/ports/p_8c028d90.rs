//! 0x8c028d90 — pick one of fr4/fr5/fr6 into fr5, return it in fr0.
//!
//! ```text
//! fcmp/gt fr4,fr5      ; T = fr5 > fr4
//! bf      0x8c028d98
//! bra     0x8c028da2   ; (fr5 > fr4) fr5 unchanged
//! nop
//! 0x8c028d98: fcmp/gt fr6,fr4  ; T = fr4 > fr6
//! bf      0x8c028da0
//! bra     0x8c028da2
//! fmov    fr6,fr5      ; delay slot: runs only on the fr4 > fr6 path
//! 0x8c028da0: fmov fr4,fr5
//! 0x8c028da2: rts
//! fmov    fr5,fr0      ; delay slot: return value
//! ```
//! Hidden: T differs per path (fr5>fr4 vs fr4>fr6), fr5 is overwritten in
//! two of the three paths, and the return happens in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem;
    cpu.set_t(cpu.fr[5] > cpu.fr[4]); // fcmp/gt fr4,fr5
    if !cpu.t() {
        cpu.set_t(cpu.fr[4] > cpu.fr[6]); // fcmp/gt fr6,fr4
        if cpu.t() {
            cpu.fr[5] = cpu.fr[6]; // delay slot of `bra`
        } else {
            cpu.fr[5] = cpu.fr[4];
        }
    }
    cpu.fr[0] = cpu.fr[5]; // delay slot of `rts`
}
