//! 0x8c0ff5b8 — free a block when requested, and return the block pointer.
//!
//! ```text
//! 8c0ff5b8: mov.l r14,@-r15
//! 8c0ff5ba: sts.l pr,@-r15
//! 8c0ff5bc: mov   r4,r14
//! 8c0ff5be: tst   r14,r14
//! 8c0ff5c0: bt    0x8c0ff5ce        ; r4 == 0: no free
//! 8c0ff5c2: exts.w r5,r0
//! 8c0ff5c4: cmp/pl r0
//! 8c0ff5c6: bf    0x8c0ff5ce        ; (short)r5 <= 0: no free
//! 8c0ff5c8: mov.l 0x8c0ff5d8,r0     ; 0x8c01032c, the free thunk
//! 8c0ff5ca: jsr   @r0
//! 8c0ff5cc: nop                     ; delay slot
//! 8c0ff5ce: mov   r14,r0
//! 8c0ff5d0: lds.l @r15+,pr
//! 8c0ff5d2: rts
//! 8c0ff5d4: mov.l @r15+,r14         ; delay slot
//! ```
//!
//! Returns param_1 in r0 on every path. It calls the free thunk (0x8c01032c)
//! with r4 = param_1 and r5 untouched only when param_1 != 0 and the
//! sign-extended low half of param_2 is strictly positive; the callee's flag
//! residue is then the function's T on that path. On the two skip paths T is
//! what `tst` (r4 == 0) or `cmp/pl` (low half <= 0) left. r14 and pr are
//! restored from the eight-byte frame, so both end at their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0ff5b8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0ff5ba: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0ff5bc: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0ff5be: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c0ff5c0: bt 0x8c0ff5ce
    if !cpu.t() {
        // 0x8c0ff5c2: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c0ff5c4: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0ff5c6: bf 0x8c0ff5ce
        if cpu.t() {
            // 0x8c0ff5c8: mov.l 0x8c0ff5d8,r0
            cpu.r[0] = mem.read_u32(0x8c0ff5d8).unwrap_or(0); // 0x8c01032c
            // 0x8c0ff5ca: jsr @r0 ; 0x8c0ff5cc: nop (delay slot)
            cpu.pr = 0x8c0ff5ce;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0ff5ce: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0ff5d0: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0ff5d2: rts ; 0x8c0ff5d4: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
