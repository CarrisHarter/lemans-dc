//! 0x8c07e94c — release a refcounted handle and optionally free it.
//!
//! ```text
//! 8c07e94c  mov.l r14,@-r15      push r14
//! 8c07e94e  mov.l r13,@-r15      push r13
//! 8c07e950  mov   r5,r14         r14 = second argument
//! 8c07e952  sts.l pr,@-r15       push pr
//! 8c07e954  mov   r4,r13         r13 = handle
//! 8c07e956  tst   r13,r13
//! 8c07e958  bt    0x8c07e96c     handle == NULL: return it untouched
//! 8c07e95a  mov.l 0x8c07e978,r0  r0 = 0x8c018bd8
//! 8c07e95c  jsr   @r0
//! 8c07e95e  mov   #-1,r5         delay: release(handle, -1)
//! 8c07e960  exts.w r14,r0        r0 = sext16(second argument)
//! 8c07e962  cmp/pl r0
//! 8c07e964  bf    0x8c07e96c     non-positive: keep the handle
//! 8c07e966  mov.l 0x8c07e97c,r0  r0 = 0x8c01032c (free thunk)
//! 8c07e968  jsr   @r0
//! 8c07e96a  mov   r13,r4         delay: free(handle)
//! 8c07e96c  mov   r13,r0         return the handle
//! 8c07e96e  lds.l @r15+,pr
//! 8c07e970  mov.l @r15+,r13
//! 8c07e972  rts
//! 8c07e974  mov.l @r15+,r14      delay
//! ```
//!
//! The refcount decrement always happens through 0x8c018bd8, called with a
//! count of -1 so that the callee decrements the object's refcount but never
//! frees the handle itself. Only the sign-extended low half of the second
//! argument decides the second step: when it is positive the handle is handed
//! to the free thunk at 0x8c01032c. `r0` returns the (possibly unchanged)
//! handle, and the T flag left by the last executed branch producer survives:
//! the `tst` (handle == NULL), the `cmp/pl` (non-positive count), or the final
//! callee (free path).

use sh4_oracle::{Cpu, Memory};

/// 0x8c07e978 holds 0x8c018bd8, 0x8c07e97c holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c07e978;
const FREE_THUNK_PTR: u32 = 0x8c07e97c;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07e94c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07e94e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07e950: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07e952: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07e954: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07e956: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c07e958: bt 0x8c07e96c
    if cpu.r[13] != 0 {
        // 0x8c07e95a: mov.l 0x8c07e978,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c07e95c: jsr @r0 ; 0x8c07e95e: mov #-1,r5 (delay)
        cpu.pr = 0x8c07e960;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c07e960: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07e962: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07e964: bf 0x8c07e96c
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07e966: mov.l 0x8c07e97c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c07e968: jsr @r0 ; 0x8c07e96a: mov r13,r4 (delay)
            cpu.pr = 0x8c07e96c;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07e96c: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c07e96e: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07e970: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07e972: rts ; 0x8c07e974: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
