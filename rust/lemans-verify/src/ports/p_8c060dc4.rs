//! 0x8c060dc4 — release a block and, if asked, the block owner.
//!
//! ```text
//! mov.l r14,@-r15
//! mov.l r13,@-r15
//! mov   r5,r14        ; r14 = count (full 32 bits)
//! sts.l pr,@-r15
//! mov   r4,r13        ; r13 = ptr
//! tst   r13,r13
//! bt    0x8c060de4    ; ptr == NULL: skip both calls
//! mov.l 0x8c060df0,r0 ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0           ; free(*ptr)
//! mov.l @r13,r4       ; delay slot: r4 = *ptr
//! exts.w r14,r0       ; r0 = sext16(low half of count)
//! cmp/pl r0
//! bf    0x8c060de4    ; signed 16-bit count <= 0: skip the second call
//! mov.l 0x8c060df0,r0
//! jsr   @r0           ; free(ptr)
//! mov   r13,r4        ; delay slot
//! 0x8c060de4: mov r13,r0
//! lds.l @r15+,pr
//! mov.l @r15+,r13
//! rts
//! mov.l @r15+,r14     ; delay slot
//! ```
//!
//! The first free always runs for a non-NULL pointer, before the count is
//! examined; only its sign-extended low 16 bits decide the second free.
//! Return value is `ptr` on every path. T is `tst`'s set flag on the NULL path,
//! `cmp/pl`'s clear flag on the non-positive path, and the second callee's T
//! otherwise (the epilogue sets no flags).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c060dc4: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c060dc6: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c060dc8: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c060dca: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c060dcc: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c060dce: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c060dd0: bt 0x8c060de4
    if cpu.r[13] != 0 {
        // 0x8c060dd2: mov.l 0x8c060df0,r0
        cpu.r[0] = mem.read_u32(0x8c060df0).unwrap_or(0x8c01032c);
        // 0x8c060dd4: jsr @r0 ; 0x8c060dd6: mov.l @r13,r4 (delay slot)
        cpu.pr = 0x8c060dd8;
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        crate::registry::p_8c01032c::run(cpu, mem);
        // 0x8c060dd8: exts.w r14,r0 — only the low 16 bits decide
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c060dda: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c060ddc: bf 0x8c060de4
        if (cpu.r[0] as i32) > 0 {
            // 0x8c060dde: mov.l 0x8c060df0,r0
            cpu.r[0] = mem.read_u32(0x8c060df0).unwrap_or(0x8c01032c);
            // 0x8c060de0: jsr @r0 ; 0x8c060de2: mov r13,r4 (delay slot)
            cpu.pr = 0x8c060de4;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c060de4: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c060de6: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c060de8: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c060dea: rts ; 0x8c060dec: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
