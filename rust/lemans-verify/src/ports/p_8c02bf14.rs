//! 0x8c02bf14 — `*(float *)(param_3 + 12) = clamp(param_1, 0.0, 1.0);`
//!
//! ```text
//! fldi0  fr0                  ; fr0 = 0.0
//! fcmp/gt fr4,fr0             ; T = fr0 > fr4
//! bf     0x8c02bf1e
//! bra    0x8c02bf2a           ; fr4 < 0: keep fr0 = 0.0
//! nop
//! fldi1  fr0                  ; fr0 = 1.0
//! fcmp/gt fr0,fr4             ; T = fr4 > fr0
//! bf     0x8c02bf28
//! bra    0x8c02bf2a           ; fr4 > 1: keep fr0 = 1.0
//! nop
//! fmov   fr4,fr0              ; 0 <= fr4 <= 1: fr0 = fr4
//! add    #12,r4
//! rts
//! fmov   fr0,@r4              ; delay slot stores at r4+12 (r4 already advanced)
//! ```
//! Hidden side effects: r4 is advanced by 12 before the delay-slot store uses
//! it, and the T bit left behind differs per path (1 when fr4 < 0 or fr4 > 1,
//! 0 when 0 <= fr4 <= 1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let x = cpu.fr[4];
    cpu.fr[0] = 0.0;
    let below = 0.0f32 > x; // fcmp/gt fr4,fr0 -> T = fr0 > fr4
    cpu.set_t(below);
    if !below {
        cpu.fr[0] = 1.0;
        let above = x > 1.0; // fcmp/gt fr0,fr4 -> T = fr4 > fr0
        cpu.set_t(above);
        if !above {
            cpu.fr[0] = x; // fmov fr4,fr0
        }
    }
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // delay slot of `rts`
}
