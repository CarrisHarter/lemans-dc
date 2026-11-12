//! 0x8c078c68 — release two embedded handles, then free the owning object.
//!
//! ```text
//! 8c078c68  mov.l r14,@-r15        push r14
//! 8c078c6a  mov.l r13,@-r15        push r13
//! 8c078c6c  mov   r5,r14           r14 = count arg (short in low half)
//! 8c078c6e  sts.l pr,@-r15         push pr
//! 8c078c70  mov   r4,r13           r13 = object
//! 8c078c72  tst   r13,r13
//! 8c078c74  bt    0x8c078c94       NULL object: return it
//! 8c078c76  mov.l 0x8c078ca0,r0    r0 = 0x8c018bd8 (release)
//! 8c078c78  add   #8,r4            r4 = object + 8
//! 8c078c7a  jsr   @r0
//! 8c078c7c  mov   #-1,r5           delay: release(&object->h2, -1)
//! 8c078c7e  mov.l 0x8c078ca0,r0
//! 8c078c80  mov   r13,r4
//! 8c078c82  add   #4,r4            r4 = object + 4
//! 8c078c84  jsr   @r0
//! 8c078c86  mov   #-1,r5           delay: release(&object->h1, -1)
//! 8c078c88  exts.w r14,r0          only the low half of the count matters
//! 8c078c8a  cmp/pl r0
//! 8c078c8c  bf    0x8c078c94       count <= 0: keep the object
//! 8c078c8e  mov.l 0x8c078ca4,r0    r0 = 0x8c01032c (free thunk)
//! 8c078c90  jsr   @r0
//! 8c078c92  mov   r13,r4           delay: free(object)
//! 8c078c94  mov   r13,r0           return the object
//! 8c078c96  lds.l @r15+,pr
//! 8c078c98  mov.l @r15+,r13
//! 8c078c9a  rts
//! 8c078c9c  mov.l @r15+,r14       delay
//! ```
//!
//! `r4 == 0` short-circuits everything, including the two releases, and leaves
//! the T bit from the entry `tst`. Otherwise the handles at `object+8` and
//! `object+4` are released with count -1; their callee residue is visible in
//! the epilogue. The final `exts.w`/`cmp/pl` only overwrite r0 and T, and when
//! the low 16 bits of the count are positive the object itself is freed
//! through the 0x8c01032c thunk, so that free routine's residue reaches the
//! epilogue too.
//!
//! Every `jsr` writes pr with its return address; the callees save and restore
//! it, and the epilogue reloads the seeded pr from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c078c68: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c078c6a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c078c6c: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c078c6e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c078c70: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c078c72: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c078c74: bt 0x8c078c94
    if cpu.r[13] != 0 {
        // 0x8c078c76: mov.l 0x8c078ca0,r0
        cpu.r[0] = mem.read_u32(0x8c078ca0).unwrap_or(0x8c018bd8);
        // 0x8c078c78: add #8,r4
        cpu.r[4] = cpu.r[13].wrapping_add(8);
        // 0x8c078c7a: jsr @r0 ; 0x8c078c7c: mov #-1,r5 (delay)
        cpu.pr = 0x8c078c7e;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c078c7e: mov.l 0x8c078ca0,r0
        cpu.r[0] = mem.read_u32(0x8c078ca0).unwrap_or(0x8c018bd8);
        // 0x8c078c80: mov r13,r4 ; 0x8c078c82: add #4,r4
        cpu.r[4] = cpu.r[13].wrapping_add(4);
        // 0x8c078c84: jsr @r0 ; 0x8c078c86: mov #-1,r5 (delay)
        cpu.pr = 0x8c078c88;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c078c88: exts.w r14,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c078c8a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c078c8c: bf 0x8c078c94
        if (cpu.r[0] as i32) > 0 {
            // 0x8c078c8e: mov.l 0x8c078ca4,r0
            cpu.r[0] = mem.read_u32(0x8c078ca4).unwrap_or(0x8c01032c);
            // 0x8c078c90: jsr @r0 ; 0x8c078c92: mov r13,r4 (delay)
            cpu.pr = 0x8c078c94;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c078c94: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c078c96: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c078c98: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c078c9a: rts ; 0x8c078c9c: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
