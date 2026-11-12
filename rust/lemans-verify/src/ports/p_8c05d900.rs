//! 0x8c05d900 — release a handle embedded at `param_1 + 4`, then optionally
//! free `param_1`; return `param_1`.
//!
//! ```text
//! 8c05d900  mov.l r14,@-r15      push r14
//! 8c05d902  mov.l r13,@-r15      push r13
//! 8c05d904  mov   r5,r14         r14 = flag arg (only its low half matters)
//! 8c05d906  sts.l pr,@-r15       push pr
//! 8c05d908  mov   r4,r13         r13 = param_1
//! 8c05d90a  tst   r13,r13
//! 8c05d90c  bt    0x8c05d922     param_1 == NULL: return it
//! 8c05d90e  mov.l 0x8c05d92c,r0  r0 = 0x8c018bd8 (release-handle)
//! 8c05d910  add   #4,r4          r4 = &handle embedded at param_1 + 4
//! 8c05d912  jsr   @r0
//! 8c05d914  mov   #-1,r5         delay: count -1, never free the embedded handle
//! 8c05d916  exts.w r14,r0        r0 = sext16(flag)
//! 8c05d918  cmp/pl r0
//! 8c05d91a  bf    0x8c05d922     flag <= 0: keep param_1
//! 8c05d91c  mov.l 0x8c05d930,r0  r0 = 0x8c01032c (free thunk)
//! 8c05d91e  jsr   @r0
//! 8c05d920  mov   r13,r4         delay: free(param_1)
//! 8c05d922  mov   r13,r0         return param_1
//! 8c05d924  lds.l @r15+,pr
//! 8c05d926  mov.l @r15+,r13
//! 8c05d928  rts
//! 8c05d92a  mov.l @r15+,r14     delay
//! ```
//!
//! The flag test is `exts.w`, so only the low 16 bits of r5 count: r5 =
//! 0x00010000 exits without freeing param_1 while r5 = 0xffff0001 frees it.
//! The handle release runs with count -1, so it decrements the object
//! refcount and may destroy the object but never frees the embedded handle.
//! Both `jsr`s push pr from their own return address; the epilogue then
//! restores the caller's pr (the seed value) from the frame.

use sh4_oracle::{Cpu, Memory};

/// 0x8c05d92c holds 0x8c018bd8, 0x8c05d930 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c05d92c;
const FREE_PTR: u32 = 0x8c05d930;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05d900: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05d902: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05d904: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05d906: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05d908: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05d90a: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05d90c: bt 0x8c05d922
    if cpu.r[13] != 0 {
        // 0x8c05d90e: mov.l 0x8c05d92c,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c05d910: add #4,r4
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        // 0x8c05d912: jsr @r0 ; 0x8c05d914: mov #-1,r5 (delay)
        cpu.pr = 0x8c05d916;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c05d916: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c05d918: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05d91a: bf 0x8c05d922
        if (cpu.r[0] as i32) > 0 {
            // 0x8c05d91c: mov.l 0x8c05d930,r0
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE_THUNK);
            // 0x8c05d91e: jsr @r0 ; 0x8c05d920: mov r13,r4 (delay)
            cpu.pr = 0x8c05d922;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05d922: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05d924: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05d926: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05d928: rts ; 0x8c05d92a: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
