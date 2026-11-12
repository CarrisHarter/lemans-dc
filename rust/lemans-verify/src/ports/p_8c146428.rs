//! 0x8c146428 — free a block if the predicate allows, then return the block.
//!
//! ```text
//! mov.l r14,@-r15      ; save r14
//! sts.l pr,@-r15       ; save pr
//! mov   r4,r14         ; r14 = block
//! tst   r14,r14
//! bt    0x8c14643e     ; block == 0: skip
//! exts.w r5,r0         ; r0 = sign-extended low half of r5
//! cmp/pl r0
//! bf    0x8c14643e     ; low half <= 0: skip
//! mov.l 0x8c146448,r0  ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0            ; pr = 0x8c14643e
//! nop
//! 0x8c14643e:
//! mov   r14,r0         ; return the block
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14      ; delay slot: restore r14
//! ```
//!
//! The predicate is on the 16-bit sign extension of r5, not on r5 itself.
//! `free` is a leaf (its `rts` leaves pr alone), so the call is delegated to
//! its port with pr set to the return address. The epilogue overwrites r0 with
//! the saved block, so `free`'s r0 does not survive; T does, since the last
//! flag-setting instruction of the taken call path runs inside the callee.
//! On both skip paths T is the `tst`/`cmp/pl` result, and the epilogue leaves
//! it alone.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let zero = cpu.r[14] == 0; // tst r14,r14
    cpu.set_t(zero);
    if !zero {
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32; // exts.w r5,r0
        let positive = (cpu.r[0] as i32) > 0; // cmp/pl r0
        cpu.set_t(positive);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c146448).unwrap_or(0);
            cpu.pr = 0x8c14643e;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
