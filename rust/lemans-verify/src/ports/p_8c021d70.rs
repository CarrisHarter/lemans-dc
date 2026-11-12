//! 0x8c021d70 — release the three handles at `+8`, `+4`, `+0` of a container
//! and, for a positive count, free the container itself.
//!
//! ```text
//! 8c021d70  mov.l r14,@-r15     push r14
//! 8c021d72  mov.l r13,@-r15     push r13
//! 8c021d74  mov   r4,r14        r14 = container
//! 8c021d76  sts.l pr,@-r15      push pr
//! 8c021d78  mov   r5,r13        r13 = count arg
//! 8c021d7a  tst   r14,r14
//! 8c021d7c  bt    0x8c021da8    container == NULL: return it
//! 8c021d7e  tst   r14,r14       dead (r14 is nonzero here)
//! 8c021d80  bt    0x8c021d9c
//! 8c021d82  mov.l 0x8c021db4,r0 r0 = 0x8c018bd8 (release handle)
//! 8c021d84  add   #8,r4
//! 8c021d86  jsr   @r0
//! 8c021d88  mov   #-1,r5        delay: release(container+8, -1)
//! 8c021d8a  mov.l 0x8c021db4,r0
//! 8c021d8c  mov   r14,r4
//! 8c021d8e  add   #4,r4
//! 8c021d90  jsr   @r0
//! 8c021d92  mov   #-1,r5        delay: release(container+4, -1)
//! 8c021d94  mov.l 0x8c021db4,r0
//! 8c021d96  mov   r14,r4
//! 8c021d98  jsr   @r0
//! 8c021d9a  mov   #-1,r5        delay: release(container+0, -1)
//! 8c021d9c  exts.w r13,r0       r0 = sext16(count arg)
//! 8c021d9e  cmp/pl r0
//! 8c021da0  bf    0x8c021da8    count <= 0: keep the container
//! 8c021da2  mov.l 0x8c021db8,r0 r0 = 0x8c01032c (free thunk)
//! 8c021da4  jsr   @r0
//! 8c021da6  mov   r14,r4        delay: free(container)
//! 8c021da8  mov   r14,r0        return the container
//! 8c021daa  lds.l @r15+,pr
//! 8c021dac  mov.l @r15+,r13
//! 8c021dae  rts
//! 8c021db0  mov.l @r15+,r14     delay
//! ```
//!
//! Every `jsr` sets pr and its delay slot only writes r5 = -1, so each release
//! sees the same count. The second `tst`/`bt` is unreachable: at 0x8c021d7e
//! r14 is already known to be nonzero. `exts.w` makes only the low 16 bits of
//! the count matter, signed: r5 = 0x00010000 is zero and does not free the
//! container, while r5 = 0xffff0001 is positive and does.
//!
//! Only r0..r7 (and T) carry the last callee's residue: r13 gets the raw r5
//! and r14 the container, and both are restored from the frame in the
//! epilogue, together with pr. r0 is the container on every path, including
//! the NULL early exit.

use sh4_oracle::{Cpu, Memory};

/// 0x8c021db4 holds 0x8c018bd8, 0x8c021db8 holds 0x8c01032c.
const RELEASE_LITERAL: u32 = 0x8c021db4;
const FREE_THUNK_LITERAL: u32 = 0x8c021db8;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

/// `mov.l RELEASE_LITERAL,r0 ; add/adjust r4 ; jsr @r0 ; mov #-1,r5`.
#[inline]
fn release(cpu: &mut Cpu, mem: &mut Memory, handle: u32, return_addr: u32) {
    cpu.r[0] = rd(mem, RELEASE_LITERAL, 0x8c018bd8);
    cpu.r[4] = handle;
    cpu.pr = return_addr;
    cpu.r[5] = 0xffff_ffff;
    crate::registry::p_8c018bd8::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c021d70: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c021d72: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c021d74: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c021d76: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c021d78: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c021d7a: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c021d7c: bt 0x8c021da8
    if !cpu.t() {
        // 0x8c021d7e/0x8c021d80: tst r14,r14 ; bt (unreachable, r14 != 0)
        // 0x8c021d82..0x8c021d88: release(container + 8, -1)
        release(cpu, mem, cpu.r[14].wrapping_add(8), 0x8c021d8a);
        // 0x8c021d8a..0x8c021d92: release(container + 4, -1)
        release(cpu, mem, cpu.r[14].wrapping_add(4), 0x8c021d94);
        // 0x8c021d94..0x8c021d9a: release(container, -1)
        release(cpu, mem, cpu.r[14], 0x8c021d9c);
        // 0x8c021d9c: exts.w r13,r0
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32;
        // 0x8c021d9e: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c021da0: bf 0x8c021da8
        if (cpu.r[0] as i32) > 0 {
            // 0x8c021da2: mov.l 0x8c021db8,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c021da4: jsr @r0 ; 0x8c021da6: mov r14,r4 (delay)
            cpu.pr = 0x8c021da8;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c021da8: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c021daa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c021dac: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c021dae: rts ; 0x8c021db0: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
