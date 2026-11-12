//! 0x8c018e38 — free a block when a positive count asks for it.
//!
//! Saves `r14`/`pr`, then:
//!
//! ```text
//! if (r4 != 0) {
//!     *0x8c23da90 = 0;          // delay slot of `bf.s`, always runs
//!     if ((int16)r5 > 0) free(r4);   // via the 0x8c01032c thunk
//! }
//! return r4;
//! ```
//!
//! The zeroing is the delay slot of the `bf.s` at 0x8c018e4a, so it happens on
//! both outcomes of the comparison, but not on the `r4 == 0` early exit (a plain
//! `bt` skips it). The `jsr` delay slot is a `nop`; the callee's register
//! residue survives except `r0`, which is overwritten with the return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // mov r4,r14 ; tst r14,r14 ; bt 0x8c018e54
    cpu.r[14] = cpu.r[4];
    let zero = cpu.r[14] == 0;
    cpu.set_t(zero);
    if !zero {
        // mov.l 0x8c018e5c,r3 ; mov #0,r1
        cpu.r[3] = mem.read_u32(0x8c018e5c).unwrap_or(0);
        cpu.r[1] = 0;
        // exts.w r5,r0 ; cmp/pl r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        // bf.s 0x8c018e54 delay slot: mov.l r1,@r3
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        if positive {
            // mov.l 0x8c018e60,r0 ; jsr @r0 ; nop
            cpu.r[0] = mem.read_u32(0x8c018e60).unwrap_or(0);
            cpu.pr = 0x8c018e54;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c018e54: mov r14,r0 ; lds.l @r15+,pr ; rts ; delay: mov.l @r15+,r14
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
