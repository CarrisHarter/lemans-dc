//! 0x8c0bfecc — free a block when the second argument is positive.
//!
//! ```text
//! mov.l r14,@-r15        ; save r14 (caller's value)
//! sts.l pr,@-r15         ; save pr
//! mov   r4,r14
//! tst   r14,r14
//! bt    0x8c0bfee2       ; null block: nothing to free
//! exts.w r5,r0           ; only the low 16 bits of the second argument count
//! cmp/pl r0
//! bf    0x8c0bfee2       ; non-positive: nothing to free
//! mov.l 0x8c0bfeec,r0    ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0              ; pr = 0x8c0bfee2; delay slot is a nop
//! nop
//! 0x8c0bfee2:
//! mov   r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14        ; delay slot: restore the caller's r14
//! ```
//!
//! `free` (0x8c159fc0) uses only r0–r6, so the body's r14 still holds the
//! block pointer when the epilogue copies it to r0; the epilogue then restores
//! r14 and pr to their entry values. T ends as the last flag-setting
//! instruction left it: the `tst` on the null path, the `cmp/pl` on the
//! non-positive path, and `free`'s own final compare on the call path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[15].wrapping_sub(4), cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15].wrapping_sub(4), cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[14] = cpu.r[4];
    if cpu.r[14] == 0 {
        cpu.set_t(true); // tst r14,r14 ; bt
    } else {
        let arg = cpu.r[5] as u16 as i16 as i32; // exts.w r5,r0
        cpu.set_t(arg > 0); // cmp/pl r0
        if arg > 0 {
            cpu.r[0] = mem.read_u32(0x8c0bfeec).unwrap_or(0);
            cpu.pr = 0x8c0bfee2; // jsr @r0 (delay slot: nop)
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
