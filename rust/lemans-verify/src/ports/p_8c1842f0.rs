//! 0x8c1842f0 — release a handle and, for a positive count, free it.
//!
//! ```text
//! 8c1842f0  mov.l r14,@-r15     push r14
//! 8c1842f2  mov.l r13,@-r15     push r13
//! 8c1842f4  mov   r5,r14        r14 = count arg
//! 8c1842f6  sts.l pr,@-r15      push pr
//! 8c1842f8  mov   r4,r13        r13 = object
//! 8c1842fa  tst   r13,r13
//! 8c1842fc  bt    0x8c184312    NULL: return it
//! 8c1842fe  mov.l 0x8c18431c,r0 r0 = 0x8c018bd8 (release)
//! 8c184300  add   #84,r4        r4 = object + 0x54 (embedded handle)
//! 8c184302  jsr   @r0
//! 8c184304  mov   #-1,r5        delay: release with count -1
//! 8c184306  exts.w r14,r0       r0 = sext16(count arg)
//! 8c184308  cmp/pl r0
//! 8c18430a  bf    0x8c184312    count <= 0: keep the object
//! 8c18430c  mov.l 0x8c184320,r0 r0 = 0x8c01032c (free thunk)
//! 8c18430e  jsr   @r0
//! 8c184310  mov   r13,r4        delay: free(object)
//! 8c184312  mov   r13,r0        return the object
//! 8c184314  lds.l @r15+,pr
//! 8c184316  mov.l @r15+,r13
//! 8c184318  rts
//! 8c18431a  mov.l @r15+,r14     delay
//! ```
//!
//! The release call always receives a hard-coded -1, so it never frees the
//! embedded handle itself; it can still destroy the object the handle points at
//! when that object's refcount reaches zero. Note that `r4` is the caller's
//! original argument, not the saved `r13` copy, so `add #84` moves it to the
//! handle field at object + 0x54. The second branch re-reads the caller's r5
//! through `exts.w`, so only its low 16 bits select the path: 0x00010000 counts
//! as zero and 0xffff0001 as positive, while 0x00008000 is negative. The final
//! free receives the restored r13. The epilogue restores pr, r13 and r14 from
//! the frame, so all three end at their entry values; r0 ends as the return
//! value, and r1/r3/r4/r5/r6 keep whichever callee ran. T is the release
//! callee's last flag state overwritten by the wrapper's own `exts.w`/`cmp/pl`
//! before the second call, or the entry `tst` on the NULL path.

use sh4_oracle::{Cpu, Memory};

/// 0x8c18431c holds 0x8c018bd8 and 0x8c184320 holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c18431c;
const FREE_PTR: u32 = 0x8c184320;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1842f0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c1842f2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c1842f4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c1842f6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1842f8: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c1842fa: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c1842fc: bt 0x8c184312
    if cpu.r[13] != 0 {
        // 0x8c1842fe: mov.l 0x8c18431c,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c184300: add #84,r4
        cpu.r[4] = cpu.r[4].wrapping_add(84);
        // 0x8c184302: jsr @r0 ; 0x8c184304: mov #-1,r5 (delay)
        cpu.pr = 0x8c184306;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c184306: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c184308: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c18430a: bf 0x8c184312
        if (cpu.r[0] as i32) > 0 {
            // 0x8c18430c: mov.l 0x8c184320,r0
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE_THUNK);
            // 0x8c18430e: jsr @r0 ; 0x8c184310: mov r13,r4 (delay)
            cpu.pr = 0x8c184312;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c184312: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c184314: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c184316: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c184318: rts ; 0x8c18431a: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
