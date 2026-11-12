//! 0x8c12b430 — first element of a counted pointer array whose byte 4 is zero.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
//! mov.l @r4,r14   ; r14 = *param
//! add   #56,r14   ; r14 = *param + 56 (count at +4, array base at +8)
//! bra   0x8c12b450
//! mov.l @(8,r14),r13      ; delay slot: r13 = first element
//! loop:
//! mov.l @(4,r14),r0 ; mov.l @(8,r14),r1
//! shll2 r0 ; add r0,r1 ; cmp/eq r13,r1
//! bf    body               ; r13 != base + count*4
//! mov   #0,r0              ; exhausted: return 0
//! bra   epilogue
//! body:
//! mov.l 0x8c12b468,r0      ; 0x8c10d7c8
//! jsr   @r0
//! mov.l @r13,r4            ; delay slot: argument = *r13
//! extu.b r0,r0 ; tst r0,r0
//! bf    advance
//! bra   epilogue
//! mov.l @r13,r0            ; delay slot: return the matching element
//! advance:
//! add   #4,r13
//! ```
//!
//! The callee (0x8c10d7c8) loads the sign-extended byte at `arg+4`; `extu.b`
//! keeps only its low byte, so the scan stops at the first element whose byte
//! four is zero. `T` is 1 on every exit: `cmp/eq` sets it on the exhausted
//! path and `tst` sets it on the found path, and no later instruction changes
//! it. `r1` residue is the last computed end pointer; `r4` is the last tested
//! element; `r13`/`r14` are restored by the epilogue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: push r14, r13 and pr.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 8c12b436..8c12b43c — r14 = *param + 56; delay slot loads the base.
    cpu.r[14] = mem.read_u32(cpu.r[4]).unwrap_or(0).wrapping_add(56);
    cpu.r[13] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);

    loop {
        // 8c12b450..8c12b45a — end = base + count*4; stop when r13 reaches it.
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 (no T bit)
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        let exhausted = cpu.r[13] == cpu.r[1]; // cmp/eq r13,r1
        cpu.set_t(exhausted);
        if exhausted {
            cpu.r[0] = 0; // mov #0,r0
            break;
        }

        // 8c12b43e..8c12b444 — 0x8c10d7c8(*r13); delay slot loads the argument.
        cpu.pr = 0x8c12b444;
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0); // delay slot of `jsr`
        crate::registry::p_8c10d7c8::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        let found = cpu.r[0] == 0; // tst r0,r0
        cpu.set_t(found);
        if found {
            cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // delay slot of `bra`
            break;
        }
        cpu.r[13] = cpu.r[13].wrapping_add(4); // add #4,r13
    }

    // Epilogue: lds.l pr; mov.l @r15+,r13; rts with mov.l @r15+,r14 as delay.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
