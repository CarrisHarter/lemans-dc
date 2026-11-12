//! 0x8c0d21e4 — release a refcounted handle, optionally freeing it.
//!
//! ```text
//! 8c0d21e4  mov.l r14,@-r15        push r14
//! 8c0d21e6  mov.l r13,@-r15        push r13
//! 8c0d21e8  mov   r5,r14           r14 = count arg (short in low half)
//! 8c0d21ea  sts.l pr,@-r15         push pr
//! 8c0d21ec  mov   r4,r13           r13 = handle
//! 8c0d21ee  tst   r13,r13
//! 8c0d21f0  bt    0x8c0d2204       handle == NULL: return it
//! 8c0d21f2  mov.l 0x8c0d2210,r0    r0 = 0x8c018bd8 (release)
//! 8c0d21f4  jsr   @r0
//! 8c0d21f6  mov   #-1,r5           delay: release with count -1
//! 8c0d21f8  exts.w r14,r0          r0 = sext16(count arg)
//! 8c0d21fa  cmp/pl r0
//! 8c0d21fc  bf    0x8c0d2204       count <= 0: keep the handle
//! 8c0d21fe  mov.l 0x8c0d2214,r0    r0 = 0x8c01032c (free thunk)
//! 8c0d2200  jsr   @r0
//! 8c0d2202  mov   r13,r4           delay: free(handle)
//! 8c0d2204  mov   r13,r0           return the handle
//! 8c0d2206  lds.l @r15+,pr
//! 8c0d2208  mov.l @r15+,r13
//! 8c0d220a  rts
//! 8c0d220c  mov.l @r15+,r14        delay
//! ```
//!
//! `0xffffffff` for the release count means the release path never frees the
//! handle itself (its final `exts.w` sees -1); the wrapper then decides from
//! the caller's own count whether the handle is freed. Only the low 16 bits
//! of that count matter, so e.g. 0x00010000 keeps the handle and 0x00008001
//! frees it. The handle-returning `mov r13,r0` runs on every path, and the
//! epilogue restores `r13`, `r14` and `pr`, so they end at their entry values.
//! The two callees are the ported reference-release routine (0x8c018bd8) and
//! the free thunk (0x8c01032c); whatever flags they leave behind are visible
//! only through the skipped epilogue instructions, so the last `cmp/pl` or
//! callee flag state must pass through untouched.

use sh4_oracle::{Cpu, Memory};

/// 0x8c0d2210 holds 0x8c018bd8, 0x8c0d2214 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c0d2210;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK_PTR: u32 = 0x8c0d2214;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d21e4: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0d21e6: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0d21e8: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0d21ea: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0d21ec: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0d21ee: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0d21f0: bt 0x8c0d2204
    if cpu.r[13] != 0 {
        // 0x8c0d21f2: mov.l 0x8c0d2210,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c0d21f4: jsr @r0 ; 0x8c0d21f6: mov #-1,r5 (delay)
        cpu.pr = 0x8c0d21f8;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c0d21f8: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as u16 as i16 as i32 as u32;
        // 0x8c0d21fa: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0d21fc: bf 0x8c0d2204
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0d21fe: mov.l 0x8c0d2214,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0d2200: jsr @r0 ; 0x8c0d2202: mov r13,r4 (delay)
            cpu.pr = 0x8c0d2204;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0d2204: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0d2206: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d2208: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d220a: rts ; 0x8c0d220c: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
