//! 0x8c13ec90 — free(ptr) if `ptr != NULL` and `(i16)count > 0`; return `ptr`.
//!
//! ```text
//! 8c13ec90  mov.l r14,@-r15     ; push r14
//! 8c13ec92  sts.l pr,@-r15      ; push pr
//! 8c13ec94  mov   r4,r14        ; r14 = ptr
//! 8c13ec96  tst   r14,r14
//! 8c13ec98  bt    0x8c13eca6    ; ptr == NULL: skip the call
//! 8c13ec9a  exts.w r5,r0        ; r0 = sext16(low half of count)
//! 8c13ec9c  cmp/pl r0
//! 8c13ec9e  bf    0x8c13eca6    ; signed 16-bit count <= 0: skip the call
//! 8c13eca0  mov.l 0x8c13ecb0,r0 ; r0 = 0x8c01032c (free thunk)
//! 8c13eca2  jsr   @r0
//! 8c13eca4  nop                 ; delay slot
//! 8c13eca6  mov   r14,r0
//! 8c13eca8  lds.l @r15+,pr
//! 8c13ecaa  rts
//! 8c13ecac  mov.l @r15+,r14     ; delay slot
//! 8c13ecb0  .long 0x8c01032c
//! ```
//!
//! Only the *low 16 bits* of r5 are tested: `exts.w r5,r0` makes r5=0x0001_0000
//! non-positive (no call) and r5=0xffff_0001 positive (call). The callee is the
//! 0x8c01032c thunk, which tail-calls free at 0x8c159fc0 with r4 and r5 exactly
//! as they are here; the jsr stores 0x8c13eca6 in pr and its delay slot is a
//! nop. The epilogue restores pr and r14 from the frame, so both leave with
//! their entry values and r15 is back where it started; r0 is the argument on
//! every path. T is `tst`'s set flag on the NULL path, `cmp/pl`'s clear flag on
//! the non-positive path, and the callee's own T after a call.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c13ec90: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c13ec92: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c13ec94: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c13ec96: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c13ec98: bt 0x8c13eca6
    if cpu.r[14] != 0 {
        // 0x8c13ec9a: exts.w r5,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c13ec9c: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c13ec9e: bf 0x8c13eca6
        if (cpu.r[0] as i32) > 0 {
            // 0x8c13eca0: mov.l 0x8c13ecb0,r0
            cpu.r[0] = mem.read_u32(0x8c13ecb0).unwrap_or(0x8c01032c);
            // 0x8c13eca2: jsr @r0 ; 0x8c13eca4: nop (delay slot)
            cpu.pr = 0x8c13eca6;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c13eca6: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c13eca8: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c13ecaa: rts ; 0x8c13ecac: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
