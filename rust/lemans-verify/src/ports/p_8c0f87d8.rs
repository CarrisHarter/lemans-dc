//! 0x8c0f87d8 — call one of two field getters on `r4 + 0x2c` based on `r5`'s low byte.
//!
//! ```text
//! sts.l pr,@-r15          ; save pr
//! extu.b r5,r0
//! tst   r0,r0
//! bt    0x8c0f87ea        ; low byte zero -> 0x8c14fe68
//! mov.l 0x8c0f87f8,r0     ; 0x8c14fe34
//! jsr   @r0               ; add #44,r4 in the delay slot
//! bra   0x8c0f87f0        ; nop in the delay slot
//! mov.l 0x8c0f87fc,r0     ; 0x8c14fe68
//! jsr   @r0               ; add #44,r4 in the delay slot
//! lds.l @r15+,pr
//! rts
//! nop
//! ```
//!
//! The callee argument is `r4 + 44` on both paths; the delay slot runs before
//! the callee body. Both callees are leaves that touch only `r0`, so `pr` and
//! `r15` end at their entry values and T keeps the value `tst` gave it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = cpu.r[5] & 0xff; // extu.b
    let zero = cpu.r[0] == 0; // tst r0,r0
    cpu.set_t(zero);
    cpu.r[4] = cpu.r[4].wrapping_add(44); // delay slot of `jsr`, both paths
    if zero {
        cpu.r[0] = mem.read_u32(0x8c0f87fc).unwrap_or(0x8c14fe68);
        cpu.pr = 0x8c0f87f0; // return address after `jsr` at 0x8c0f87ec
        crate::registry::p_8c14fe68::run(cpu, mem);
    } else {
        cpu.r[0] = mem.read_u32(0x8c0f87f8).unwrap_or(0x8c14fe34);
        cpu.pr = 0x8c0f87e6; // return address after `jsr` at 0x8c0f87e2
        crate::registry::p_8c14fe34::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts delay slot is `nop`
}
