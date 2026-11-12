//! 0x8c237700 — look `r4` up with 0x8c237780 and, on a hit with an in-range
//! index, hand the masked table entry back through `r6`.
//!
//! The prologue spills `r4`/`r5`/`r6` to a 12-byte frame and the `jsr` delay
//! slot reloads `r4` from the frame. A miss from the lookup returns the
//! sentinel 0x0000eeee. A hit compares `param_2` against `[param_1+12]`
//! (unsigned `cmp/hs`), and an out-of-range index also returns 0xeeee.
//! Otherwise `param_2 * 8` is computed with `shll2`/`shll` (32-bit wrap; the
//! `shll` is the last flag-setting instruction of that path and leaves
//! T = bit 29 of `param_2`), added to `param_1 + 0x48`, and the loaded word is
//! stored through `param_3` twice: raw, then ANDed with 0xefffffff (bit 28
//! cleared).
//!
//! Exit T: 1 on both failure paths (`tst` / `cmp/hs` are the last flag setters
//! before the returns), bit 29 of `param_2` on the success path.

use sh4_oracle::{Cpu, Memory};

/// Entry of 0x8c237780, also the word in the literal pool at 0x8c237760.
const CALLEE: u32 = 0x8c237780;

/// Shared epilogue `add #12,r15 ; lds.l @r15+,pr` then `rts`.
fn leave(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // add #-12,r15
    cpu.r[3] = mem.read_u32(0x8c237760).unwrap_or(CALLEE); // mov.l 0x8c237760,r3
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[4]); // mov.l r4,@(8,r15)
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r15)
    let _ = mem.write_u32(cpu.r[15], cpu.r[6]); // mov.l r6,@r15
    cpu.pr = 0x8c237710; // jsr @r3
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0); // delay: mov.l @(8,r15),r4
    crate::registry::p_8c237780::run(cpu, mem);

    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        // bt 0x8c23773e — lookup miss
        cpu.r[0] = mem.read_u32(0x8c237768).unwrap_or(0); // mov.l 0x8c237768,r0
        leave(cpu, mem); // 0x8c237740: add #12,r15 ; lds.l @r15+,pr ; rts
        return;
    }

    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r15),r4
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r2
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r4),r3
    cpu.set_t(cpu.r[2] >= cpu.r[3]); // cmp/hs r3,r2
    if cpu.t() {
        // bt 0x8c23773e — index out of range
        cpu.r[0] = mem.read_u32(0x8c237768).unwrap_or(0); // mov.l 0x8c237768,r0
        leave(cpu, mem);
        return;
    }

    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r2
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(72); // add #72,r1
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[2] = cpu.r[2] << 2; // shll2 r2 (no T)
    cpu.set_t(cpu.r[2] >> 31 != 0); // shll r2
    cpu.r[2] = cpu.r[2] << 1;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[1]); // add r1,r2
    cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2,r0
    cpu.r[2] = mem.read_u32(0x8c237764).unwrap_or(0); // mov.l 0x8c237764,r2
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]); // mov.l r0,@r3
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r1
    cpu.r[1] &= cpu.r[2]; // and r2,r1
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    // bra 0x8c237748 ; delay 0x8c23773c: nop
    cpu.r[0] = 0; // mov #0,r0
    leave(cpu, mem); // 0x8c23774a: add #12,r15 ; lds.l @r15+,pr ; rts
}
