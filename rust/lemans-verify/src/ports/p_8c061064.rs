//! 0x8c061064 — release a block when given a non-null pointer and a positive
//! size hint.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15
//! mov   r4,r14
//! tst   r14,r14   ; bt out
//! exts.w r5,r0    ; cmp/pl r0 ; bf out
//! mov.l 0x8c061084,r0 (0x8c01032c) ; jsr @r0 ; nop (delay slot)
//! out: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay slot)
//! ```
//!
//! The hint is not `r5` itself but its sign-extended low halfword, so
//! `0x0000ffff` is non-positive and skips the call. `r0` returns the original
//! `r4`. T on exit is the `tst`/`cmp/pl` residue when the call is skipped and
//! the callee's residue otherwise; the callee is reached with the original
//! `r4`/`r5` untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c061064: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c061066: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c061068: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c06106a: tst r14,r14
    let null = cpu.r[14] == 0;
    cpu.set_t(null);
    // 0x8c06106c: bt 0x8c06107a
    if !null {
        // 0x8c06106e: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c061070: cmp/pl r0
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        // 0x8c061072: bf 0x8c06107a
        if positive {
            // 0x8c061074: mov.l 0x8c061084,r0
            cpu.r[0] = mem.read_u32(0x8c061084).unwrap_or(0); // 0x8c01032c
            // 0x8c061076: jsr @r0 ; 0x8c061078: nop (delay slot)
            cpu.pr = 0x8c06107a;
            crate::registry::p_8c01032c::run(cpu, mem); // thunk to free (0x8c159fc0)
        }
    }
    // 0x8c06107a: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c06107c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c06107e: rts ; 0x8c061080: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
