//! 0x8c01e078 — free a block when it is non-null and the count is positive.
//!
//! ```text
//! 8c01e078  mov.l r14,@-r15
//! 8c01e07a  sts.l pr,@-r15
//! 8c01e07c  mov   r4,r14
//! 8c01e07e  tst   r14,r14
//! 8c01e080  bt    0x8c01e08e      ; null block: skip the call
//! 8c01e082  exts.w r5,r0          ; r0 = sign-extended count
//! 8c01e084  cmp/pl r0
//! 8c01e086  bf    0x8c01e08e      ; count <= 0: skip the call
//! 8c01e088  mov.l 0x8c01e098,r0   ; r0 = 0x8c01032c (thunk to free)
//! 8c01e08a  jsr   @r0
//! 8c01e08c  nop                   ; delay slot
//! 8c01e08e  mov   r14,r0
//! 8c01e090  lds.l @r15+,pr
//! 8c01e092  rts
//! 8c01e094  mov.l @r15+,r14       ; delay slot
//! ```
//!
//! The two early exits only differ in T and in the exts.w residue, which the
//! trailing `mov r14,r0` overwrites. On the call path the callee's residue
//! (r0..r6, T) is part of the result: nothing after the call restores it, and
//! the caller's `rts` leaves T exactly as the callee left it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if !cpu.t() {
        cpu.r[0] = (cpu.r[5] as u16 as i16 as i32) as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c01e098).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c01e08e; // jsr return address; delay slot is a nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
