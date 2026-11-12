//! 0x8c0d357c — release a refcounted handle, then optionally free it outright.
//!
//! ```text
//! 8c0d357c  mov.l r14,@-r15       push r14
//! 8c0d357e  mov.l r13,@-r15       push r13
//! 8c0d3580  mov   r5,r14          r14 = count arg (short in the low half)
//! 8c0d3582  sts.l pr,@-r15        push pr
//! 8c0d3584  mov   r4,r13          r13 = handle
//! 8c0d3586  tst   r13,r13
//! 8c0d3588  bt    0x8c0d35a0      handle == NULL: return it
//! 8c0d358a  tst   r13,r13         dead (r13 is nonzero on this path)
//! 8c0d358c  bt    0x8c0d3594
//! 8c0d358e  mov.l 0x8c0d35ac,r0   r0 = 0x8c018bd8 (release)
//! 8c0d3590  jsr   @r0
//! 8c0d3592  mov   #-1,r5          delay: release with count -1
//! 8c0d3594  exts.w r14,r0         r0 = sext16(count)
//! 8c0d3596  cmp/pl r0
//! 8c0d3598  bf    0x8c0d35a0      count <= 0: keep the handle
//! 8c0d359a  mov.l 0x8c0d35b0,r0   r0 = 0x8c01032c (free thunk)
//! 8c0d359c  jsr   @r0
//! 8c0d359e  mov   r13,r4          delay: free(handle)
//! 8c0d35a0  mov   r13,r0          return the handle
//! 8c0d35a2  lds.l @r15+,pr
//! 8c0d35a4  mov.l @r15+,r13
//! 8c0d35a6  rts
//! 8c0d35a8  mov.l @r15+,r14       delay
//! ```
//!
//! Every non-NULL handle is first released at 0x8c018bd8 with a count of -1 in
//! r5, so the release's own `exts.w`/`cmp/pl` test never frees the handle. The
//! caller's count is re-tested **after** that call, on the saved r14 copy, and
//! only a *positive signed 16-bit* value triggers the final free at 0x8c01032c.
//! The delay slot at 0x8c0d3592 overwrites r5 before the callee runs; the value
//! the callee leaves in r5 is irrelevant because the free thunk only reads r4.
//!
//! r13 and r14 are pushed by the prologue and popped by the epilogue, so both
//! end at their entry values; pr likewise ends at the seed value. The last
//! flag-setting instruction on the early path is the `tst` at 0x8c0d3586
//! (T=1), and on the released path it is `cmp/pl` (T=0) or, when the final
//! free runs, whatever the free implementation leaves behind.

use sh4_oracle::{Cpu, Memory};

/// 0x8c0d35ac holds 0x8c018bd8 (the release routine).
const RELEASE_PTR: u32 = 0x8c0d35ac;
/// 0x8c0d35b0 holds 0x8c01032c (the free thunk).
const FREE_THUNK_PTR: u32 = 0x8c0d35b0;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d357c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0d357e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0d3580: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0d3582: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0d3584: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0d3586: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0d3588: bt 0x8c0d35a0
    if cpu.r[13] != 0 {
        // 0x8c0d358a/0x8c0d358c: tst r13,r13 ; bt (unreachable, r13 != 0)
        // 0x8c0d358e: mov.l 0x8c0d35ac,r0
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c0d3590: jsr @r0 ; 0x8c0d3592: mov #-1,r5 (delay slot)
        cpu.pr = 0x8c0d3594;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c0d3594: exts.w r14,r0 — only the low 16 bits of the saved count
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0d3596: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0d3598: bf 0x8c0d35a0
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0d359a: mov.l 0x8c0d35b0,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0d359c: jsr @r0 ; 0x8c0d359e: mov r13,r4 (delay slot)
            cpu.pr = 0x8c0d35a0;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0d35a0: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0d35a2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d35a4: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d35a6: rts ; 0x8c0d35a8: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
