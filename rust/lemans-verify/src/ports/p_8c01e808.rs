//! 0x8c01e808 — free the object behind a link, then test the link's next field.
//!
//! ```text
//! 8c01e808: mov.l r14,@-r15      ; save r14
//! 8c01e80a: sts.l pr,@-r15       ; save pr
//! 8c01e80c: mov r5,r14
//! 8c01e80e: tst r14,r14
//! 8c01e810: bt 0x8c01e81c        ; null: skip straight to the epilogue
//! 8c01e812: mov.l 0x8c01e824,r0  ; r0 = 0x8c01032c (free thunk)
//! 8c01e814: jsr @r0
//! 8c01e816: mov.l @(12,r14),r4   ; delay slot: r4 = [r14+12], old r14
//! 8c01e818: add #4,r14
//! 8c01e81a: tst r14,r14          ; T = (r14+4 == 0), i.e. old r14 == -4
//! 8c01e81c: lds.l @r15+,pr
//! 8c01e81e: rts
//! 8c01e820: mov.l @r15+,r14      ; delay slot
//! ```
//!
//! The branch target sits on the epilogue's first instruction, so the null
//! path skips only the call and the increment: r0 and r4 keep their incoming
//! values, r14 is restored from the stack, and T ends as the `tst` of the
//! (zero) parameter set it — 1. The live path leaves the callee's r0 (the
//! callee is `free` through the thunk at 0x8c01032c, which never writes r0 on
//! its `r4 == 0` early exit), r4 as the callee leaves it, T = 0, and r14
//! incremented by 4 before the restore. The `jsr` delay slot dereferences the
//! *old* r14; neither path sets r0 from a return value of its own.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01e808: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c01e80a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c01e80c: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c01e80e: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c01e810: bt 0x8c01e81c
    if !cpu.t() {
        // 0x8c01e812: mov.l 0x8c01e824,r0
        cpu.r[0] = mem.read_u32(0x8c01e824).unwrap_or(0); // 0x8c01032c
        // 0x8c01e814: jsr @r0 ; 0x8c01e816: mov.l @(12,r14),r4 (delay slot)
        cpu.pr = 0x8c01e818;
        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
        crate::registry::p_8c01032c::run(cpu, mem);
        // 0x8c01e818: add #4,r14
        cpu.r[14] = cpu.r[14].wrapping_add(4);
        // 0x8c01e81a: tst r14,r14
        cpu.set_t(cpu.r[14] == 0);
    }
    // 0x8c01e81c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c01e81e: rts ; 0x8c01e820: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
