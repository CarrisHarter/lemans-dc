//! 0x8c05d7bc — release a reference and optionally free a refcounted handle.
//!
//! ```text
//! 8c05d7bc  mov.l r14,@-r15        push r14
//! 8c05d7be  mov.l r13,@-r15        push r13
//! 8c05d7c0  mov   r5,r14           r14 = count arg (tested as a 16-bit value)
//! 8c05d7c2  sts.l pr,@-r15         push pr
//! 8c05d7c4  mov   r4,r13           r13 = handle
//! 8c05d7c6  tst   r13,r13
//! 8c05d7c8  bt    0x8c05d7dc       handle == NULL: return it, no release
//! 8c05d7ca  mov.l 0x8c05d7e8,r0    r0 = 0x8c018bd8 (release a reference)
//! 8c05d7cc  jsr   @r0
//! 8c05d7ce  mov   #-1,r5           delay: release with count -1
//! 8c05d7d0  exts.w r14,r0          r0 = sext16(count)
//! 8c05d7d2  cmp/pl r0
//! 8c05d7d4  bf    0x8c05d7dc       count <= 0: keep the handle allocated
//! 8c05d7d6  mov.l 0x8c05d7ec,r0    r0 = 0x8c01032c (free thunk)
//! 8c05d7d8  jsr   @r0
//! 8c05d7da  mov   r13,r4           delay: free(handle)
//! 8c05d7dc  mov   r13,r0           return the handle argument
//! 8c05d7de  lds.l @r15+,pr
//! 8c05d7e0  mov.l @r15+,r13
//! 8c05d7e2  rts
//! 8c05d7e4  mov.l @r15+,r14        delay
//! ```
//!
//! `exts.w` is what decides the free: only the low 16 bits of the count
//! argument matter, and they are sign-extended. So r5 = 0x00010000 counts as
//! zero (no free), r5 = 0xffff8000 as negative (no free), and r5 = 0xffff0001
//! as one (free). The release callee always gets r5 = -1, which decrements the
//! object refcount; the handle block itself is only freed here, after the
//! callee returns.
//!
//! Final registers: r0 = handle argument (zero on the early exit). r13 and r14
//! are restored from the frame, so they end as the *incoming* r13/r14, not as
//! the handle/count copies used internally. r15 is unchanged overall, and T is
//! the last flag written before the exit: 1 for a NULL handle, 0 for the
//! `cmp/pl` skip, and the free routine's residue when the handle is freed.

use sh4_oracle::{Cpu, Memory};

/// 0x8c05d7e8 holds 0x8c018bd8, 0x8c05d7ec holds 0x8c01032c.
const RELEASE_PTR: u32 = 0x8c05d7e8;
const FREE_PTR: u32 = 0x8c05d7ec;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05d7bc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05d7be: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05d7c0: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05d7c2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05d7c4: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05d7c6: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05d7c8: bt 0x8c05d7dc
    if cpu.r[13] != 0 {
        // 0x8c05d7ca: mov.l 0x8c05d7e8,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c05d7cc: jsr @r0 ; 0x8c05d7ce: mov #-1,r5 (delay)
        cpu.pr = 0x8c05d7d0;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c05d7d0: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c05d7d2: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05d7d4: bf 0x8c05d7dc
        if (cpu.r[0] as i32) > 0 {
            // 0x8c05d7d6: mov.l 0x8c05d7ec,r0
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE_THUNK);
            // 0x8c05d7d8: jsr @r0 ; 0x8c05d7da: mov r13,r4 (delay)
            cpu.pr = 0x8c05d7da;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05d7dc: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05d7de: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05d7e0: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05d7e2: rts ; 0x8c05d7e4: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
