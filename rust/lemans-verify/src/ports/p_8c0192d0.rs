//! 0x8c0192d0 — zero a global, then free the argument when a signed 16-bit
//! count is positive; always return the argument.
//!
//! ```text
//! mov.l r14,@-r15        ; save r14
//! sts.l pr,@-r15         ; save pr
//! mov   r4,r14
//! tst   r14,r14
//! bt    0x8c0192ec       ; null: no store, T stays 1
//! mov.l 0x8c0192f4,r3    ; r3 = 0x8c23daa0
//! mov   #0,r1
//! exts.w r5,r0           ; signed 16-bit count
//! cmp/pl r0
//! bf.s  0x8c0192ec       ; count <= 0: skip the call, T stays 0
//! mov.l r1,@r3           ; delay slot: *0x8c23daa0 = 0 on *both* paths
//! mov.l 0x8c0192f8,r0    ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0
//! nop
//! mov   r14,r0           ; return value is the original argument
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14
//! ```
//!
//! `r4` reaches free unchanged. The null path leaves `r1`/`r3` untouched and
//! `T = 1`; the negative-count path leaves `r1 = 0`, `r3 = 0x8c23daa0` and
//! `T = 0`. On the positive path `T` is whatever free leaves, and the residue
//! of `r1..r6` is free's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14 ; bt has no delay slot
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c0192f4).unwrap_or(0x8c23daa0);
        cpu.r[1] = 0;
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // bf.s delay slot: always runs
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c0192f8).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c0192ec; // jsr sets pr after its nop delay slot
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
