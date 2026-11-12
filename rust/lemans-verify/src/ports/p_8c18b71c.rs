//! 0x8c18b71c — clear the current-object global and destroy a refcounted
//! handle.
//!
//! ```text
//! 8c18b71c  mov.l r14,@-r15        push r14
//! 8c18b71e  mov.l r13,@-r15        push r13
//! 8c18b720  mov   r4,r14           r14 = handle
//! 8c18b722  sts.l pr,@-r15         push pr
//! 8c18b724  mov   r5,r13           r13 = count
//! 8c18b726  tst   r14,r14
//! 8c18b728  bt    0x8c18b742       handle == NULL: return it
//! 8c18b72a  mov.l 0x8c18b74c,r3    r3 = 0x8c2b7578
//! 8c18b72c  mov   #0,r1
//! 8c18b72e  mov.l 0x8c18b750,r0    r0 = 0x8c018bd8 (release)
//! 8c18b730  mov   #-1,r5           count -1: release one reference
//! 8c18b732  jsr   @r0
//! 8c18b734  mov.l r1,@r3           delay: *(u32*)0x8c2b7578 = 0
//! 8c18b736  exts.w r13,r0          r0 = sext16(count)
//! 8c18b738  cmp/pl r0
//! 8c18b73a  bf    0x8c18b742       count <= 0: keep the handle
//! 8c18b73c  mov.l 0x8c18b754,r0    r0 = 0x8c01032c (free thunk)
//! 8c18b73e  jsr   @r0
//! 8c18b740  mov   r14,r4           delay: free(handle)
//! 8c18b742  mov   r14,r0           return the handle (0 on the early exit)
//! 8c18b744  lds.l @r15+,pr
//! 8c18b746  mov.l @r15+,r13
//! 8c18b748  rts
//! 8c18b74a  mov.l @r15+,r14       delay
//! ```
//!
//! The decompiled C calls the release helper with only `0xffffffff`; the
//! disassembly shows `r4` still holds the handle, so the call is
//! `release(handle, -1)`. The handle is the first argument of both the release
//! helper and the free thunk, and `exts.w` makes a count such as `0x00010000`
//! count as zero while `0xffff0001` counts as positive.
//!
//! Only the literal pool loads are modelled from memory; the stores are the
//! global clear at 0x8c2b7578 and whatever the callees do. Both callees save
//! and restore `pr`, so the epilogue returns it to the seed value.

use sh4_oracle::{Cpu, Memory};

/// 0x8c18b74c holds 0x8c2b7578, 0x8c18b750 holds 0x8c018bd8 and 0x8c18b754
/// holds 0x8c01032c.
const OBJECT_GLOBAL_PTR: u32 = 0x8c18b74c;
const RELEASE_PTR: u32 = 0x8c18b750;
const FREE_PTR: u32 = 0x8c18b754;
const OBJECT_GLOBAL: u32 = 0x8c2b7578;
const RELEASE: u32 = 0x8c018bd8;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c18b71c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c18b71e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c18b720: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c18b722: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c18b724: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c18b726: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c18b728: bt 0x8c18b742
    if cpu.r[14] != 0 {
        // 0x8c18b72a: mov.l 0x8c18b74c,r3 (0x8c2b7578)
        cpu.r[3] = mem.read_u32(OBJECT_GLOBAL_PTR).unwrap_or(OBJECT_GLOBAL);
        // 0x8c18b72c: mov #0,r1
        cpu.r[1] = 0;
        // 0x8c18b72e: mov.l 0x8c18b750,r0 (0x8c018bd8)
        cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
        // 0x8c18b730: mov #-1,r5
        cpu.r[5] = 0xFFFF_FFFF;
        // 0x8c18b732: jsr @r0 ; 0x8c18b734: mov.l r1,@r3 (delay)
        cpu.pr = 0x8C18B736;
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c18b736: exts.w r13,r0
        cpu.r[0] = cpu.r[13] as u16 as i16 as i32 as u32;
        // 0x8c18b738: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c18b73a: bf 0x8c18b742
        if (cpu.r[0] as i32) > 0 {
            // 0x8c18b73c: mov.l 0x8c18b754,r0 (0x8c01032c)
            cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(FREE_THUNK);
            // 0x8c18b73e: jsr @r0 ; 0x8c18b740: mov r14,r4 (delay)
            cpu.pr = 0x8C18B742;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c18b742: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c18b744: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18b746: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18b748: rts ; 0x8c18b74a: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
