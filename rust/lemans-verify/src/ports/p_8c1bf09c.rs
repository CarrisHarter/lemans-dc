//! 0x8c1bf09c — look the object up under three global names, then set or
//! clear bits 0 and 1 of each hit according to the selector stored at
//! `[r4+56]` (the entry `r5`).
//!
//! Selector 0 sets both bits on the first hit and clears them on the other
//! two. Selector 1 sets them on the second hit. Selector 2 sets them on the
//! third. Any other selector leaves the flag words alone. `r11`–`r14` are
//! restored. Each lookup is `0x8c05b1ec`; each bit update is `0x8c05d274`.

use sh4_oracle::{Cpu, Memory};

fn call_lookup(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = mem.read_u32(0x8c1bf1ac).unwrap_or(0); // 0x8c05b1ec
    cpu.pr = ret;
    crate::registry::p_8c05b1ec::run(cpu, mem);
}

fn call_apply(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = mem.read_u32(0x8c1bf1b8).unwrap_or(0); // 0x8c05d274
    cpu.pr = ret;
    crate::registry::p_8c05d274::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1bf09c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c1bf09e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c1bf0a0: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c1bf0a2: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c1bf0a4: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1bf0a6: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c1bf0a8: mov.l r5,@(56,r11)
    let _ = mem.write_u32(cpu.r[11].wrapping_add(56), cpu.r[5]);
    // 0x8c1bf0aa: mov.l 0x8c1bf1a8,r0
    cpu.r[0] = mem.read_u32(0x8c1bf1a8).unwrap_or(0);
    // 0x8c1bf0ac: mov.l @r0,r5
    cpu.r[5] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c1bf0ae: jsr 0x8c05b1ec ; delay nop. Return 0x8c1bf0b4.
    call_lookup(cpu, mem, 0x8c1bf0b4);
    // 0x8c1bf0b4: mov.l 0x8c1bf1b0,r1
    cpu.r[1] = mem.read_u32(0x8c1bf1b0).unwrap_or(0);
    // 0x8c1bf0b6: mov r0,r12
    cpu.r[12] = cpu.r[0];
    // 0x8c1bf0ba: mov r11,r4
    cpu.r[4] = cpu.r[11];
    // 0x8c1bf0bc: jsr ; delay 0x8c1bf0be mov.l @r1,r5. Return 0x8c1bf0c0.
    cpu.r[5] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    call_lookup(cpu, mem, 0x8c1bf0c0);
    // 0x8c1bf0c0: mov.l 0x8c1bf1b4,r1
    cpu.r[1] = mem.read_u32(0x8c1bf1b4).unwrap_or(0);
    // 0x8c1bf0c2: mov r0,r13
    cpu.r[13] = cpu.r[0];
    // 0x8c1bf0c6: mov r11,r4
    cpu.r[4] = cpu.r[11];
    // 0x8c1bf0c8: jsr ; delay 0x8c1bf0ca mov.l @r1,r5. Return 0x8c1bf0cc.
    cpu.r[5] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    call_lookup(cpu, mem, 0x8c1bf0cc);
    // 0x8c1bf0cc: mov r0,r14
    cpu.r[14] = cpu.r[0];
    // 0x8c1bf0ce: mov.l @(56,r11),r0
    cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(56)).unwrap_or(0);
    // 0x8c1bf0d0: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c1bf0d2: bt 0x8c1bf0e0
    if cpu.r[0] == 0 {
        // first hit: set bits 0 and 1. The other two: clear them.
        cpu.r[6] = 1;
        cpu.r[5] = cpu.r[6];
        cpu.r[4] = cpu.r[12];
        call_apply(cpu, mem, 0x8c1bf0ea);
        cpu.r[4] = cpu.r[12];
        cpu.r[6] = 1;
        cpu.r[5] = 2;
        call_apply(cpu, mem, 0x8c1bf0f4);
        cpu.r[4] = cpu.r[13];
        cpu.r[6] = 0;
        cpu.r[5] = 1;
        call_apply(cpu, mem, 0x8c1bf0fe);
        cpu.r[4] = cpu.r[13];
        cpu.r[6] = 0;
        cpu.r[5] = 2;
        call_apply(cpu, mem, 0x8c1bf108);
        cpu.r[4] = cpu.r[14];
        cpu.r[6] = 0;
        cpu.r[5] = 1;
        call_apply(cpu, mem, 0x8c1bf112);
        cpu.r[4] = cpu.r[14];
        cpu.r[6] = 0;
        cpu.r[5] = 2;
        call_apply(cpu, mem, 0x8c1bf11c);
    } else {
        // 0x8c1bf0d4: cmp/eq #1,r0
        cpu.set_t(cpu.r[0] == 1);
        // 0x8c1bf0d6: bt 0x8c1bf120
        if cpu.r[0] == 1 {
            cpu.r[4] = cpu.r[12];
            cpu.r[6] = 0;
            cpu.r[5] = 1;
            call_apply(cpu, mem, 0x8c1bf12a);
            cpu.r[4] = cpu.r[12];
            cpu.r[6] = 0;
            cpu.r[5] = 2;
            call_apply(cpu, mem, 0x8c1bf134);
            cpu.r[6] = 1;
            cpu.r[5] = cpu.r[6];
            cpu.r[4] = cpu.r[13];
            call_apply(cpu, mem, 0x8c1bf13e);
            cpu.r[4] = cpu.r[13];
            cpu.r[6] = 1;
            cpu.r[5] = 2;
            call_apply(cpu, mem, 0x8c1bf148);
            cpu.r[4] = cpu.r[14];
            cpu.r[6] = 0;
            cpu.r[5] = 1;
            call_apply(cpu, mem, 0x8c1bf152);
            cpu.r[4] = cpu.r[14];
            cpu.r[6] = 0;
            cpu.r[5] = 2;
            call_apply(cpu, mem, 0x8c1bf15c);
        } else {
            // 0x8c1bf0d8: cmp/eq #2,r0
            cpu.set_t(cpu.r[0] == 2);
            // 0x8c1bf0da: bt 0x8c1bf160
            if cpu.r[0] == 2 {
                cpu.r[4] = cpu.r[12];
                cpu.r[6] = 0;
                cpu.r[5] = 1;
                call_apply(cpu, mem, 0x8c1bf16a);
                cpu.r[4] = cpu.r[12];
                cpu.r[6] = 0;
                cpu.r[5] = 2;
                call_apply(cpu, mem, 0x8c1bf174);
                cpu.r[4] = cpu.r[13];
                cpu.r[6] = 0;
                cpu.r[5] = 1;
                call_apply(cpu, mem, 0x8c1bf17e);
                cpu.r[4] = cpu.r[13];
                cpu.r[6] = 0;
                cpu.r[5] = 2;
                call_apply(cpu, mem, 0x8c1bf188);
                cpu.r[6] = 1;
                cpu.r[5] = cpu.r[6];
                cpu.r[4] = cpu.r[14];
                call_apply(cpu, mem, 0x8c1bf192);
                cpu.r[4] = cpu.r[14];
                cpu.r[6] = 1;
                cpu.r[5] = 2;
                call_apply(cpu, mem, 0x8c1bf19c);
            }
            // else bra 0x8c1bf19c with a nop delay: r0 stays the selector
        }
    }
    // 0x8c1bf19c: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1bf19e: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1bf1a0: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1bf1a2: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1bf1a4: rts ; delay 0x8c1bf1a6 mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
