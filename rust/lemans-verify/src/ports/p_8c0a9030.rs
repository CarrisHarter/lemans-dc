//! 0x8c0a9030 — write a 6-field record at `r4` and flag the slot at
//! `[DAT_8c0a9070 + r4]`, copying 4 words from `r5` plus `fr4/fr5/fr6`.
//!
//! Original:
//! ```text
//! mov.l r14,@-r15 ; mov.l 0x8c0a9070,r0 ; mov #1,r6 ; mov.l 0x8c0a9074,r3
//! mov r4,r1 ; mov r4,r14 ; mov.b r6,@(r0,r4) ; add r3,r1 ; fmov fr4,@r1
//! mov r4,r6 ; mov.l 0x8c0a9078,r1 ; mov.l @r5,r7 ; add r1,r14
//! mov.l 0x8c0a907c,r1 ; mov.l @(4,r5),r3 ; add r1,r6 ; mov.l r7,@r14
//! mov.l r3,@(4,r14) ; mov.l @(8,r5),r7 ; mov r4,r3 ; mov.l 0x8c0a9080,r1
//! mov.l @(12,r5),r5 ; add r1,r3 ; mov #0,r1 ; mov.l r7,@(8,r14)
//! mov.l r5,@(12,r14) ; fmov fr5,@r6 ; fmov fr6,@r3 ; mov.b r1,@(r0,r4)
//! rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r0` keeps the DAT_8c0a9070 pointer, `r1` ends 0,
//! `r3` = r4+0x254, `r5` = 4th source word, `r6` = r4+0x250, `r7` = 3rd source
//! word; `r14` is pushed/popped so the seed survives. T is never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0a9070).unwrap_or(0); // 0x26c
    let base = cpu.r[0];
    cpu.r[6] = 1;
    let off1 = mem.read_u32(0x8c0a9074).unwrap_or(0); // 0x258
    cpu.r[1] = cpu.r[4];
    let r14 = cpu.r[4]; // r14 is pushed/popped; the seed survives, work in a local
    let _ = mem.write_u8(base.wrapping_add(cpu.r[4]), cpu.r[6] as u8);
    cpu.r[1] = cpu.r[1].wrapping_add(off1);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[4]);
    cpu.r[6] = cpu.r[4];
    let off2 = mem.read_u32(0x8c0a9078).unwrap_or(0); // 0x25c
    let r7 = mem.read_u32(cpu.r[5]).unwrap_or(0);
    let r14 = r14.wrapping_add(off2);
    let off3 = mem.read_u32(0x8c0a907c).unwrap_or(0); // 0x250
    let r3 = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(off3);
    let _ = mem.write_u32(r14, r7);
    let _ = mem.write_u32(r14.wrapping_add(4), r3);
    cpu.r[7] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.r[3] = cpu.r[4];
    let off4 = mem.read_u32(0x8c0a9080).unwrap_or(0); // 0x254
    let r5 = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
    cpu.r[3] = cpu.r[3].wrapping_add(off4);
    cpu.r[1] = 0;
    let _ = mem.write_u32(r14.wrapping_add(8), cpu.r[7]);
    let _ = mem.write_u32(r14.wrapping_add(12), r5);
    let _ = mem.write_f32(cpu.r[6], cpu.fr[5]);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[6]);
    let _ = mem.write_u8(base.wrapping_add(cpu.r[4]), cpu.r[1] as u8);
    cpu.r[5] = r5;
}
