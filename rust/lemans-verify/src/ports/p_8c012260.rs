//! 0x8c012260 — classify the bit pattern of `fr4` (with `fr5` as a companion):
//! 0 when `fr4` is finite, 2 when `fr4` is exactly +/-inf and `fr5` is +0.0,
//! 1 otherwise.
//!
//! Original spills both floats to the stack first and classifies from the
//! stored words:
//! ```text
//! add #-8,r15 ; mov r15,r4 ; add #4,r4 ; mov.l (0x7ff00000),r5
//! mov r15,r2 ; add #4,r2 ; fmov fr5,@r15 ; fmov fr4,@r2
//! mov.l @r4,r3 ; and r5,r3 ; cmp/eq r5,r3 ; bt 0x8c01227e ; mov #0,r0 ; rts ; add #8,r15
//! mov.l @r4,r2 ; mov.l (0x000fffff),r3 ; tst r3,r2 ; bf 0x8c01228e ; add #-4,r4
//! mov.l @r4,r3 ; tst r3,r3 ; bt 0x8c012294
//! mov #1,r0 ; rts ; add #8,r15
//! mov #2,r0 ; rts ; add #8,r15
//! ```
//! Side effects the C hides: `r15` is decremented and restored, `r4`/`r2`
//! keep stack addresses (and `r4` drops back by 4 on the last path), `r5`
//! keeps 0x7ff00000, `r3` keeps the last tested word, and T ends as the
//! result of the final `cmp/eq`/`tst` (1 on the return-0 and return-2 paths,
//! 0 on the return-1 paths).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r15 = cpu.r[15].wrapping_sub(8);
    cpu.r[15] = r15;
    cpu.r[4] = r15.wrapping_add(4);
    cpu.r[5] = 0x7ff0_0000; // mov.l 0x8c01229c,r5
    cpu.r[2] = r15.wrapping_add(4);
    // fmov fr5,@r15 ; fmov fr4,@r2 (both inside the stack window)
    let _ = mem.write_f32(r15, cpu.fr[5]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[4]);

    // mov.l @r4,r3 ; and r5,r3 ; cmp/eq r5,r3
    let bits4 = cpu.fr[4].to_bits();
    cpu.r[3] = bits4 & 0x7ff0_0000;
    cpu.set_t(cpu.r[3] == 0x7ff0_0000);
    if cpu.r[3] != 0x7ff0_0000 {
        // bt not taken: mov #0,r0 ; rts ; add #8,r15
        cpu.r[0] = 0;
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        return;
    }
    // bt taken (immediate in the oracle): mov.l @r4,r2 ; tst r3,r2 with r3=0x000fffff
    cpu.r[2] = bits4;
    cpu.r[3] = 0x000f_ffff; // mov.l 0x8c0122a0,r3
    cpu.set_t(cpu.r[2] & cpu.r[3] == 0);
    if cpu.r[2] & cpu.r[3] != 0 {
        // bf taken: mov #1,r0 ; rts ; add #8,r15
        cpu.r[0] = 1;
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        return;
    }
    // bf not taken: add #-4,r4 ; mov.l @r4,r3 ; tst r3,r3
    cpu.r[4] = cpu.r[4].wrapping_sub(4);
    cpu.r[3] = cpu.fr[5].to_bits();
    cpu.set_t(cpu.r[3] == 0);
    // bt 0x8c012294: mov #2,r0 when taken, else fall through to mov #1,r0
    cpu.r[0] = if cpu.r[3] == 0 { 2 } else { 1 };
    // rts ; add #8,r15
    cpu.r[15] = cpu.r[15].wrapping_add(8);
}
