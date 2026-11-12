//! 0x8c0ff594 — free a block when the signed 16-bit length is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15
//! mov   r4,r14 ; tst r14,r14 ; bt out        ; r4 == 0: nothing to free
//! exts.w r5,r0 ; cmp/pl r0 ; bf out          ; length <= 0: nothing to free
//! mov.l 0x8c0ff5b4,r0 (0x8c01032c) ; jsr @r0 ; nop
//! out: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay slot)
//! ```
//!
//! Returns the pointer it was handed. T at the exit is `tst`'s on the null arm,
//! `cmp/pl`'s on the non-positive arm, and the callee's when free ran: the
//! `mov r14,r0` / register restores after the call set no flag, so the last
//! flag-setting instruction of the call path belongs to the callee.
//!
//! Only the low 16 bits of r5 are examined (`exts.w`), so a caller passing a
//! larger positive word with a zero or negative low half frees nothing.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0ff594: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0ff596: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0ff598: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0ff59a: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c0ff59c: bt 0x8c0ff5aa
    if !cpu.t() {
        // 0x8c0ff59e: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as u16 as i16 as i32 as u32;
        // 0x8c0ff5a0: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0ff5a2: bf 0x8c0ff5aa
        if cpu.t() {
            // 0x8c0ff5a4: mov.l 0x8c0ff5b4,r0 (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8c0ff5b4).unwrap_or(0x8c01032c);
            // 0x8c0ff5a6: jsr @r0 ; 0x8c0ff5a8: nop (delay slot)
            cpu.pr = 0x8c0ff5aa;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0ff5aa: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0ff5ac: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0ff5ae: rts ; 0x8c0ff5b0: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
