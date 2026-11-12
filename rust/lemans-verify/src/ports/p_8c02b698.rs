//! 0x8c02b698 — pick a blend factor for element `r5` of the array at `r4`.
//!
//! Arguments: r4 = object base, r5 = element index (stride 0xac). The flag
//! word at element+20 selects the source of the factor in fr0; the routine
//! then stores the flag word at element+156 and the factor at element+164.
//!
//! ```text
//! e = r4 + 0xac*r5 + 88
//! fl = *(u32*)(e+20)
//! fr0 = 1.0
//! fl == 0                     -> store 0 at e+68, 1.0 at e+76
//! fl & 0x008                  -> fr0 = *(f32*)(g+52)
//! fl & 0x004                  -> fr0 = *(f32*)(g+72)
//! fl & 0x010 -> fr0 = max(fr0, *(f32*)(g+92))
//! fl & 0x040 -> fr0 = max(fr0, *(f32*)(g+132))
//! fl & 0x020 -> fr0 = max(fr0, *(f32*)(g+112))
//! fl & 0x100 -> fr0 = max(fr0, *(f32*)(g+132))
//! fl & 0x200 -> fr0 = max(fr0, *(f32*)(g+112))
//! store fl at e+68, fr0 at e+76
//! ```
//!
//! `g` is the global pointer `*(u32*)0x8c2b7578`. The comparisons are strict
//! `fcmp/gt fr0,fr1` (T = fr1 > fr0), so an equal value leaves fr0 alone.
//!
//! Side effects the decompiled C hides:
//! - `mul.l r0,r5` writes both `mach` (high word) and `macl` (low word) with
//!   the signed 0xac*r5 product; `r5` then keeps `macl` on every exit.
//! - The `bf.s` delay slot `mov.l @r0,r3` runs on both outcomes, so `r3` is
//!   reloaded with `g` even on the zero path.
//! - The early exits at 0x8c02b6cc / 0x8c02b6da leave `r0` = `fl & 8` / `fl & 4`,
//!   `r3` = `g+52` / `g+72`, `r4` at the element base, and T = 0 from that
//!   path's `tst`.
//! - On the zero path `r0` = 68, `r3` = `g`, `r4` = element base + 76, T = 1.
//! - On the common path `r0` = 68, `r5` = 0x200, `r3` = `g` (+112 when bit
//!   0x200 is set), `r4` = element base + 76, and T is the last `tst`
//!   (bit 0x200 clear -> 1) or the `fcmp/gt` for bit 0x200.
//! - `fr1` holds the last field loaded; `fr0` is the running maximum.
//! - The 0x8c02b6bc path (`fl == 0` at 0x8c02b6b8) is unreachable: 0x8c02b6b8
//!   is entered only when `fl != 0`, so the second `tst` always sets T = 0 and
//!   the `bf` is always taken.

use sh4_oracle::{Cpu, Memory};

/// Global float-table pointer.
const TABLE: u32 = 0x8c2b_7578;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l 0x8c02b768,r0 (0xac) ; mul.l r0,r5 ; sts macl,r5
    cpu.r[0] = 0x0000_00ac;
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[5] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[5] = cpu.macl;

    // mov.l 0x8c02b76c,r0 (0x8c2b7578) ; add r5,r4 ; add #88,r4
    cpu.r[0] = TABLE;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]).wrapping_add(88);
    // mov.l @(20,r4),r1 ; fldi1 fr0 ; tst r1,r1
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0);
    cpu.fr[0] = 1.0;
    cpu.set_t(cpu.r[1] == 0);
    // bf.s 0x8c02b6b8 ; mov.l @r0,r3 — the delay slot runs either way.
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    if cpu.t() {
        // mov #68,r0 ; mov.l r1,@(r0,r4) ; add #76,r4 ; rts ; fmov fr0,@r4
        cpu.r[0] = 68;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]);
        cpu.r[4] = cpu.r[4].wrapping_add(76);
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
        return;
    }

    // 0x8c02b6b8 tst r1,r1 ; bf 0x8c02b6c2: r1 is nonzero, so the 0x8c02b6bc
    // arm (`fr0 = *(f32*)(g+32)`) is unreachable.

    // mov r1,r0 ; and #8,r0 ; tst r0,r0 ; bt 0x8c02b6d0
    cpu.r[0] = cpu.r[1] & 8;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // add #52,r3 ; rts ; fmov @r3,fr0
        cpu.r[3] = cpu.r[3].wrapping_add(52);
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        return;
    }

    // mov r1,r0 ; and #4,r0 ; tst r0,r0 ; bt 0x8c02b6de
    cpu.r[0] = cpu.r[1] & 4;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // add #72,r3 ; rts ; fmov @r3,fr0
        cpu.r[3] = cpu.r[3].wrapping_add(72);
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        return;
    }

    // mov r1,r0 ; and #16,r0 ; tst r0,r0 ; bt 0x8c02b6f6
    cpu.r[0] = cpu.r[1] & 16;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // mov r3,r0 ; add #92,r0 ; fmov @r0,fr1 ; fcmp/gt fr0,fr1
        cpu.r[0] = cpu.r[3].wrapping_add(92);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        // bt 0x8c02b6f4 ; fmov fr1,fr0 (else bra 0x8c02b6f6)
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }

    // mov r1,r0 ; and #64,r0 ; tst r0,r0 ; bt 0x8c02b710
    cpu.r[0] = cpu.r[1] & 64;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // mov.l 0x8c02b770,r5 (0x84) ; mov r3,r0 ; add r5,r0
        cpu.r[5] = 0x0000_0084;
        cpu.r[0] = cpu.r[3].wrapping_add(cpu.r[5]);
        // fmov @r0,fr1 ; fcmp/gt fr0,fr1 ; bt 0x8c02b70e
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }

    // mov r1,r0 ; and #32,r0 ; tst r0,r0 ; bt 0x8c02b728
    cpu.r[0] = cpu.r[1] & 32;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // mov r3,r0 ; add #112,r0 ; fmov @r0,fr1 ; fcmp/gt fr0,fr1
        cpu.r[0] = cpu.r[3].wrapping_add(112);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }

    // mov.l 0x8c02b774,r5 (0x100) ; mov r1,r0 ; and r5,r0 ; tst r0,r0
    cpu.r[5] = 0x0000_0100;
    cpu.r[0] = cpu.r[1] & cpu.r[5];
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // mov.l 0x8c02b770,r5 (0x84) ; mov r3,r0 ; add r5,r0
        cpu.r[5] = 0x0000_0084;
        cpu.r[0] = cpu.r[3].wrapping_add(cpu.r[5]);
        // fmov @r0,fr1 ; fcmp/gt fr0,fr1 ; bt 0x8c02b742
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }

    // mov.l 0x8c02b778,r5 (0x200) ; mov r1,r0 ; and r5,r0 ; tst r0,r0
    cpu.r[5] = 0x0000_0200;
    cpu.r[0] = cpu.r[1] & cpu.r[5];
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // add #112,r3 ; fmov @r3,fr1 ; fcmp/gt fr0,fr1 ; bt 0x8c02b75a
        cpu.r[3] = cpu.r[3].wrapping_add(112);
        cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }

    // mov #68,r0 ; mov.l r1,@(r0,r4) ; add #76,r4 ; rts ; fmov fr0,@r4
    cpu.r[0] = 68;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[4] = cpu.r[4].wrapping_add(76);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
}
