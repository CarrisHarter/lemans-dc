//! 0x8c02bef0 — clamp param_1 to [-1.0, 1.0], store at [param_3 + 24].
//!
//! ```text
//! mova  0x8c02bf10,r0       ; r0 = &literal (-1.0f)
//! fmov  @r0,fr0
//! fcmp/gt fr4,fr0           ; T = fr0 > fr4  (i.e. fr4 < -1.0)
//! bf    0x8c02befc
//! bra   0x8c02bf08          ; fr4 < -1.0: fr0 stays -1.0
//! nop
//! 0x8c02befc: fldi1 fr0     ; fr0 = 1.0
//! fcmp/gt fr0,fr4           ; T = fr4 > fr0  (i.e. fr4 > 1.0)
//! bf    0x8c02bf06
//! bra   0x8c02bf08          ; fr4 > 1.0: fr0 stays 1.0
//! nop
//! 0x8c02bf06: fmov fr4,fr0
//! 0x8c02bf08: add #24,r4    ; r4 is mutated in place
//! rts
//! fmov  fr0,@r4             ; delay slot: store at the *mutated* r4
//! ```
//! Hidden: r4 += 24 before the store (the C shows param_3+0x18 but the store
//! uses the already-advanced r4), r0 = 0x8c02bf10 leftover, and T is the
//! second fcmp on the middle path but the first on the fr4 < -1.0 path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8C02BF10; // mova @(7,pc),r0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // -1.0f literal
    cpu.set_t(cpu.fr[0] > cpu.fr[4]); // fcmp/gt fr4,fr0
    if !cpu.t() {
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.set_t(cpu.fr[4] > cpu.fr[0]); // fcmp/gt fr0,fr4
        if !cpu.t() {
            cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0
        }
    }
    cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // delay slot of `rts`
}
