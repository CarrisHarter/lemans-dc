//! 0x8c0fee44 — release a handle and, for a positive count, free it too.
//!
//! ```text
//! 8c0fee44  mov.l r14,@-r15     push r14
//! 8c0fee46  mov.l r13,@-r15     push r13
//! 8c0fee48  mov   r5,r14        r14 = count arg
//! 8c0fee4a  sts.l pr,@-r15      push pr
//! 8c0fee4c  mov   r4,r13        r13 = handle
//! 8c0fee4e  tst   r13,r13
//! 8c0fee50  bt    0x8c0fee64    handle == NULL: return it
//! 8c0fee52  mov.l 0x8c0fee70,r0 r0 = 0x8c018bd8 (release handle)
//! 8c0fee54  jsr   @r0
//! 8c0fee56  mov   #-1,r5        delay: release(handle, -1)
//! 8c0fee58  exts.w r14,r0       r0 = sext16(count arg)
//! 8c0fee5a  cmp/pl r0
//! 8c0fee5c  bf    0x8c0fee64    count <= 0: keep the handle
//! 8c0fee5e  mov.l 0x8c0fee74,r0 r0 = 0x8c01032c (free thunk)
//! 8c0fee60  jsr   @r0
//! 8c0fee62  mov   r13,r4        delay: free(handle)
//! 8c0fee64  mov   r13,r0        return the handle
//! 8c0fee66  lds.l @r15+,pr
//! 8c0fee68  mov.l @r15+,r13
//! 8c0fee6a  rts
//! 8c0fee6c  mov.l @r15+,r14     delay
//! ```
//!
//! The first `jsr` always passes r5 = -1, so the release decrements the
//! handle's object refcount and never frees the handle itself; the following
//! `exts.w` makes only the low 16 bits of the caller's r5 decide whether this
//! function then frees the handle's block. `exts.w` and the epilogue do not
//! touch T, so the surviving T is the release's on the early-out branch, 0
//! from `cmp/pl` when the count is non-positive, or the free's when it runs.
//!
//! r13 and r14 hold the raw entry r4/r5 and are restored from the frame;
//! pr is likewise restored, and r0 is the handle on every path.

use sh4_oracle::{Cpu, Memory};

/// 0x8c0fee70 holds 0x8c018bd8, 0x8c0fee74 holds 0x8c01032c.
const RELEASE_LITERAL: u32 = 0x8c0fee70;
const FREE_THUNK_LITERAL: u32 = 0x8c0fee74;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0fee44: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0fee46: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0fee48: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0fee4a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0fee4c: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0fee4e: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0fee50: bt 0x8c0fee64
    if cpu.r[13] != 0 {
        // 0x8c0fee52: mov.l 0x8c0fee70,r0
        cpu.r[0] = rd(mem, RELEASE_LITERAL, 0x8c018bd8);
        // 0x8c0fee54: jsr @r0 ; 0x8c0fee56: mov #-1,r5 (delay)
        cpu.r[4] = cpu.r[13];
        cpu.pr = 0x8c0fee58;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c0fee58: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0fee5a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0fee5c: bf 0x8c0fee64
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0fee5e: mov.l 0x8c0fee74,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c0fee60: jsr @r0 ; 0x8c0fee62: mov r13,r4 (delay)
            cpu.pr = 0x8c0fee64;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0fee64: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0fee66: lds.l @r15+,pr
    cpu.pr = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fee68: mov.l @r15+,r13
    cpu.r[13] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fee6a: rts ; 0x8c0fee6c: mov.l @r15+,r14 (delay)
    cpu.r[14] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
