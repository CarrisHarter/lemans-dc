//! 0x8c1b5c7c — release a counted handle.
//!
//! ```text
//! 8c1b5c7c  mov.l r14,@-r15        push r14
//! 8c1b5c7e  mov.l r13,@-r15        push r13
//! 8c1b5c80  mov   r5,r14           r14 = count arg (low half is the flag)
//! 8c1b5c82  sts.l pr,@-r15         push pr
//! 8c1b5c84  mov   r4,r13           r13 = handle
//! 8c1b5c86  tst   r13,r13
//! 8c1b5c88  bt    0x8c1b5c9c       NULL: return it
//! 8c1b5c8a  mov.l 0x8c1b5ca8,r0    r0 = 0x8c018bd8 (release a reference)
//! 8c1b5c8c  jsr   @r0
//! 8c1b5c8e  mov   #-1,r5           delay: release with count -1
//! 8c1b5c90  exts.w r14,r0          r0 = sext16(count arg)
//! 8c1b5c92  cmp/pl r0
//! 8c1b5c94  bf    0x8c1b5c9c       count <= 0: keep the handle
//! 8c1b5c96  mov.l 0x8c1b5cac,r0    r0 = 0x8c01032c (free thunk)
//! 8c1b5c98  jsr   @r0
//! 8c1b5c9a  mov   r13,r4           delay: free(handle)
//! 8c1b5c9c  mov   r13,r0           return the handle
//! 8c1b5c9e  lds.l @r15+,pr
//! 8c1b5ca0  mov.l @r15+,r13
//! 8c1b5ca2  rts
//! 8c1b5ca4  mov.l @r15+,r14        delay
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // mov r5,r14
    cpu.r[14] = cpu.r[5];
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov r4,r13
    cpu.r[13] = cpu.r[4];
    // tst r13,r13
    let null = cpu.r[13] == 0;
    cpu.set_t(null);
    if !null {
        // mov.l 0x8c1b5ca8,r0 ; jsr @r0
        cpu.r[0] = mem.read_u32(0x8c1b5ca8).unwrap_or(0x8c018bd8);
        cpu.r[5] = 0xffff_ffff; // delay slot: mov #-1,r5
        crate::registry::p_8c018bd8::run(cpu, mem);
        // exts.w r14,r0
        cpu.r[0] = (cpu.r[14] as i16) as i32 as u32;
        // cmp/pl r0
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        if positive {
            // mov.l 0x8c1b5cac,r0 ; jsr @r0
            cpu.r[0] = mem.read_u32(0x8c1b5cac).unwrap_or(0x8c01032c);
            cpu.r[4] = cpu.r[13]; // delay slot: mov r13,r4
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // mov r13,r0
    cpu.r[0] = cpu.r[13];
    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
