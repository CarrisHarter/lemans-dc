//! 0x8c0112e0 — call the r0/r1-convention strcpy with (r4, r5) as (dst, src).
//!
//! ```text
//! sts.l pr,@-r15          ; r15 -= 4, [r15] = pr
//! mov.l 0x8c0112f0,r3     ; r3 = 0x8c016484
//! mov r5,r1               ; r1 = src
//! jsr @r3                 ; pr = 0x8c0112ea
//! mov r4,r0               ; delay slot: r0 = dst
//! lds.l @r15+,pr          ; after the callee returns
//! rts
//! nop
//! ```
//!
//! 0x8c016484 does not use the normal argument registers: it reads the
//! destination from `r0` and the source from `r1`, which is the only reason
//! this thunk shuffles `r4`/`r5` into place. The callee's aligned path
//! tail-jumps into 0x8c0163e8, whose four pops unwind the callee's two saved
//! registers plus its own two, so the callee leaves `r15` exactly at our
//! saved-PR slot and the thunk's own pop returns the caller's `pr`; the
//! thunk is therefore a plain call, not a frame-owning one.
//!
//! Residue: `r0` = dst (callee), `r1` = one past the copied NUL, `r3` = the
//! loaded literal 0x8c016484 (both callee paths restore their saved `r3`),
//! `r4`/`r5`/`r6` keep their entry values (the aligned tail's pushes and pops
//! restore them), and `T` is the callee's exit flag.

use sh4_oracle::{Cpu, Memory};

/// Entry of 0x8c016484, also the word in the literal pool at 0x8c0112f0.
const CALLEE: u32 = 0x8c016484;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c0112f0).unwrap_or(CALLEE); // mov.l 0x8c0112f0,r3
    cpu.r[1] = cpu.r[5]; // mov r5,r1
    cpu.pr = 0x8c0112ea; // jsr @r3
    cpu.r[0] = cpu.r[4]; // delay slot: mov r4,r0
    crate::registry::p_8c016484::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts; delay slot 0x8c0112ee: nop
}
