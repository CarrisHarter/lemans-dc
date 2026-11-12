//! 0x8c07964c — `free` a block when its length is positive, return the argument.
//!
//! ```text
//! mov.l r14,@-r15
//! sts.l pr,@-r15
//! mov   r4,r14
//! tst   r14,r14           ; r4 == 0?
//! bt    0x8c079662        ; yes: skip
//! exts.w r5,r0            ; r0 = (int16)r5
//! cmp/pl r0               ; r0 > 0?
//! bf    0x8c079662        ; no: skip
//! mov.l 0x8c07966c,r0     ; r0 = 0x8c01032c (thunk to free)
//! jsr   @r0               ; free(r4, r5); pr = 0x8c079660
//! nop
//! 0x8c079662:
//! mov   r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14
//! ```
//!
//! `exts.w` only widens the low half of `r5` for the test; the callee still
//! receives the raw `r5`. The call path returns the callee's whole residue,
//! including T, while each skip keeps its own last compare (`tst` -> T=1,
//! `cmp/pl` -> T=0). The epilogue never touches flags.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if !cpu.t() {
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c07966c).unwrap_or(0x8c01032c); // mov.l @(0x8c07966c,pc),r0
            cpu.pr = 0x8c079660; // jsr @r0
            crate::registry::p_8c01032c::run(cpu, mem); // delay slot: nop
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
