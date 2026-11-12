//! 0x8c01a710 — release three embedded handles, then free the owning object.
//!
//! ```text
//! 8c01a710  mov.l r14,@-r15        push r14
//! 8c01a712  mov.l r13,@-r15        push r13
//! 8c01a714  mov   r5,r14           r14 = count arg (short in low half)
//! 8c01a716  sts.l pr,@-r15         push pr
//! 8c01a718  mov   r4,r13           r13 = object
//! 8c01a71a  tst   r13,r13
//! 8c01a71c  bt    0x8c01a758       NULL object: return it
//! 8c01a71e  add   #12,r4           r4 = object + 12
//! 8c01a720  tst   r4,r4
//! 8c01a722  bt    0x8c01a730       object == -12: skip the span free
//! 8c01a724  mov.l 0x8c01a764,r0    r0 = 0x8c01efb8 (wait and free +20)
//! 8c01a726  jsr   @r0
//! 8c01a728  nop                    delay
//! 8c01a72a  mov   r13,r0
//! 8c01a72c  add   #12,r0
//! 8c01a72e  tst   r0,r0            dead: r13 != 0 here
//! 8c01a730  mov.l 0x8c01a768,r0    r0 = 0x8c018bd8 (release)
//! 8c01a732  mov   r13,r4
//! 8c01a734  add   #8,r4
//! 8c01a736  jsr   @r0
//! 8c01a738  mov   #-1,r5           delay: release(&object->h2, -1)
//! 8c01a73a  mov.l 0x8c01a768,r0
//! 8c01a73c  mov   r13,r4
//! 8c01a73e  add   #4,r4
//! 8c01a740  jsr   @r0
//! 8c01a742  mov   #-1,r5           delay: release(&object->h1, -1)
//! 8c01a744  mov.l 0x8c01a768,r0
//! 8c01a746  mov   r13,r4
//! 8c01a748  jsr   @r0
//! 8c01a74a  mov   #-1,r5           delay: release(&object->h0, -1)
//! 8c01a74c  exts.w r14,r0          only the low half of the count matters
//! 8c01a74e  cmp/pl r0
//! 8c01a750  bf    0x8c01a758       count <= 0: keep the object
//! 8c01a752  mov.l 0x8c01a76c,r0    r0 = 0x8c01032c (free thunk)
//! 8c01a754  jsr   @r0
//! 8c01a756  mov   r13,r4           delay: free(object)
//! 8c01a758  mov   r13,r0           return the object
//! 8c01a75a  lds.l @r15+,pr
//! 8c01a75c  mov.l @r15+,r13
//! 8c01a75e  rts
//! 8c01a760  mov.l @r15+,r14       delay
//! ```
//!
//! `r4 == 0` short-circuits everything, including the three releases, and
//! leaves T set by the entry `tst`. A non-NULL object first hands `object+12`
//! to 0x8c01efb8 (which spins over a size-derived span and frees `[object+20]`
//! or returns at once when that field is NULL), then releases the handles at
//! `+8`, `+4` and `+0` with count -1. The release call is also where the later
//! register residue comes from: the final `exts.w`/`cmp/pl` only overwrite r0
//! and T. When the low 16 bits of the count are positive the object itself is
//! freed through the 0x8c01032c thunk, so the free routine's residue and flags
//! reach the epilogue unchanged.
//!
//! Every `jsr` here writes pr with its return address; the callees save and
//! restore it, and the epilogue reloads the seeded pr from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01a710: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c01a712: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c01a714: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c01a716: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c01a718: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c01a71a: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c01a71c: bt 0x8c01a758
    if cpu.r[13] != 0 {
        // 0x8c01a71e: add #12,r4
        cpu.r[4] = cpu.r[13].wrapping_add(12);
        // 0x8c01a720: tst r4,r4
        cpu.set_t(cpu.r[4] == 0);
        // 0x8c01a722: bt 0x8c01a730
        if cpu.r[4] != 0 {
            // 0x8c01a724: mov.l 0x8c01a764,r0
            cpu.r[0] = mem.read_u32(0x8c01a764).unwrap_or(0x8c01efb8);
            // 0x8c01a726: jsr @r0 ; 0x8c01a728 nop (delay)
            cpu.pr = 0x8c01a72a;
            crate::registry::p_8c01efb8::run(cpu, mem);
            // 0x8c01a72a: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c01a72c: add #12,r0
            cpu.r[0] = cpu.r[0].wrapping_add(12);
            // 0x8c01a72e: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
        }

        // 0x8c01a730: mov.l 0x8c01a768,r0
        cpu.r[0] = mem.read_u32(0x8c01a768).unwrap_or(0x8c018bd8);
        // 0x8c01a732: mov r13,r4 ; 0x8c01a734: add #8,r4
        cpu.r[4] = cpu.r[13].wrapping_add(8);
        // 0x8c01a736: jsr @r0 ; 0x8c01a738: mov #-1,r5 (delay)
        cpu.pr = 0x8c01a73a;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c01a73a: mov.l 0x8c01a768,r0
        cpu.r[0] = mem.read_u32(0x8c01a768).unwrap_or(0x8c018bd8);
        // 0x8c01a73c: mov r13,r4 ; 0x8c01a73e: add #4,r4
        cpu.r[4] = cpu.r[13].wrapping_add(4);
        // 0x8c01a740: jsr @r0 ; 0x8c01a742: mov #-1,r5 (delay)
        cpu.pr = 0x8c01a744;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c01a744: mov.l 0x8c01a768,r0
        cpu.r[0] = mem.read_u32(0x8c01a768).unwrap_or(0x8c018bd8);
        // 0x8c01a746: mov r13,r4
        cpu.r[4] = cpu.r[13];
        // 0x8c01a748: jsr @r0 ; 0x8c01a74a: mov #-1,r5 (delay)
        cpu.pr = 0x8c01a74c;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c01a74c: exts.w r14,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c01a74e: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c01a750: bf 0x8c01a758
        if (cpu.r[0] as i32) > 0 {
            // 0x8c01a752: mov.l 0x8c01a76c,r0
            cpu.r[0] = mem.read_u32(0x8c01a76c).unwrap_or(0x8c01032c);
            // 0x8c01a754: jsr @r0 ; 0x8c01a756: mov r13,r4 (delay)
            cpu.pr = 0x8c01a758;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c01a758: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c01a75a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c01a75c: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c01a75e: rts ; 0x8c01a760: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
