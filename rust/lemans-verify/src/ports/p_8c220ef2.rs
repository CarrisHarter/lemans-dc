//! 0x8c220ef2 — bring up the 128-byte slot record at `r4`.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
//! r3 = [0x8c221040] (0x8c221da8) ; jsr @r3 ; r14 = r4 (delay)
//! tst r0,r0 ; bt 0x8c220f06
//! bra 0x8c220f26 ; mov #-1,r0 (delay)       ; first word nonzero -> -1
//! r3 = [0x8c221044] (0x8c221db8) ; r5 = 1 ; jsr @r3 ; r4 = r14 (delay)
//! r13 = 0 ; r2 = [0x8c221048] (0x8c221dc8)
//! push r13 ; r5 = r14 ; push r13 ; r6 = r13 ; push r13 ; r7 = r13
//! jsr @r2 ; r4 = r14 (delay)
//! add #12,r15 ; r0 = r13
//! out: lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 (delay)
//! ```
//!
//! The slot index is the entry `r4`; the first callee reads `+4` of that slot
//! and only a zero sends the routine on to store `+8 = 1` and fill `+20..+40`.
//! On the nonzero arm T is left by `tst r0,r0` (clear); on the slot-fill arm
//! the last flag setter is the third callee's final `shll` of the index.
//!
//! The prologue pushes `r14`, `r13` and `pr`, so all three come back at their
//! entry values; `r15` lands at its entry value too.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c220ef2: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let r14_entry = cpu.r[14];
    let _ = mem.write_u32(cpu.r[15], r14_entry);
    // 0x8c220ef4: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let r13_entry = cpu.r[13];
    let _ = mem.write_u32(cpu.r[15], r13_entry);
    // 0x8c220ef6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let pr_entry = cpu.pr;
    let _ = mem.write_u32(cpu.r[15], pr_entry);
    // 0x8c220ef8: mov.l 0x8c221040,r3 (0x8c221da8)
    cpu.r[3] = mem.read_u32(0x8c221040).unwrap_or(0);
    // 0x8c220efa: jsr @r3 ; 0x8c220efc: mov r4,r14 (delay slot)
    cpu.r[14] = cpu.r[4];
    cpu.pr = 0x8c220efe;
    crate::registry::p_8c221da8::run(cpu, mem);
    // 0x8c220efe: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c220f00: bt 0x8c220f06
    if cpu.t() {
        // 0x8c220f06: mov.l 0x8c221044,r3 (0x8c221db8)
        cpu.r[3] = mem.read_u32(0x8c221044).unwrap_or(0);
        // 0x8c220f08: mov #1,r5
        cpu.r[5] = 1;
        // 0x8c220f0a: jsr @r3 ; 0x8c220f0c: mov r14,r4 (delay slot)
        cpu.r[4] = cpu.r[14];
        cpu.pr = 0x8c220f0e;
        crate::registry::p_8c221db8::run(cpu, mem);
        // 0x8c220f0e: mov #0,r13
        cpu.r[13] = 0;
        // 0x8c220f10: mov.l 0x8c221048,r2 (0x8c221dc8)
        cpu.r[2] = mem.read_u32(0x8c221048).unwrap_or(0);
        // 0x8c220f12: mov.l r13,@-r15
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
        // 0x8c220f14: mov r14,r5
        cpu.r[5] = cpu.r[14];
        // 0x8c220f16: mov.l r13,@-r15
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
        // 0x8c220f18: mov r13,r6
        cpu.r[6] = cpu.r[13];
        // 0x8c220f1a: mov.l r13,@-r15
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
        // 0x8c220f1c: mov r13,r7
        cpu.r[7] = cpu.r[13];
        // 0x8c220f1e: jsr @r2 ; 0x8c220f20: mov r14,r4 (delay slot)
        cpu.r[4] = cpu.r[14];
        cpu.pr = 0x8c220f22;
        crate::registry::p_8c221dc8::run(cpu, mem);
        // 0x8c220f22: add #12,r15
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        // 0x8c220f24: mov r13,r0
        cpu.r[0] = cpu.r[13];
    } else {
        // 0x8c220f02: bra 0x8c220f26 ; 0x8c220f04: mov #-1,r0 (delay slot)
        cpu.r[0] = 0xffff_ffff;
    }
    // 0x8c220f26: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c220f28: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c220f2a: rts ; 0x8c220f2c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
