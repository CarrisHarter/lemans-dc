//! 0x8c184260 — clear a 12-byte object and publish it as the current one.
//!
//! ```text
//! mov.l r14,@-r15          ; push r14
//! sts.l pr,@-r15           ; push pr
//! add #-4,r15              ; one scratch word, handed to the callee as r5
//! mov r4,r14               ; r14 = object
//! mov.l 0x8c184284,r0      ; r0 = 0x8c01e484
//! jsr @r0
//! mov r15,r5               ; delay slot: r5 = scratch
//! mov #0,r3
//! mov.l r3,@r14            ; object[0] = 0
//! mov r14,r0               ; return object
//! mov.l 0x8c184288,r1      ; r1 = &current (0x8c2b72a0)
//! mov.l r3,@(4,r14)        ; object[1] = 0
//! mov.l r3,@(8,r14)        ; object[2] = 0
//! mov.l r14,@r1            ; current = object
//! add #4,r15
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14          ; delay slot
//! ```
//!
//! 0x8c01e484 is `rts ; mov r4,r0`: the call changes nothing the caller
//! observes (r0 is overwritten at 0x8c184272), but it is dispatched through the
//! registry so the port composes like the original. No instruction here or in
//! the callee sets T, so the exit flag is whatever the caller passed in.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c184260: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c184262: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c184264: add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c184266: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c184268: mov.l 0x8c184284,r0
    cpu.r[0] = mem.read_u32(0x8c184284).unwrap_or(0); // 0x8c01e484
    // 0x8c18426a: jsr @r0 ; 0x8c18426c: mov r15,r5 (delay slot)
    cpu.pr = 0x8c18426e;
    cpu.r[5] = cpu.r[15];
    crate::registry::p_8c01e484::run(cpu, mem);
    // 0x8c18426e: mov #0,r3
    cpu.r[3] = 0;
    // 0x8c184270: mov.l r3,@r14
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]);
    // 0x8c184272: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c184274: mov.l 0x8c184288,r1
    cpu.r[1] = mem.read_u32(0x8c184288).unwrap_or(0); // 0x8c2b72a0
    // 0x8c184276: mov.l r3,@(4,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]);
    // 0x8c184278: mov.l r3,@(8,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[3]);
    // 0x8c18427a: mov.l r14,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[14]);
    // 0x8c18427c: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18427e: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c184280: rts ; 0x8c184282: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
