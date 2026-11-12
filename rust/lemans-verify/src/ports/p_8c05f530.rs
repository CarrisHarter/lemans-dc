//! 0x8c05f530 — destroy a reference-counted object with two handle fields.
//!
//! ```text
//! 8c05f530  mov.l r14,@-r15        push r14
//! 8c05f532  mov.l r13,@-r15        push r13
//! 8c05f534  mov   r5,r14           r14 = second argument (short)
//! 8c05f536  sts.l pr,@-r15         push pr
//! 8c05f538  mov   r4,r13           r13 = object
//! 8c05f53a  tst   r13,r13
//! 8c05f53c  bt    0x8c05f570       object == NULL: return it
//! 8c05f53e  mov.l 0x8c05f57c,r1    r1 = 0x8c1e9174
//! 8c05f540  add   #24,r4           r4 = object + 24
//! 8c05f542  mov.l 0x8c05f580,r0    r0 = 0x8c018bd8 (release handle)
//! 8c05f544  mov   #-1,r5           count = -1
//! 8c05f546  jsr   @r0
//! 8c05f548  mov.l r1,@r13          delay: *object = 0x8c1e9174
//! 8c05f54a  mov.l 0x8c05f580,r0    r0 = 0x8c018bd8
//! 8c05f54c  mov   r13,r4
//! 8c05f54e  add   #20,r4           r4 = object + 20
//! 8c05f550  jsr   @r0
//! 8c05f552  mov   #-1,r5           delay: count = -1
//! 8c05f554  tst   r13,r13
//! 8c05f556  bt    0x8c05f564       never taken, object is non-NULL
//! 8c05f558  mov.l 0x8c05f584,r0    r0 = 0x8c1e9198
//! 8c05f55a  mov   r13,r4
//! 8c05f55c  mov.l r0,@r13          *object = 0x8c1e9198
//! 8c05f55e  mov.l 0x8c05f588,r0    r0 = 0x8c056bcc (clear + free)
//! 8c05f560  jsr   @r0
//! 8c05f562  mov   #0,r5            delay: count = 0
//! 8c05f564  exts.w r14,r0          r0 = sext16(second argument)
//! 8c05f566  cmp/pl r0
//! 8c05f568  bf    0x8c05f570       low half <= 0: do not free the object
//! 8c05f56a  mov.l 0x8c05f58c,r0    r0 = 0x8c01032c (free thunk)
//! 8c05f56c  jsr   @r0
//! 8c05f56e  mov   r13,r4           delay: free(object)
//! 8c05f570  mov   r13,r0           return the object
//! 8c05f572  lds.l @r15+,pr
//! 8c05f574  mov.l @r15+,r13
//! 8c05f576  rts
//! 8c05f578  mov.l @r15+,r14       delay
//! ```
//!
//! The object layout the routine drives: `*object` is a vtable word, handles at
//! +20 and +24. Each release decrements the pointed-to object's refcount at
//! +8; with the -1 count the handle is cleared when another reference remains
//! and the pointee is freed when the count reaches zero. The two releases run
//! before `*object` is repointed at 0x8c1e9198, and 0x8c056bcc then stores its
//! own sentinel (0x8c1e8f88) over that word because its delay slot executes on
//! both branch outcomes. The final `free(object)` is gated only by the *low
//! half* of the second argument (`exts.w`), so 0x00010000 does not free and
//! 0xffff0001 does; the upper bits never matter.
//!
//! Every early exit returns `r13` in `r0`, and the epilogue restores `pr`,
//! `r13` and `r14` from the frame, so those end at their seed values. On the
//! NULL path the entry `tst` leaves T=1; on a non-NULL no-free path the final
//! `cmp/pl` leaves T=0; on the free path T is whatever the free routine leaves.
//! Literal pointers are read from the image, as the original does.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05f530: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05f532: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05f534: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05f536: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05f538: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05f53a: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05f53c: bt 0x8c05f570
    if cpu.r[13] != 0 {
        // 0x8c05f53e: mov.l 0x8c05f57c,r1
        cpu.r[1] = mem.read_u32(0x8c05f57c).unwrap_or(0x8c1e9174);
        // 0x8c05f540: add #24,r4
        cpu.r[4] = cpu.r[4].wrapping_add(24);
        // 0x8c05f542: mov.l 0x8c05f580,r0
        cpu.r[0] = mem.read_u32(0x8c05f580).unwrap_or(0x8c018bd8);
        // 0x8c05f544: mov #-1,r5
        cpu.r[5] = 0xffff_ffff;
        // 0x8c05f546: jsr @r0 ; 0x8c05f548: mov.l r1,@r13 (delay)
        cpu.pr = 0x8c05f548;
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c05f54a: mov.l 0x8c05f580,r0
        cpu.r[0] = mem.read_u32(0x8c05f580).unwrap_or(0x8c018bd8);
        // 0x8c05f54c: mov r13,r4
        cpu.r[4] = cpu.r[13];
        // 0x8c05f54e: add #20,r4
        cpu.r[4] = cpu.r[4].wrapping_add(20);
        // 0x8c05f550: jsr @r0 ; 0x8c05f552: mov #-1,r5 (delay)
        cpu.pr = 0x8c05f552;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c05f554: tst r13,r13
        cpu.set_t(cpu.r[13] == 0);
        // 0x8c05f556: bt 0x8c05f564
        if cpu.r[13] != 0 {
            // 0x8c05f558: mov.l 0x8c05f584,r0
            cpu.r[0] = mem.read_u32(0x8c05f584).unwrap_or(0x8c1e9198);
            // 0x8c05f55a: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c05f55c: mov.l r0,@r13
            let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
            // 0x8c05f55e: mov.l 0x8c05f588,r0
            cpu.r[0] = mem.read_u32(0x8c05f588).unwrap_or(0x8c056bcc);
            // 0x8c05f560: jsr @r0 ; 0x8c05f562: mov #0,r5 (delay)
            cpu.pr = 0x8c05f562;
            cpu.r[5] = 0;
            crate::registry::p_8c056bcc::run(cpu, mem);
        }
        // 0x8c05f564: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as u16 as i16 as i32 as u32;
        // 0x8c05f566: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05f568: bf 0x8c05f570
        if (cpu.r[0] as i32) > 0 {
            // 0x8c05f56a: mov.l 0x8c05f58c,r0
            cpu.r[0] = mem.read_u32(0x8c05f58c).unwrap_or(0x8c01032c);
            // 0x8c05f56c: jsr @r0 ; 0x8c05f56e: mov r13,r4 (delay)
            cpu.pr = 0x8c05f56e;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05f570: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05f572: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05f574: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05f576: rts ; 0x8c05f578: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
