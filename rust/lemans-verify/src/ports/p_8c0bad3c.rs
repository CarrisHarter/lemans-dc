//! 0x8c0bad3c — free both tracked block lists and clear their heads.
//!
//! If `mem[0x8c26d8e8 + 0xc610]` is null the routine returns 0 immediately.
//! Otherwise it first drains the list at `+0xc614`, then the list at `+0xc610`,
//! and finally zeroes both heads. A list entry `q` is a payload pointer: the
//! block passed to `free` is `q - 8` and the next entry is read from `q - 4`
//! before the call (the `jsr` delay slot). The callee's size word lives at
//! `q - 12`, so the walk is unaffected by the callee's negation.
//!
//! `r14` and `pr` are saved on entry and restored on exit, so both end at their
//! entry values. The last flag-setting instruction on every halting path is a
//! `tst` of the just-loaded next pointer (or of the list head), leaving T=1.

use sh4_oracle::{Cpu, Memory};

const BASE: u32 = 0x8c26d8e8;
const OFF_A: u32 = 0xc610;
const OFF_B: u32 = 0xc614;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0bad3c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 8c0bad3e: mov.l <pool>,r1
    cpu.r[1] = OFF_A;
    // 8c0bad40: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c0bad42: mov.l <pool>,r0 ; 8c0bad44: mov.l @(r0,r1),r1
    cpu.r[0] = BASE;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    // 8c0bad46: tst r1,r1 ; 8c0bad48: bt 0x8c0bad84
    cpu.set_t(cpu.r[1] == 0);
    if !cpu.t() {
        // 8c0bad4a: mov.l <pool>,r1 ; 8c0bad4c: mov.l @(r0,r1),r4
        cpu.r[1] = OFF_B;
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        // 8c0bad4e: tst r4,r4 ; 8c0bad50: bt 0x8c0bad60
        cpu.set_t((cpu.r[4] & cpu.r[4]) == 0);
        if !cpu.t() {
            loop {
                // 8c0bad52: add #-8,r4
                cpu.r[4] = cpu.r[4].wrapping_sub(8);
                // 8c0bad54: mov.l <pool>,r0
                cpu.r[0] = mem.read_u32(0x8c0bad98).unwrap_or(0);
                // 8c0bad56: jsr @r0 ; 8c0bad58: mov.l @(4,r4),r14 (delay slot)
                cpu.r[14] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
                cpu.pr = 0x8c0bad5a;
                crate::registry::p_8c159fc0::run(cpu, mem);
                // 8c0bad5a: tst r14,r14 ; 8c0bad5c: bf.s 0x8c0bad52
                cpu.set_t((cpu.r[14] & cpu.r[14]) == 0);
                let t = cpu.t();
                // 8c0bad5e: mov r14,r4 (delay slot, always runs)
                cpu.r[4] = cpu.r[14];
                if t {
                    break;
                }
            }
        }
        // 8c0bad60: mov.l <pool>,r1 ; 8c0bad62: mov.l <pool>,r0
        cpu.r[1] = BASE;
        cpu.r[0] = OFF_A;
        // 8c0bad64: mov.l @(r0,r1),r4 ; 8c0bad66: tst r4,r4 ; bt 0x8c0bad78
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        cpu.set_t((cpu.r[4] & cpu.r[4]) == 0);
        if !cpu.t() {
            loop {
                // 8c0bad6a: add #-8,r4
                cpu.r[4] = cpu.r[4].wrapping_sub(8);
                // 8c0bad6c: mov.l <pool>,r0
                cpu.r[0] = mem.read_u32(0x8c0bad98).unwrap_or(0);
                // 8c0bad6e: jsr @r0 ; 8c0bad70: mov.l @(4,r4),r14 (delay slot)
                cpu.r[14] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
                cpu.pr = 0x8c0bad72;
                crate::registry::p_8c159fc0::run(cpu, mem);
                // 8c0bad72: tst r14,r14 ; 8c0bad74: bf.s 0x8c0bad6a
                cpu.set_t((cpu.r[14] & cpu.r[14]) == 0);
                let t = cpu.t();
                // 8c0bad76: mov r14,r4 (delay slot, always runs)
                cpu.r[4] = cpu.r[14];
                if t {
                    break;
                }
            }
        }
        // 8c0bad78: mov.l <pool>,r1 ; 8c0bad7a: mov #0,r3
        cpu.r[1] = BASE;
        cpu.r[3] = 0;
        // 8c0bad7c: mov.l <pool>,r0 ; 8c0bad7e: mov.l r3,@(r0,r1)
        cpu.r[0] = OFF_A;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[3]);
        // 8c0bad80: mov.l <pool>,r0 ; 8c0bad82: mov.l r3,@(r0,r1)
        cpu.r[0] = OFF_B;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[3]);
    }
    // 8c0bad84: mov #0,r0
    cpu.r[0] = 0;
    // 8c0bad86: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c0bad88: rts ; 8c0bad8a: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
