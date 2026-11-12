//! 0x8c07687c — advance a 12-byte-entry ring cursor and its 84-byte shadow.
//!
//! `r4` points at a struct with fields
//! `+0x08` ring base, `+0x0c` shadow base, `+0x10` byte cursor,
//! `+0x14` countdown, `+0x18` ring entry count, `+0x20` shadow cursor,
//! `+0x24` accumulated count, `+0x28` current frame.
//!
//! The byte under `[+0x10]` is masked to 7 bits (`uVar1`). If it equals
//! `[+0x28]`, the shadow cursor resets to `[+0x0c]` and the accumulator to 0;
//! otherwise the shadow cursor advances by `84*uVar1` (wrapping back by
//! `84*[+0x28]` when it reaches `[+0x0c] + 84*[+0x28]`) and the accumulator
//! loses `uVar1`. Then the byte cursor advances 12, wrapping by
//! `12*[+0x18]` at `[+0x08] + 12*[+0x18]`, and the countdown decrements.
//!
//! Both `mul.l` instructions are reproduced: the second one is the one whose
//! MACH/MACL survive, and the equal path leaves both untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07687c: mov.l @(16,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
    // 0x8c07687e: mov.l @(40,r4),r5
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0);
    // 0x8c076880: mov.b @r0,r0 (sign-extends, undone by extu.b below)
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
    // 0x8c076882: extu.b r0,r0
    cpu.r[0] &= 0xff;
    // 0x8c076884: extu.b r0,r0
    cpu.r[0] &= 0xff;
    // 0x8c076886: and #127,r0
    cpu.r[0] &= 0x7f;
    // 0x8c076888: extu.b r0,r3
    cpu.r[3] = cpu.r[0] & 0xff;
    // 0x8c07688a: cmp/eq r3,r5
    cpu.set_t(cpu.r[5] == cpu.r[3]);
    // 0x8c07688c: bf 0x8c076898
    if !cpu.t() {
        // 0x8c076898: mov #84,r1
        cpu.r[1] = 84;
        // 0x8c07689a: mul.l r1,r3
        let p = (cpu.r[3] as i32 as i64) * (cpu.r[1] as i32 as i64);
        cpu.mach = (p >> 32) as u32;
        cpu.macl = p as u32;
        // 0x8c07689c: sts macl,r3
        cpu.r[3] = cpu.macl;
        // 0x8c07689e: mul.l r1,r5
        let p = (cpu.r[5] as i32 as i64) * (cpu.r[1] as i32 as i64);
        cpu.mach = (p >> 32) as u32;
        cpu.macl = p as u32;
        // 0x8c0768a0: mov.l @(32,r4),r6
        cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
        // 0x8c0768a2: mov.l @(12,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
        // 0x8c0768a4: sts macl,r5
        cpu.r[5] = cpu.macl;
        // 0x8c0768a6: add r3,r6
        cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[3]);
        // 0x8c0768a8: add r5,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]);
        // 0x8c0768aa: cmp/hs r0,r6
        cpu.set_t(cpu.r[6] >= cpu.r[0]);
        // 0x8c0768ac: bf 0x8c0768b0
        if cpu.t() {
            // 0x8c0768ae: sub r5,r6
            cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[5]);
        }
        // 0x8c0768b0: mov.l r6,@(32,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(32), cpu.r[6]);
        // 0x8c0768b2: mov.l @(16,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
        // 0x8c0768b4: mov.l @(36,r4),r1
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
        // 0x8c0768b6: mov.b @r0,r0
        cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
        // 0x8c0768b8: extu.b r0,r0
        cpu.r[0] &= 0xff;
        // 0x8c0768ba: and #127,r0
        cpu.r[0] &= 0x7f;
        // 0x8c0768bc: extu.b r0,r0
        cpu.r[0] &= 0xff;
        // 0x8c0768be: sub r0,r1
        cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]);
        // 0x8c0768c0: mov.l r1,@(36,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[1]);
    } else {
        // 0x8c07688e: mov.l @(12,r4),r1
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
        // 0x8c076890: mov #0,r0
        cpu.r[0] = 0;
        // 0x8c076892: mov.l r1,@(32,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(32), cpu.r[1]);
        // 0x8c076894: bra 0x8c0768c2 ; 0x8c076896: mov.l r0,@(36,r4) (delay slot)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[0]);
    }

    // 0x8c0768c2: mov.l @(24,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
    // 0x8c0768c4: mov.l @(16,r4),r5
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
    // 0x8c0768c6: mov r0,r3
    cpu.r[3] = cpu.r[0];
    // 0x8c0768c8: shll r3
    cpu.r[3] <<= 1;
    // 0x8c0768ca: add r0,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    // 0x8c0768cc: shll2 r3
    cpu.r[3] <<= 2;
    // 0x8c0768ce: mov.l @(8,r4),r1
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c0768d0: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c0768d2: add #12,r5
    cpu.r[5] = cpu.r[5].wrapping_add(12);
    // 0x8c0768d4: add r1,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    // 0x8c0768d6: cmp/hs r0,r5
    cpu.set_t(cpu.r[5] >= cpu.r[0]);
    // 0x8c0768d8: bf 0x8c0768dc
    if cpu.t() {
        // 0x8c0768da: sub r3,r5
        cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[3]);
    }
    // 0x8c0768dc: mov.l r5,@(16,r4)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[5]);
    // 0x8c0768de: mov.l @(20,r4),r0
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0);
    // 0x8c0768e0: add #-1,r0
    cpu.r[0] = cpu.r[0].wrapping_sub(1);
    // 0x8c0768e2: rts ; 0x8c0768e4: mov.l r0,@(20,r4) (delay slot)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[0]);
}
