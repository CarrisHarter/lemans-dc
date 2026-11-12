//! 0x8c1862b4 — free `r4` when the caller says the slot is live.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15
//! mov   r4,r14
//! tst   r14,r14   ; bt  skip
//! exts.w r5,r0
//! cmp/pl r0       ; bf  skip
//! mov.l 0x8c1862d4,r0   ; r0 = 0x8c01032c (thunk to free)
//! jsr   @r0
//! nop             ; delay slot
//! skip: mov r14,r0
//! lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! The pointer survives in `r14`; the callee's return value is discarded and
//! the original hands back the pointer in `r0`. Both no-call exits leave the
//! flag from their own test: `tst` (T set) on the null path, `cmp/pl` (T clear)
//! on the non-positive-count path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let null = cpu.r[14] == 0;
    cpu.set_t(null);
    if !null {
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c1862d4).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c1862ca; // jsr @r0; delay slot: nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
