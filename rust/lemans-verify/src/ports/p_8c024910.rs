//! 0x8c024910 — reset an object header, release its embedded handle, then free
//! the object when the signed halfword flag is positive.
//!
//! ```text
//! 8c024910  mov.l r14,@-r15        push r14
//! 8c024912  mov.l r13,@-r15        push r13
//! 8c024914  mov   r4,r14           r14 = object
//! 8c024916  sts.l pr,@-r15         push pr
//! 8c024918  mov   r5,r13           r13 = flag arg
//! 8c02491a  tst   r14,r14
//! 8c02491c  bt    0x8c02493e       NULL object: return it
//! 8c02491e  mov.l 0x8c024948,r1    r1 = 0x8c1e88a0
//! 8c024920  add   #16,r4           r4 = &object->handle
//! 8c024922  mov.l 0x8c02494c,r0    r0 = 0x8c018bd8 (release)
//! 8c024924  mov   #-1,r5
//! 8c024926  jsr   @r0
//! 8c024928  mov.l r1,@r14          delay: *object = 0x8c1e88a0
//! 8c02492a  mov.l 0x8c024950,r0    r0 = 0x8c05ad54 (clear and maybe free)
//! 8c02492c  mov   r14,r4
//! 8c02492e  jsr   @r0
//! 8c024930  mov   #0,r5            delay: r5 = 0, so that call never frees
//! 8c024932  exts.w r13,r0          only the low half of the flag matters
//! 8c024934  cmp/pl r0
//! 8c024936  bf    0x8c02493e       flag <= 0: keep the object
//! 8c024938  mov.l 0x8c024954,r0    r0 = 0x8c01032c (free thunk)
//! 8c02493a  jsr   @r0
//! 8c02493c  mov   r14,r4           delay: free(object)
//! 8c02493e  mov   r14,r0           return the object
//! 8c024940  lds.l @r15+,pr
//! 8c024942  mov.l @r15+,r13
//! 8c024944  rts
//! 8c024946  mov.l @r15+,r14       delay
//! ```
//!
//! The 0x8c1e88a0 store sits in the first `jsr` delay slot, so it lands before
//! 0x8c018bd8 runs; the release then decrements the handle's refcount and may
//! destroy the pointee, and 0x8c05ad54 overwrites `*object` with its own
//! sentinel (through 0x8c056bcc) while passing count 0, so it never frees.
//! Only the `exts.w` halfword of the flag decides the final free, e.g.
//! r5 = 0xffff0001 is positive while r5 = 0x00010000 is zero. The free path
//! ends with the free routine's residue in r0/r1/r3..r6 and its T; the other
//! paths end with `tst`'s or `cmp/pl`'s flag. r14 and r13 come back from the
//! frame, so they end at their entry values, and r0 always ends as the object.
//!
//! Literals are read from the image rather than hardcoded, as the original
//! does, and every register the prologue saves is restored from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c024910: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c024912: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c024914: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c024916: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c024918: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c02491a: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c02491c: bt 0x8c02493e
    if cpu.r[14] != 0 {
        // 0x8c02491e: mov.l 0x8c024948,r1
        cpu.r[1] = mem.read_u32(0x8c024948).unwrap_or(0x8c1e88a0);
        // 0x8c024920: add #16,r4
        cpu.r[4] = cpu.r[4].wrapping_add(16);
        // 0x8c024922: mov.l 0x8c02494c,r0
        cpu.r[0] = mem.read_u32(0x8c02494c).unwrap_or(0x8c018bd8);
        // 0x8c024924: mov #-1,r5
        cpu.r[5] = 0xffff_ffff;
        // 0x8c024926: jsr @r0 ; 0x8c024928: mov.l r1,@r14 (delay)
        cpu.pr = 0x8c02492a;
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c02492a: mov.l 0x8c024950,r0
        cpu.r[0] = mem.read_u32(0x8c024950).unwrap_or(0x8c05ad54);
        // 0x8c02492c: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c02492e: jsr @r0 ; 0x8c024930: mov #0,r5 (delay)
        cpu.pr = 0x8c024932;
        cpu.r[5] = 0;
        crate::registry::p_8c05ad54::run(cpu, mem);

        // 0x8c024932: exts.w r13,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[13] as u16 as i16 as i32 as u32;
        // 0x8c024934: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c024936: bf 0x8c02493e
        if cpu.t() {
            // 0x8c024938: mov.l 0x8c024954,r0
            cpu.r[0] = mem.read_u32(0x8c024954).unwrap_or(0x8c01032c);
            // 0x8c02493a: jsr @r0 ; 0x8c02493c: mov r14,r4 (delay)
            cpu.pr = 0x8c02493e;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c02493e: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c024940: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c024942: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c024944: rts ; 0x8c024946: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
