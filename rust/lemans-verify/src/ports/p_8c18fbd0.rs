//! 0x8c18fbd0 — same teardown shape as `0x8c12d33c`, with vtable
//! `0x8c1f0b34` and offsets 32 / `0xd4` / `0xbc` / `0x9c`.
//!
//! The low half of `r5` selects the depth: zero wires the first table only,
//! negative also installs `0x8c1eed70` and tears the two embedded objects
//! down, positive then frees `r4`. A null `r4` returns 0. `r10`–`r14` are
//! restored and `r0` is the entry `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c18fbd0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c18fbd2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c18fbd4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c18fbd6: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c18fbd8: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c18fbda: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c18fbdc: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    // 0x8c18fbde: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c18fbe0: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c18fbe2: bt 0x8c18fcc4
    if cpu.r[13] != 0 {
        // 0x8c18fbe4: mov.l 0x8c18fcd4,r0
        cpu.r[0] = mem.read_u32(0x8c18fcd4).unwrap_or(0);
        // 0x8c18fbe6: mov r13,r12
        cpu.r[12] = cpu.r[13];
        // 0x8c18fbe8: mov r13,r7
        cpu.r[7] = cpu.r[13];
        // 0x8c18fbea: mov.l r0,@(4,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[0]);
        // 0x8c18fbec: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c18fbee: mov.l 0x8c18fcd8,r4
        cpu.r[4] = mem.read_u32(0x8c18fcd8).unwrap_or(0);
        // 0x8c18fbf0: mov r0,r3
        cpu.r[3] = cpu.r[0];
        // 0x8c18fbf2: mov.l @r13,r5
        cpu.r[5] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fbf4: add r4,r3
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        // 0x8c18fbf6: mov r13,r6
        cpu.r[6] = cpu.r[13];
        // 0x8c18fbf8: mov.l r3,@(8,r5)
        let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
        // 0x8c18fbfa: add #32,r1
        cpu.r[1] = cpu.r[1].wrapping_add(32);
        // 0x8c18fbfc: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fbfe: mov r0,r11
        cpu.r[11] = cpu.r[0];
        // 0x8c18fc00: add #8,r12
        cpu.r[12] = cpu.r[12].wrapping_add(8);
        // 0x8c18fc02: mov.l @r3,r5
        cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c18fc04: add #36,r7
        cpu.r[7] = cpu.r[7].wrapping_add(36);
        // 0x8c18fc06: add #76,r6
        cpu.r[6] = cpu.r[6].wrapping_add(76);
        // 0x8c18fc08: mov.l r1,@r5
        let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
        // 0x8c18fc0a: mov r13,r3
        cpu.r[3] = cpu.r[13];
        // 0x8c18fc0c: mov.l 0x8c18fcdc,r5
        cpu.r[5] = mem.read_u32(0x8c18fcdc).unwrap_or(0);
        // 0x8c18fc0e: exts.w r14,r1
        cpu.r[1] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c18fc10: mov.l @r13,r10
        cpu.r[10] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fc12: add r5,r11
        cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[5]);
        // 0x8c18fc14: mov.l 0x8c18fce0,r5
        cpu.r[5] = mem.read_u32(0x8c18fce0).unwrap_or(0);
        // 0x8c18fc16: add #96,r3
        cpu.r[3] = cpu.r[3].wrapping_add(96);
        // 0x8c18fc18: mov.l @(4,r10),r10
        cpu.r[10] = mem.read_u32(cpu.r[10].wrapping_add(4)).unwrap_or(0);
        // 0x8c18fc1a: add r5,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]);
        // 0x8c18fc1c: tst r1,r1
        cpu.set_t(cpu.r[1] == 0);
        // 0x8c18fc1e: mov.l r11,@(4,r10)
        let _ = mem.write_u32(cpu.r[10].wrapping_add(4), cpu.r[11]);
        // 0x8c18fc20: mov.l @r13,r5
        cpu.r[5] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fc22: mov.l @(4,r5),r11
        cpu.r[11] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
        // 0x8c18fc24: mov.l @r11,r11
        cpu.r[11] = mem.read_u32(cpu.r[11]).unwrap_or(0);
        // 0x8c18fc26: mov r3,r5
        cpu.r[5] = cpu.r[3];
        // 0x8c18fc28: mov.l r0,@(4,r11)
        let _ = mem.write_u32(cpu.r[11].wrapping_add(4), cpu.r[0]);
        // 0x8c18fc2a: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fc2c: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c18fc2e: sub r0,r12
        cpu.r[12] = cpu.r[12].wrapping_sub(cpu.r[0]);
        // 0x8c18fc30: mov.l r12,@(24,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[12]);
        // 0x8c18fc32: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fc34: mov.l @(4,r0),r0
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
        // 0x8c18fc36: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c18fc38: sub r0,r7
        cpu.r[7] = cpu.r[7].wrapping_sub(cpu.r[0]);
        // 0x8c18fc3a: mov.l r7,@(36,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(36), cpu.r[7]);
        // 0x8c18fc3c: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fc3e: mov.l @(4,r0),r0
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
        // 0x8c18fc40: sub r0,r6
        cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[0]);
        // 0x8c18fc42: mov.l r6,@(16,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[6]);
        // 0x8c18fc44: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c18fc46: sub r0,r5
        cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[0]);
        // 0x8c18fc48: bt.s 0x8c18fcb8 ; delay 0x8c18fc4a always
        let _ = mem.write_u32(cpu.r[0].wrapping_add(12), cpu.r[5]);
        if !cpu.t() {
            // 0x8c18fc4c: tst r3,r3
            cpu.set_t(cpu.r[3] == 0);
            // 0x8c18fc4e: bt 0x8c18fc98
            if cpu.r[3] != 0 {
                // 0x8c18fc50: mov.l 0x8c18fce4,r1
                cpu.r[1] = mem.read_u32(0x8c18fce4).unwrap_or(0);
                // 0x8c18fc52: mov #104,r0
                cpu.r[0] = 104;
                // 0x8c18fc54: mov.l r1,@(r0,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(cpu.r[0]), cpu.r[1]);
                // 0x8c18fc56: mov r1,r5
                cpu.r[5] = cpu.r[1];
                // 0x8c18fc58: mov.l @r3,r6
                cpu.r[6] = mem.read_u32(cpu.r[3]).unwrap_or(0);
                // 0x8c18fc5a: add #56,r5
                cpu.r[5] = cpu.r[5].wrapping_add(56);
                // 0x8c18fc5c: mov r13,r0
                cpu.r[0] = cpu.r[13];
                // 0x8c18fc5e: mov.l r5,@r6
                let _ = mem.write_u32(cpu.r[6], cpu.r[5]);
                // 0x8c18fc60: add #100,r0
                cpu.r[0] = cpu.r[0].wrapping_add(100);
                // 0x8c18fc62: mov r1,r5
                cpu.r[5] = cpu.r[1];
                // 0x8c18fc64: add r4,r5
                cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[4]);
                // 0x8c18fc66: mov.l 0x8c18fce8,r4
                cpu.r[4] = mem.read_u32(0x8c18fce8).unwrap_or(0);
                // 0x8c18fc68: mov.l @r0,r6
                cpu.r[6] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c18fc6a: add r4,r1
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[4]);
                // 0x8c18fc6c: mov r13,r4
                cpu.r[4] = cpu.r[13];
                // 0x8c18fc6e: mov.l r5,@(4,r6)
                let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[5]);
                // 0x8c18fc70: mov.l @r0,r5
                cpu.r[5] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c18fc72: mov r13,r6
                cpu.r[6] = cpu.r[13];
                // 0x8c18fc74: add #108,r6
                cpu.r[6] = cpu.r[6].wrapping_add(108);
                // 0x8c18fc76: mov.l @r5,r7
                cpu.r[7] = mem.read_u32(cpu.r[5]).unwrap_or(0);
                // 0x8c18fc78: mov.l 0x8c18fcec,r5
                cpu.r[5] = mem.read_u32(0x8c18fcec).unwrap_or(0);
                // 0x8c18fc7a: mov.l r1,@(4,r7)
                let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[1]);
                // 0x8c18fc7c: add r5,r4
                cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
                // 0x8c18fc7e: mov.l @r3,r5
                cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
                // 0x8c18fc80: mov r13,r1
                cpu.r[1] = cpu.r[13];
                // 0x8c18fc82: mov.l 0x8c18fcf0,r3
                cpu.r[3] = mem.read_u32(0x8c18fcf0).unwrap_or(0);
                // 0x8c18fc84: sub r5,r6
                cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[5]);
                // 0x8c18fc86: mov.l r6,@(24,r5)
                let _ = mem.write_u32(cpu.r[5].wrapping_add(24), cpu.r[6]);
                // 0x8c18fc88: add r3,r1
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
                // 0x8c18fc8a: mov.l @r0,r3
                cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c18fc8c: mov.l @r3,r3
                cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0);
                // 0x8c18fc8e: sub r3,r4
                cpu.r[4] = cpu.r[4].wrapping_sub(cpu.r[3]);
                // 0x8c18fc90: mov.l r4,@(36,r3)
                let _ = mem.write_u32(cpu.r[3].wrapping_add(36), cpu.r[4]);
                // 0x8c18fc92: mov.l @r0,r0
                cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
                // 0x8c18fc94: sub r0,r1
                cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]);
                // 0x8c18fc96: mov.l r1,@(16,r0)
                let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[1]);
            }
            // 0x8c18fc98: mov.l 0x8c18fcf4,r0
            cpu.r[0] = mem.read_u32(0x8c18fcf4).unwrap_or(0);
            // 0x8c18fc9a: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c18fc9c: add #76,r4
            cpu.r[4] = cpu.r[4].wrapping_add(76);
            // 0x8c18fc9e: jsr @r0 ; delay 0x8c18fca0 mov #0,r5
            cpu.pr = 0x8c18fca2;
            cpu.r[5] = 0;
            crate::registry::p_8c0cd048::run(cpu, mem);
            // 0x8c18fca2: mov.l 0x8c18fcf8,r0
            cpu.r[0] = mem.read_u32(0x8c18fcf8).unwrap_or(0);
            // 0x8c18fca4: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c18fca6: add #36,r4
            cpu.r[4] = cpu.r[4].wrapping_add(36);
            // 0x8c18fca8: jsr @r0 ; delay 0x8c18fcaa mov #0,r5
            cpu.pr = 0x8c18fcac;
            cpu.r[5] = 0;
            crate::registry::p_8c07c100::run(cpu, mem);
            // 0x8c18fcac: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c18fcae: add #8,r0
            cpu.r[0] = cpu.r[0].wrapping_add(8);
            // 0x8c18fcb0: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c18fcb2: bt 0x8c18fcb8
            if cpu.r[0] != 0 {
                // 0x8c18fcb4: mov.l 0x8c18fcfc,r0
                cpu.r[0] = mem.read_u32(0x8c18fcfc).unwrap_or(0);
                // 0x8c18fcb6: mov.l r0,@(8,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]);
            }
        }
        // 0x8c18fcb8: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c18fcba: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c18fcbc: bf 0x8c18fcc4
        if (cpu.r[0] as i32) > 0 {
            // 0x8c18fcbe: mov.l 0x8c18fd00,r0
            cpu.r[0] = mem.read_u32(0x8c18fd00).unwrap_or(0);
            // 0x8c18fcc0: jsr @r0 ; delay 0x8c18fcc2 mov r13,r4
            cpu.pr = 0x8c18fcc4;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c18fcc4: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c18fcc6: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18fcc8: mov.l @r15+,r10
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18fcca: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18fccc: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18fcce: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c18fcd0: rts ; delay 0x8c18fcd2 mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
