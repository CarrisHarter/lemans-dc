//! 0x8c02c4a0 — clamp `fr4` to [0.0, 1.0] and store it at `r4 + 0xec + 0xac*r5`.
//!
//! ```text
//! fldi0   fr0            ; fr0 = 0.0
//! fcmp/gt fr4,fr0        ; T = (fr0 > fr4)      i.e. fr4 < 0.0
//! bf      0x8c02c4aa
//! bra     0x8c02c4b6     ; fr4 < 0: keep fr0 = 0.0, T = 1
//! nop
//! fldi1   fr0            ; fr0 = 1.0
//! fcmp/gt fr0,fr4        ; T = (fr4 > fr0)      i.e. fr4 > 1.0
//! bf      0x8c02c4b4
//! bra     0x8c02c4b6     ; fr4 > 1: keep fr0 = 1.0, T = 1
//! nop
//! fmov    fr4,fr0        ; otherwise fr0 = fr4 (passes NaN and -0.0 through)
//! mov.l   @(0x8c02c4c8,pc),r0   ; r0 = 0xac
//! mul.l   r0,r5
//! sts     macl,r5
//! mov.l   @(0x8c02c4cc,pc),r0   ; r0 = 0xec
//! add     r0,r4
//! add     r5,r4
//! rts
//! fmov    fr0,@r4        ; delay slot: f32 store
//! ```
//!
//! The literals at 0x8c02c4c8/0x8c02c4cc are 4-byte words 0xac and 0xec; the
//! disassembler renders their bytes as unrelated instructions. Every path
//! merges at 0x8c02c4b6, so there is a single store and no early exit.
//!
//! Side effects the decompiled C hides: `mul.l` overwrites both `mach` (high
//! word) and `macl` (low word) with the signed 0xac*r5 product and `r5` keeps
//! `macl`; `r0` ends as 0xec; `r4` is advanced by 0xec + macl (wrapping) and is
//! the store address; T is left as the last `fcmp/gt` executed. `fpscr` is not
//! touched (the oracle models all `fr` as single precision, so FPSCR.SZ/PR do
//! not change the store width). `fr0` ends as the clamped value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.set_t(cpu.fr[0] > cpu.fr[4]); // fcmp/gt fr4,fr0
    if !cpu.t() {
        // bf 0x8c02c4aa: fr4 is not negative.
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.set_t(cpu.fr[4] > cpu.fr[0]); // fcmp/gt fr0,fr4
        if !cpu.t() {
            // bf 0x8c02c4b4: fr4 <= 1.0 (NaN compares false).
            cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0
        }
    }
    // 0x8c02c4b6: mov.l @(0x8c02c4c8,pc),r0
    cpu.r[0] = 0x0000_00ac;
    // 0x8c02c4b8: mul.l r0,r5
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[5] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    // 0x8c02c4ba: sts macl,r5
    cpu.r[5] = cpu.macl;
    // 0x8c02c4bc: mov.l @(0x8c02c4cc,pc),r0
    cpu.r[0] = 0x0000_00ec;
    // 0x8c02c4be: add r0,r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    // 0x8c02c4c0: add r5,r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    // 0x8c02c4c2: rts ; 0x8c02c4c4: fmov fr0,@r4 (delay slot)
    let _ = mem.write_u32(cpu.r[4], cpu.fr[0].to_bits());
}
