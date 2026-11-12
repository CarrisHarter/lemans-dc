//! 0x8c23a800 — validate a pointer against a fixed-stride table, then attach it.
//!
//! ```text
//! sts.l pr,@-r15 ; add #-8,r15
//! mov.l 0x8c23a840,r3 (0x8c237780) ; mov.l r4,@(4,r15) ; mov.l r5,@r15
//! jsr @r3 ; mov.l @(4,r15),r4 (delay)
//! tst r0,r0 ; bt out
//! mov.l @r15,r3 ; tst r3,r3 ; bf store
//! out:   mov.l 0x8c23a844,r0 (0x0000eeee) ; add #8,r15 ; lds.l @r15+,pr ; rts
//! store: mov.l @r15,r3 ; mov #0,r0 ; mov.l @(4,r15),r4
//!        mov.l r3,@(20,r4) ; add #8,r15 ; lds.l @r15+,pr ; rts
//! ```
//!
//! `r4 = param_1`, `r5 = param_2`. The callee (0x8c237780) linear-searches a
//! fixed-stride table for `param_1` and returns 1 on a hit, 0 otherwise. When
//! the search misses, or `param_2 == 0`, the function returns the literal
//! 0x0000eeee without storing anything; otherwise it writes `param_2` to
//! `[param_1+0x14]` and returns 0.
//!
//! T ends set on both sentinel exits (the `tst` that selected them is the last
//! flag-setting instruction) and clear on the storing exit (`tst r3,r3` before
//! `bf`). The callee's own flags are overwritten by `tst r0,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c23a800: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c23a802: add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // 0x8c23a804: mov.l 0x8c23a840,r3
    cpu.r[3] = mem.read_u32(0x8c23a840).unwrap_or(0x8c237780);
    // 0x8c23a806: mov.l r4,@(4,r15)
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[4]);
    // 0x8c23a808: mov.l r5,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    // 0x8c23a80a: jsr @r3 ; 0x8c23a80c: mov.l @(4,r15),r4 (delay slot)
    cpu.pr = 0x8c23a80e;
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    crate::registry::p_8c237780::run(cpu, mem);
    // 0x8c23a80e: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c23a810: bt 0x8c23a818
    if cpu.t() {
        // 0x8c23a818: mov.l 0x8c23a844,r0
        cpu.r[0] = mem.read_u32(0x8c23a844).unwrap_or(0x0000eeee);
    } else {
        // 0x8c23a812: mov.l @r15,r3
        cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        // 0x8c23a814: tst r3,r3
        cpu.set_t(cpu.r[3] == 0);
        // 0x8c23a816: bf 0x8c23a822
        if !cpu.t() {
            // 0x8c23a822: mov.l @r15,r3
            cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            // 0x8c23a824: mov #0,r0
            cpu.r[0] = 0;
            // 0x8c23a826: mov.l @(4,r15),r4
            cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
            // 0x8c23a828: mov.l r3,@(20,r4)
            let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[3]);
            // 0x8c23a82a: add #8,r15
            cpu.r[15] = cpu.r[15].wrapping_add(8);
            // 0x8c23a82c: lds.l @r15+,pr
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            // 0x8c23a82e: rts ; 0x8c23a830: nop
            return;
        }
        // 0x8c23a818: mov.l 0x8c23a844,r0
        cpu.r[0] = mem.read_u32(0x8c23a844).unwrap_or(0x0000eeee);
    }
    // 0x8c23a81a: add #8,r15
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    // 0x8c23a81c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c23a81e: rts ; 0x8c23a820: nop
}
