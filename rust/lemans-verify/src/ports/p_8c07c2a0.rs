//! 0x8c07c2a0 — release a handle, then optionally free the handle itself.
//!
//! ```text
//! 8c07c2a0  mov.l r14,@-r15        push r14
//! 8c07c2a2  mov.l r13,@-r15        push r13
//! 8c07c2a4  mov   r5,r14           r14 = count argument (full word)
//! 8c07c2a6  sts.l pr,@-r15         push pr
//! 8c07c2a8  mov   r4,r13           r13 = handle
//! 8c07c2aa  tst   r13,r13
//! 8c07c2ac  bt    0x8c07c2c0       NULL handle: return it untouched
//! 8c07c2ae  mov.l 0x8c07c2cc,r0    r0 = 0x8c018bd8 (release thunk)
//! 8c07c2b0  jsr   @r0
//! 8c07c2b2  mov   #-1,r5           delay: release with count -1
//! 8c07c2b4  exts.w r14,r0          r0 = sext16(count argument)
//! 8c07c2b6  cmp/pl r0
//! 8c07c2b8  bf    0x8c07c2c0       count <= 0: keep the handle
//! 8c07c2ba  mov.l 0x8c07c2d0,r0    r0 = 0x8c01032c (free thunk)
//! 8c07c2bc  jsr   @r0
//! 8c07c2be  mov   r13,r4           delay: free(handle)
//! 8c07c2c0  mov   r13,r0           return the handle
//! 8c07c2c2  lds.l @r15+,pr
//! 8c07c2c4  mov.l @r15+,r13
//! 8c07c2c6  rts
//! 8c07c2c8  mov.l @r15+,r14        delay
//! ```
//!
//! The release call is always made with count -1, so it clears `*handle` when a
//! reference remains and destroys the object when the refcount hits zero; it
//! never frees the handle itself. Whether the *caller* then frees the handle is
//! decided by `exts.w` of the caller's own `r5`: only the low 16 bits count, so
//! `r5 = 0x00010000` holds the handle while `r5 = 0xffff0001` frees it.
//!
//! The exit flags are the caller's `tst` on the NULL path, its `cmp/pl` on the
//! hold path, and the free callee's flags on the free path — nothing between the
//! second `jsr` and `rts` touches T.

use sh4_oracle::{Cpu, Memory};

/// Literal pool slots at 0x8c07c2cc / 0x8c07c2d0.
const RELEASE_PTR: u32 = 0x8C07_C2CC;
const FREE_PTR: u32 = 0x8C07_C2D0;
const RELEASE: u32 = 0x8C01_8BD8;
const FREE: u32 = 0x8C01_032C;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07c2a0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07c2a2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07c2a4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07c2a6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07c2a8: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07c2aa: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c07c2ac: bt 0x8c07c2c0
    if cpu.r[13] != 0 {
        // 0x8c07c2ae: mov.l 0x8c07c2cc,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c07c2b0: jsr @r0 ; 0x8c07c2b2: mov #-1,r5 (delay)
        cpu.pr = 0x8C07_C2B4;
        cpu.r[5] = u32::MAX;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c07c2b4: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07c2b6: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07c2b8: bf 0x8c07c2c0
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07c2ba: mov.l 0x8c07c2d0,r0
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE);
            // 0x8c07c2bc: jsr @r0 ; 0x8c07c2be: mov r13,r4 (delay)
            cpu.pr = 0x8C07_C2BE;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07c2c0: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c07c2c2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07c2c4: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07c2c6: rts ; 0x8c07c2c8: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
