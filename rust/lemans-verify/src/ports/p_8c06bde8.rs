//! 0x8c06bde8 — take the next 44-byte record and push it onto a list.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! mov.l 0x8c06be14,r4 (0x8c23f438) ; mov.l 0x8c06be18,r0 (0x8c06c3b8)
//! jsr @r0 ; nop
//! r3 = r0 ; tst r3,r3 ; bf have
//! bra out ; mov #0,r0
//! have: r0 = r14 ; r0 += 84 ; r1 = [r0] ; [r3+40] = r1
//!        r0 = 84 ; [r14+84] = r3 ; r0 = r3
//! out: lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! The callee (0x8c06c3b8) returns the next element of the 44-byte array at
//! 0x8c23f438, or 0 once its index reaches the count. On success the new
//! record's +0x28 link becomes the old list head and the record becomes the
//! new head at `[r14+0x54]`; otherwise nothing is written.
//!
//! T ends set only when the callee returned 0: `tst r3,r3` sets it on that
//! path and no later instruction touches flags. Both stack pushes land in the
//! synthetic stack window.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov r4,r14
    cpu.r[14] = cpu.r[4];
    // mov.l 0x8c06be14,r4 ; mov.l 0x8c06be18,r0
    cpu.r[4] = mem.read_u32(0x8c06be14).unwrap_or(0); // 0x8c23f438
    cpu.r[0] = mem.read_u32(0x8c06be18).unwrap_or(0); // 0x8c06c3b8
    // jsr @r0 ; nop (delay slot)
    cpu.pr = 0x8c06bdf6;
    crate::registry::p_8c06c3b8::run(cpu, mem);
    // mov r0,r3 ; tst r3,r3
    cpu.r[3] = cpu.r[0];
    cpu.set_t(cpu.r[3] == 0);
    if cpu.t() {
        // bra 0x8c06be0e ; mov #0,r0 (delay slot)
        cpu.r[0] = 0;
    } else {
        // mov r14,r0 ; add #84,r0 ; mov.l @r0,r1
        cpu.r[0] = cpu.r[14].wrapping_add(84);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // mov.l r1,@(40,r3)
        let _ = mem.write_u32(cpu.r[3].wrapping_add(40), cpu.r[1]);
        // mov #84,r0 ; mov.l r3,@(r0,r14)
        cpu.r[0] = 84;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3]);
        // mov r3,r0
        cpu.r[0] = cpu.r[3];
    }
    // lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay slot)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
