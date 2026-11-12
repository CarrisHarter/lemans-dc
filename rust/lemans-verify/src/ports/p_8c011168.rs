//! 0x8c011168 — `context_save`: copy the live CPU state into the 152-byte
//! structure at `r4` (jmp_buf-style), then return 0.
//!
//! Layout written, all longwords unless noted:
//!   0x00 r15, 0x04 r14, 0x08 pr, 0x0c r13 … 0x30 r4, 0x34 r3 … 0x40 r0,
//!   0x44 mach, 0x48 macl, 0x4c gbr,
//!   0x50 fr0 … 0x8c fr15, 0x90 fpul, 0x94 fpscr.
//!
//! The routine smuggles `pr` through `r14` (sts pr,r14 at 0x8c01116c) and
//! reloads the original `r14` from the buffer at 0x8c011170, then walks `r4`
//! itself as the store cursor and finally reloads the caller's `r4` from
//! +0x30. So the only visible residue is `r0 = 0`; r1-r15, pr, sr and the
//! system registers are untouched.
//!
//! The fp stores are plain 4-byte longword moves (`fmov frN,@Rn` / `@-Rn`);
//! the harness models no SZ pair transfers, and the routine never reads FPSCR.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];

    // 0x8c011168: mov.l r15,@r4
    let _ = mem.write_u32(base, cpu.r[15]);
    // 0x8c01116a: mov.l r14,@(4,r4)
    let _ = mem.write_u32(base.wrapping_add(4), cpu.r[14]);
    // 0x8c01116c: sts pr,r14 ; 0x8c01116e: mov.l r14,@(8,r4)
    cpu.r[14] = cpu.pr;
    let _ = mem.write_u32(base.wrapping_add(8), cpu.r[14]);
    // 0x8c011170: mov.l @(4,r4),r14 — restore the caller's r14
    cpu.r[14] = mem.read_u32(base.wrapping_add(4)).unwrap_or(0);

    // 0x8c011172..0x8c011182: mov.l r13..r5,@(disp,r4)
    for (reg, disp) in [
        (13usize, 12u32),
        (12, 16),
        (11, 20),
        (10, 24),
        (9, 28),
        (8, 32),
        (7, 36),
        (6, 40),
        (5, 44),
    ] {
        let _ = mem.write_u32(base.wrapping_add(disp), cpu.r[reg]);
    }
    // 0x8c011184: mov.l r4,@(48,r4) ; 0x8c011186: add #48,r4
    let _ = mem.write_u32(base.wrapping_add(48), cpu.r[4]);
    cpu.r[4] = cpu.r[4].wrapping_add(48);

    // 0x8c011188..0x8c01118e: r3,r2,r1,r0 at +4,+8,+12,+16
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[2]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[0]);

    // 0x8c011190..0x8c01119a: sts mach / sts macl / stc gbr, each stored
    cpu.r[0] = cpu.mach;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[0]);
    cpu.r[0] = cpu.macl;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[0]);
    cpu.r[0] = cpu.gbr;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[0]);

    // 0x8c01119c: add #92,r4 ; 0x8c01119e: sts fpscr,r0 ; @(8,r4)
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[0] = cpu.fpscr;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
    // 0x8c0111a2: sts fpul,r0 ; 0x8c0111a4: mov.l r0,@(4,r4)
    cpu.r[0] = cpu.fpul;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);

    // 0x8c0111a6: fmov fr15,@r4 ; then 0x8c0111a8..0x8c0111c4: fmov frN,@-r4
    let _ = mem.write_f32(cpu.r[4], cpu.fr[15]);
    for n in (0..15).rev() {
        cpu.r[4] = cpu.r[4].wrapping_sub(4);
        let _ = mem.write_f32(cpu.r[4], cpu.fr[n]);
    }

    // 0x8c0111c6: add #-32,r4 ; 0x8c0111c8: mov.l @r4,r4
    cpu.r[4] = cpu.r[4].wrapping_add(-32i32 as u32);
    cpu.r[4] = mem.read_u32(cpu.r[4]).unwrap_or(0);

    // 0x8c0111ca/0x8c0111cc: rts ; mov #0,r0 (delay slot)
    cpu.r[0] = 0;
}
