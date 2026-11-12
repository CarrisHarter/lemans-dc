//! 0x8c07fd24 — free a block when the block pointer is non-null and the
//! caller's flag argument (low 16 bits, sign-extended) is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15
//! r14 = r4 ; tst r14,r14 ; bt out
//! r0 = (int16)r5 ; cmp/pl r0 ; bf out
//! r0 = 0x8c01032c ; jsr @r0 ; nop (delay slot)
//! out: r0 = r14 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay slot)
//! ```
//!
//! The call reaches the free thunk at 0x8c01032c, which tail-jumps to the real
//! free at 0x8c159fc0 with `r4` unchanged. `r14` is callee-saved, so the
//! callee must leave it at the block pointer; the epilogue then restores the
//! caller's `r14` and `pr` from the stack. T ends as `tst` left it on the
//! null-block path, as `cmp/pl` left it on the non-positive-flag path, and as
//! free's final comparison left it on the call path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07fd24: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07fd26: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07fd28: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c07fd2a: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c07fd2c: bt 0x8c07fd3a
    if !cpu.t() {
        // 0x8c07fd2e: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as u16 as i16 as i32 as u32;
        // 0x8c07fd30: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07fd32: bf 0x8c07fd3a
        if cpu.t() {
            // 0x8c07fd34: mov.l 0x8c07fd44,r0
            cpu.r[0] = mem.read_u32(0x8c07fd44).unwrap_or(0); // 0x8c01032c
            // 0x8c07fd36: jsr @r0 ; 0x8c07fd38: nop (delay slot)
            cpu.pr = 0x8c07fd3a;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07fd3a: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c07fd3c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07fd3e: rts ; 0x8c07fd40: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
