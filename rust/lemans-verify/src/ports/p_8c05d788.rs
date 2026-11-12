//! 0x8c05d788 — release a handle and, for a positive count, free it.
//!
//! ```text
//! 8c05d788  mov.l r14,@-r15     push r14
//! 8c05d78a  mov.l r13,@-r15     push r13
//! 8c05d78c  mov   r5,r14        r14 = count arg
//! 8c05d78e  sts.l pr,@-r15      push pr
//! 8c05d790  mov   r4,r13        r13 = handle
//! 8c05d792  tst   r13,r13
//! 8c05d794  bt    0x8c05d7a8    NULL: return it
//! 8c05d796  mov.l 0x8c05d7b4,r0 r0 = 0x8c018bd8 (release)
//! 8c05d798  jsr   @r0
//! 8c05d79a  mov   #-1,r5        delay: release with count -1
//! 8c05d79c  exts.w r14,r0       r0 = sext16(count arg)
//! 8c05d79e  cmp/pl r0
//! 8c05d7a0  bf    0x8c05d7a8    count <= 0: keep the handle
//! 8c05d7a2  mov.l 0x8c05d7b8,r0 r0 = 0x8c01032c (free thunk)
//! 8c05d7a4  jsr   @r0
//! 8c05d7a6  mov   r13,r4        delay: free(handle)
//! 8c05d7a8  mov   r13,r0        return the handle
//! 8c05d7aa  lds.l @r15+,pr
//! 8c05d7ac  mov.l @r15+,r13
//! 8c05d7ae  rts
//! 8c05d7b0  mov.l @r15+,r14     delay
//! ```
//!
//! The release call always receives a hard-coded -1, so it never frees the
//! handle; it can still destroy the object the handle points at when that
//! object's refcount reaches zero. The second branch re-reads the caller's r5
//! through `exts.w`, so only its low 16 bits select the path: 0x00010000 counts
//! as zero and 0xffff0001 as positive, while 0x00008000 is negative. The final
//! free(handle) receives the restored r13. The epilogue restores pr and r14
//! from the frame, so both end at their seed values; r0 ends as the return
//! value, and r4/r1/r3/r5/r6 keep whichever callee ran. T is the last flag
//! state left by the release callee, overwritten by the wrapper's own
//! `exts.w`/`cmp/pl` before the second call, or the entry `tst` on the NULL
//! path.

use sh4_oracle::{Cpu, Memory};

/// 0x8c05d7b4 holds 0x8c018bd8 and 0x8c05d7b8 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c05d7b4;
const FREE_PTR: u32 = 0x8c05d7b8;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05d788: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05d78a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05d78c: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05d78e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05d790: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05d792: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05d794: bt 0x8c05d7a8
    if cpu.r[13] != 0 {
        // 0x8c05d796: mov.l 0x8c05d7b4,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c05d798: jsr @r0 ; 0x8c05d79a: mov #-1,r5 (delay)
        cpu.pr = 0x8c05d79c;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c05d79c: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c05d79e: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05d7a0: bf 0x8c05d7a8
        if (cpu.r[0] as i32) > 0 {
            // 0x8c05d7a2: mov.l 0x8c05d7b8,r0
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE_THUNK);
            // 0x8c05d7a4: jsr @r0 ; 0x8c05d7a6: mov r13,r4 (delay)
            cpu.pr = 0x8c05d7a8;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05d7a8: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05d7aa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05d7ac: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05d7ae: rts ; 0x8c05d7b0: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
