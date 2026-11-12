//! 0x8c1bb264 — release a handle and, for a positive short count, free it.
//!
//! ```text
//! 8c1bb264  mov.l r14,@-r15      push r14
//! 8c1bb266  mov.l r13,@-r15      push r13
//! 8c1bb268  mov   r5,r14         r14 = count arg (short in the low half)
//! 8c1bb26a  sts.l pr,@-r15       push pr
//! 8c1bb26c  mov   r4,r13         r13 = handle
//! 8c1bb26e  tst   r13,r13
//! 8c1bb270  bt    0x8c1bb284     handle == NULL: return it
//! 8c1bb272  mov.l 0x8c1bb290,r0  r0 = 0x8c018bd8 (release handle)
//! 8c1bb274  jsr   @r0
//! 8c1bb276  mov   #-1,r5         delay: release(handle, -1)
//! 8c1bb278  exts.w r14,r0        r0 = sext16(count arg)
//! 8c1bb27a  cmp/pl r0
//! 8c1bb27c  bf    0x8c1bb284     count <= 0: keep the handle
//! 8c1bb27e  mov.l 0x8c1bb294,r0  r0 = 0x8c01032c (free thunk)
//! 8c1bb280  jsr   @r0
//! 8c1bb282  mov   r13,r4         delay: free(handle)
//! 8c1bb284  mov   r13,r0         return the handle
//! 8c1bb286  lds.l @r15+,pr
//! 8c1bb288  mov.l @r15+,r13
//! 8c1bb28a  rts
//! 8c1bb28c  mov.l @r15+,r14      delay
//! ```
//!
//! The release call always passes -1, so the release routine never frees the
//! handle by itself; only the outer `exts.w`/`cmp/pl` on the caller's r5 can
//! reach the free thunk. `exts.w` makes just the low 16 bits matter, signed:
//! r5 = 0x00010000 counts as zero and leaves the handle allocated, while
//! r5 = 0xffff0001 is positive and frees it.
//!
//! Both callees save r12/r13/r14 and pr (the free thunk delegates to the
//! dlmalloc free), so the epilogue restores r13, r14 and pr to their entry
//! values; only r0..r7 and T carry the last callee's residue.

use sh4_oracle::{Cpu, Memory};

/// 0x8c1bb290 holds 0x8c018bd8, 0x8c1bb294 holds 0x8c01032c.
const RELEASE_LITERAL: u32 = 0x8c1bb290;
const FREE_THUNK_LITERAL: u32 = 0x8c1bb294;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1bb264: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c1bb266: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c1bb268: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c1bb26a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1bb26c: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c1bb26e: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c1bb270: bt 0x8c1bb284
    if cpu.r[13] != 0 {
        // 0x8c1bb272: mov.l 0x8c1bb290,r0
        cpu.r[0] = rd(mem, RELEASE_LITERAL, 0x8c018bd8);
        // 0x8c1bb274: jsr @r0 ; 0x8c1bb276: mov #-1,r5 (delay)
        cpu.pr = 0x8c1bb278;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c1bb278: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c1bb27a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c1bb27c: bf 0x8c1bb284
        if (cpu.r[0] as i32) > 0 {
            // 0x8c1bb27e: mov.l 0x8c1bb294,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c1bb280: jsr @r0 ; 0x8c1bb282: mov r13,r4 (delay)
            cpu.pr = 0x8c1bb284;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c1bb284: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c1bb286: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1bb288: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1bb28a: rts ; 0x8c1bb28c: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
