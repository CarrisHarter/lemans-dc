//! 0x8c0db7dc — copy a record out of a source block and initialise it.
//!
//! `r4` is the destination object (`a`), `r5` the source block (`b`), `r6` a
//! flag byte. The function:
//!
//! ```text
//! [a+0x38] = [b+4] ; [a+0x3c] = [b+4]
//! if ([[0x8c2ab468]+0x28] == 3) [a+0x3c] = 57600.0f   ; via the stack slot
//! [a+0x5b8] = [b+8]
//! [a+0x5c0] = (u8)[b+0x71c]
//! [a+0x5bc] = (f32)[b+0x720]
//! f_8c066604([a+0x53c])                              ; tail-writes [.. +0x9c]=1
//! [a+0xbc] = 0                                       ; bt.s delay slot
//! if ((u8)flag != 0) f_8c1ccc18([a+0x5e4] + 0xd0)    ; init_record_a
//! else               f_8c1ccc3c([a+0x5e4] + 0xd0)    ; init_record_b
//! ```
//!
//! The equality test's `bf` has no delay slot; the `bt.s` and both `jsr`s do.
//! The 57600.0 constant is materialised through the 4-byte stack slot with
//! `mova`/`fmov`, so the low word survives as the u32 0x47610000.
//!
//! `0x8c066604` tail-jumps to `0x8c067f08`, which leaves `r0=0x9c`, `r1=1`;
//! the later literal loads overwrite both. The init routines leave `r0=0x178c`
//! and `r1=0`, `fr0=1.0`, and set `r3=1` (record_a) or leave `r3` alone
//! (record_b). Neither init routine touches T, so the result is the `tst r1,r1`
//! of the flag test: set iff `(u8)flag == 0`. `r13`/`r14` come back from the
//! stack at their entry values, as does `r15` after the 4-byte local.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0db7dc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0db7de: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0db7e0: mov r6,r14 — flag
    cpu.r[14] = cpu.r[6];
    // 0x8c0db7e2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0db7e4: add #-4,r15 — local slot for the f32 round-trip
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c0db7e6: mov r4,r13 — destination object
    cpu.r[13] = cpu.r[4];
    // 0x8c0db7e8: mov r5,r1
    cpu.r[1] = cpu.r[5];
    // 0x8c0db7ea: add #4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    // 0x8c0db7ec: mov.l @r1,r0
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    // 0x8c0db7ee: mov r13,r3
    cpu.r[3] = cpu.r[13];
    // 0x8c0db7f0: add #56,r3
    cpu.r[3] = cpu.r[3].wrapping_add(56);
    // 0x8c0db7f2: mov.l r0,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    // 0x8c0db7f4: mov.l @r1,r1
    cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    // 0x8c0db7f6: mov r13,r3
    cpu.r[3] = cpu.r[13];
    // 0x8c0db7f8: mov.l 0x8c0db870,r0 — global object pointer
    cpu.r[0] = mem.read_u32(0x8C0DB870).unwrap_or(0);
    // 0x8c0db7fa: add #60,r3
    cpu.r[3] = cpu.r[3].wrapping_add(60);
    // 0x8c0db7fc: mov.l r1,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    // 0x8c0db7fe: mov.l @r0,r0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c0db800: mov.l @(40,r0),r0
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(40)).unwrap_or(0);
    // 0x8c0db802: cmp/eq #3,r0
    let mode_is_3 = cpu.r[0] == 3;
    cpu.set_t(mode_is_3);
    // 0x8c0db804: bf 0x8c0db816 — no delay slot
    if mode_is_3 {
        // 0x8c0db806: mova 0x8c0db874,r0
        cpu.r[0] = 0x8C0DB874;
        // 0x8c0db808: fmov @r0,fr0 — 57600.0f
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c0db80a: mov #0,r2
        cpu.r[2] = 0;
        // 0x8c0db80c: add r15,r2
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
        // 0x8c0db80e: fmov fr0,@r2
        let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
        // 0x8c0db810: mov r15,r0
        cpu.r[0] = cpu.r[15];
        // 0x8c0db812: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c0db814: mov.l r0,@r3
        let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    }
    // 0x8c0db816: mov.l @(8,r5),r1
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    // 0x8c0db818: mov r13,r3
    cpu.r[3] = cpu.r[13];
    // 0x8c0db81a: mov.l 0x8c0db878,r0 — 0x5b8
    cpu.r[0] = mem.read_u32(0x8C0DB878).unwrap_or(0);
    // 0x8c0db81c: mov.l 0x8c0db87c,r7 — 0x5c0
    cpu.r[7] = mem.read_u32(0x8C0DB87C).unwrap_or(0);
    // 0x8c0db81e: mov.l r1,@(r0,r13)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[1]);
    // 0x8c0db820: mov.l 0x8c0db880,r0 — 0x71c
    cpu.r[0] = mem.read_u32(0x8C0DB880).unwrap_or(0);
    // 0x8c0db822: mov.l 0x8c0db884,r4 — 0x5bc
    cpu.r[4] = mem.read_u32(0x8C0DB884).unwrap_or(0);
    // 0x8c0db824: mov.b @(r0,r5),r6 — sign-extends
    cpu.r[6] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0) as i8 as i32 as u32;
    // 0x8c0db826: add r4,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    // 0x8c0db828: mov.l 0x8c0db888,r1 — 0x720
    cpu.r[1] = mem.read_u32(0x8C0DB888).unwrap_or(0);
    // 0x8c0db82a: mov r7,r0
    cpu.r[0] = cpu.r[7];
    // 0x8c0db82c: extu.b r6,r6
    cpu.r[6] &= 0xff;
    // 0x8c0db82e: mov.b r6,@(r0,r13)
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[6] as u8);
    // 0x8c0db830: add r1,r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
    // 0x8c0db832: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    // 0x8c0db834: mov.l 0x8c0db88c,r0 — 0x53c
    cpu.r[0] = mem.read_u32(0x8C0DB88C).unwrap_or(0);
    // 0x8c0db836: fmov fr0,@r3
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    // 0x8c0db838: mov.l 0x8c0db890,r1 — 0x8c066604
    cpu.r[1] = mem.read_u32(0x8C0DB890).unwrap_or(0);
    // 0x8c0db83a: jsr @r1 ; 0x8c0db83c: mov.l @(r0,r13),r4 (delay)
    cpu.pr = 0x8C0DB83E;
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    crate::registry::p_8c066604::run(cpu, mem);
    // 0x8c0db83e: mov.l 0x8c0db894,r0 — 0xbc
    cpu.r[0] = mem.read_u32(0x8C0DB894).unwrap_or(0);
    // 0x8c0db840: mov #0,r3
    cpu.r[3] = 0;
    // 0x8c0db842: extu.b r14,r1
    cpu.r[1] = cpu.r[14] & 0xff;
    // 0x8c0db844: tst r1,r1
    let flag_is_zero = (cpu.r[1] & cpu.r[1]) == 0;
    cpu.set_t(flag_is_zero);
    // 0x8c0db846: bt.s 0x8c0db85a ; 0x8c0db848: mov.l r3,@(r0,r13) (delay)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[3]);
    if flag_is_zero {
        // 0x8c0db85a: mov.l 0x8c0db898,r0 — 0x5e4
        cpu.r[0] = mem.read_u32(0x8C0DB898).unwrap_or(0);
        // 0x8c0db85c: mov.l 0x8c0db89c,r1 — 0xd0
        cpu.r[1] = mem.read_u32(0x8C0DB89C).unwrap_or(0);
        // 0x8c0db85e: mov.l @(r0,r13),r4
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
        // 0x8c0db860: mov.l 0x8c0db8a4,r0 — 0x8c1ccc3c
        cpu.r[0] = mem.read_u32(0x8C0DB8A4).unwrap_or(0);
        // 0x8c0db862: jsr @r0 ; 0x8c0db864: add r1,r4 (delay)
        cpu.pr = 0x8C0DB866;
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
        crate::registry::p_8c1ccc3c::run(cpu, mem);
    } else {
        // 0x8c0db84a: mov.l 0x8c0db898,r0 — 0x5e4
        cpu.r[0] = mem.read_u32(0x8C0DB898).unwrap_or(0);
        // 0x8c0db84c: mov.l 0x8c0db89c,r1 — 0xd0
        cpu.r[1] = mem.read_u32(0x8C0DB89C).unwrap_or(0);
        // 0x8c0db84e: mov.l @(r0,r13),r4
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
        // 0x8c0db850: mov.l 0x8c0db8a0,r0 — 0x8c1ccc18
        cpu.r[0] = mem.read_u32(0x8C0DB8A0).unwrap_or(0);
        // 0x8c0db852: jsr @r0 ; 0x8c0db854: add r1,r4 (delay)
        cpu.pr = 0x8C0DB856;
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
        crate::registry::p_8c1ccc18::run(cpu, mem);
        // 0x8c0db856: bra 0x8c0db866 ; 0x8c0db858: nop (delay)
    }
    // 0x8c0db866: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0db868: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0db86a: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0db86c: rts ; 0x8c0db86e: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
