//! 0x8c239440 — masked lookup of `r4` in three open-addressed tables.
//!
//! A negative `r4` returns `0x101` and leaves `sr` alone apart from T.
//! Otherwise the imask nibble is saved in `r7`, raised to 15, and restored
//! on the way out. Each table is a halfword index plus a halfword probe
//! count at `0x8c2343a0`, with word keys at strides of 64. A hit returns
//! `0x100`; exhausting every table returns `0x101`. `r11`–`r14` are restored.
//! The `0xff0f` mask is sign-extended, so it clears only the imask nibble.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

fn sx16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

/// Rebuild `sr` from the saved imask in `r7` and the live `sr` (including T).
/// Hit sites leave the captured `sr` in `r1`; the miss site leaves it in `r3`.
fn restore_imask(cpu: &mut Cpu, mem: &Memory, mask_at: u32, via_r1: bool) {
    let captured = cpu.sr;
    cpu.r[0] = cpu.r[7] & 15;
    cpu.r[0] <<= 2;
    cpu.r[0] <<= 2;
    let masked = captured & sx16(mem, mask_at);
    if via_r1 {
        cpu.r[1] = captured;
        cpu.r[3] = sx16(mem, mask_at);
        cpu.r[1] = masked;
    } else {
        cpu.r[2] = sx16(mem, mask_at);
        cpu.r[3] = masked;
    }
    cpu.r[0] |= masked;
    cpu.sr = cpu.r[0];
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c239440: mov.l r14,@-r15
    push(cpu, mem, cpu.r[14]);
    // 0x8c239442: cmp/pz r4
    cpu.set_t((cpu.r[4] as i32) >= 0);
    // 0x8c239444..448: push r13, r12, r11
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    push(cpu, mem, cpu.r[11]);
    // 0x8c23944a: bt 0x8c239450
    if !cpu.t() {
        // 0x8c23944c: bra 0x8c239580 ; delay nop
        // 0x8c239580: mov.w 0x8c2395a6,r0  ! 0x101
        cpu.r[0] = sx16(mem, 0x8c2395a6);
    } else {
        // 0x8c239450: stc sr,r0 / mov.w 0xff0f,r2 / shlr2 / shlr2 / and #15
        cpu.r[0] = cpu.sr;
        cpu.r[2] = sx16(mem, 0x8c239500);
        cpu.r[0] >>= 2;
        cpu.r[0] >>= 2;
        cpu.r[0] &= 15;
        // 0x8c23945a: mov r0,r7
        cpu.r[7] = cpu.r[0];
        // 0x8c23945c: stc sr,r0 / and r2,r0 / or #240,r0 / ldc r0,sr
        cpu.r[0] = cpu.sr;
        cpu.r[0] &= cpu.r[2];
        cpu.r[0] |= 240;
        cpu.sr = cpu.r[0];
        // 0x8c239464: mov.l 0x8c239504,r5
        cpu.r[5] = mem.read_u32(0x8c239504).unwrap_or(0);
        // 0x8c239466: mov #46,r0 / mov #0,r11 / mov.w @(r0,r5),r13
        cpu.r[0] = 46;
        cpu.r[11] = 0;
        cpu.r[13] = sx16(mem, cpu.r[5].wrapping_add(cpu.r[0]));
        // 0x8c23946c: mov #48,r0 / mov.w @(r0,r5),r12
        cpu.r[0] = 48;
        cpu.r[12] = sx16(mem, cpu.r[5].wrapping_add(cpu.r[0]));
        // 0x8c239470: mov r11,r3 / mov r11,r14
        cpu.r[3] = cpu.r[11];
        cpu.r[14] = cpu.r[11];
        // 0x8c239474: cmp/hs r12,r3 / bt.s ; delay mov #31,r6
        cpu.set_t(cpu.r[3] >= cpu.r[12]);
        cpu.r[6] = 31;
        let mut hit = false;
        if !cpu.t() {
            loop {
                cpu.r[1] = cpu.r[13] << 6;
                cpu.r[3] = cpu.r[5].wrapping_add(64);
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
                cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                let eq = cpu.r[2] == cpu.r[4];
                cpu.set_t(eq);
                // bf.s delay: add #1,r13
                cpu.r[13] = cpu.r[13].wrapping_add(1);
                if eq {
                    restore_imask(cpu, mem, 0x8c239500, true);
                    cpu.r[0] = sx16(mem, 0x8c2395a4);
                    hit = true;
                    break;
                }
                cpu.r[14] = cpu.r[14].wrapping_add(1);
                let cont = cpu.r[14] < cpu.r[12];
                cpu.set_t(cpu.r[14] >= cpu.r[12]);
                // bf.s delay: and r6,r13
                cpu.r[13] &= cpu.r[6];
                if !cont {
                    break;
                }
            }
        }
        if !hit {
            // 0x8c2394ac: second table. Index in r14, probes in r13.
            cpu.r[0] = 52;
            cpu.r[14] = sx16(mem, cpu.r[5].wrapping_add(cpu.r[0]));
            cpu.r[0] = 54;
            cpu.r[12] = sx16(mem, cpu.r[5].wrapping_add(cpu.r[0]));
            cpu.r[3] = 0;
            cpu.set_t(cpu.r[3] >= cpu.r[12]);
            // bt.s delay: mov r11,r13
            cpu.r[13] = cpu.r[11];
            if !cpu.t() {
                loop {
                    cpu.r[1] = cpu.r[14] << 6;
                    cpu.r[3] = sx16(mem, 0x8c239502).wrapping_add(cpu.r[5]);
                    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
                    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                    let eq = cpu.r[2] == cpu.r[4];
                    cpu.set_t(eq);
                    cpu.r[14] = cpu.r[14].wrapping_add(1);
                    if eq {
                        restore_imask(cpu, mem, 0x8c239500, true);
                        cpu.r[0] = sx16(mem, 0x8c2395a4);
                        hit = true;
                        break;
                    }
                    cpu.r[13] = cpu.r[13].wrapping_add(1);
                    let cont = cpu.r[13] < cpu.r[12];
                    cpu.set_t(cpu.r[13] >= cpu.r[12]);
                    cpu.r[14] &= cpu.r[6];
                    if !cont {
                        break;
                    }
                }
            }
        }
        if !hit {
            // 0x8c239528: third table. Index in r13, probes in r14.
            cpu.r[0] = 58;
            cpu.r[13] = sx16(mem, cpu.r[5].wrapping_add(cpu.r[0]));
            cpu.r[0] = 60;
            cpu.r[12] = sx16(mem, cpu.r[5].wrapping_add(cpu.r[0]));
            cpu.r[3] = 0;
            cpu.set_t(cpu.r[3] >= cpu.r[12]);
            // bt.s delay: mov r11,r14
            cpu.r[14] = cpu.r[11];
            if !cpu.t() {
                loop {
                    cpu.r[1] = cpu.r[13] << 6;
                    cpu.r[3] = sx16(mem, 0x8c2395a0).wrapping_add(cpu.r[5]);
                    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
                    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                    let eq = cpu.r[2] == cpu.r[4];
                    cpu.set_t(eq);
                    cpu.r[13] = cpu.r[13].wrapping_add(1);
                    if eq {
                        restore_imask(cpu, mem, 0x8c2395a2, true);
                        cpu.r[0] = sx16(mem, 0x8c2395a4);
                        hit = true;
                        break;
                    }
                    cpu.r[14] = cpu.r[14].wrapping_add(1);
                    let cont = cpu.r[14] < cpu.r[12];
                    cpu.set_t(cpu.r[14] >= cpu.r[12]);
                    cpu.r[13] &= cpu.r[6];
                    if !cont {
                        break;
                    }
                }
            }
        }
        if !hit {
            // 0x8c23956e: miss. Captured sr stays in r3, mask in r2.
            restore_imask(cpu, mem, 0x8c2395a2, false);
            cpu.r[0] = sx16(mem, 0x8c2395a6);
        }
    }
    // 0x8c239582: pop r11, r12, r13 ; rts ; delay pop r14
    cpu.r[11] = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
