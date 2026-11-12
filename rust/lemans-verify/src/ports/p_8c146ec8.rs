//! 0x8c146ec8 — construct: init the object via 0x8c14b1f4, then install vtable.
//!
//! ```text
//! 8c146ec8: mov.l r14,@-r15
//! 8c146eca: sts.l pr,@-r15
//! 8c146ecc: mov r4,r14
//! 8c146ece: mov.l 0x8c146ee0,r0   ; 0x8c14b1f4
//! 8c146ed0: jsr @r0
//! 8c146ed2: nop                   ; delay slot
//! 8c146ed4: mov.l 0x8c146ee4,r0   ; 0x8c1ef1fc
//! 8c146ed6: mov.l r0,@r14
//! 8c146ed8: mov r14,r0
//! 8c146eda: lds.l @r15+,pr
//! 8c146edc: rts
//! 8c146ede: mov.l @r15+,r14       ; delay slot
//! ```
//!
//! The callee initializes the object at r4 and returns it; the constructor then
//! overwrites word 0 with its own vtable (0x8c1ef1fc) and returns the object in
//! r0. `r14` is saved and restored around the call, so it ends at its entry
//! value, and `r15` is balanced. No instruction here touches T.

use sh4_oracle::{Cpu, Memory};

/// Literal at 0x8c146ee0.
const CALLEE: u32 = 0x8c14b1f4;
/// Literal at 0x8c146ee4.
const VTABLE: u32 = 0x8c1ef1fc;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c146ec8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c146eca: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c146ecc: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c146ece: mov.l 0x8c146ee0,r0 (0x8c14b1f4)
    cpu.r[0] = mem.read_u32(0x8c146ee0).unwrap_or(CALLEE);
    // 0x8c146ed0: jsr @r0 ; 0x8c146ed2: nop (delay slot)
    cpu.pr = 0x8c146ed4;
    crate::registry::p_8c14b1f4::run(cpu, mem);
    // 0x8c146ed4: mov.l 0x8c146ee4,r0 (0x8c1ef1fc)
    cpu.r[0] = mem.read_u32(0x8c146ee4).unwrap_or(VTABLE);
    // 0x8c146ed6: mov.l r0,@r14
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    // 0x8c146ed8: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c146eda: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c146edc: rts ; 0x8c146ede: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
