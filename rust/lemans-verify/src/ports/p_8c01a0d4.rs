//! 0x8c01a0d4 — recursive integer accumulation.
//!
//! Forsyth-style port of the original, instruction for instruction. The
//! routine pushes `r14`/`pr`, computes `r14 = n`, then walks a ladder of
//! `cmp/pl` early exits:
//!
//! * `n <= 0` → `0`
//! * `n == 1` → `1`
//! * `n == 2` → `3`
//! * `n == 3` → `6`
//! * `n >= 4` → `f(n - 4) + 4n - 6`
//!
//! The `jsr` at 0x8c01a112 targets 0x8c01a0d4 itself (literal pool word at
//! 0x8c01a13c), so the "call" is direct recursion and goes through the ported
//! registry entry. Every early-exit path leaves its own residue in `r0`/`r1`/
//! `r3`, and the recursion leaves `r4` and `T` from the innermost frame, so all
//! of it is modelled here rather than folded into the final formula.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01a0d4  mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c01a0d6  sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c01a0d8  mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c01a0da  cmp/pl r14
    let t = (cpu.r[14] as i32) > 0;
    cpu.set_t(t);
    if t {
        // 0x8c01a0e2  mov r14,r0
        cpu.r[0] = cpu.r[14];
        // 0x8c01a0e4  mov #-1,r1
        cpu.r[1] = 0xffff_ffff;
        // 0x8c01a0e6  add r1,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
        // 0x8c01a0e8  cmp/pl r0
        let t = (cpu.r[0] as i32) > 0;
        cpu.set_t(t);
        if t {
            // 0x8c01a0f0  mov.l 0x8c01a134,r1  (0xfffffffe)
            cpu.r[1] = mem.read_u32(0x8c01a134).unwrap_or(0);
            // 0x8c01a0f2  mov r14,r0 ; 0x8c01a0f4 add r1,r0
            cpu.r[0] = cpu.r[14].wrapping_add(cpu.r[1]);
            // 0x8c01a0f6  cmp/pl r0
            let t = (cpu.r[0] as i32) > 0;
            cpu.set_t(t);
            if t {
                // 0x8c01a0fe  mov r14,r0 ; mov #-3,r1 ; add r1,r0
                cpu.r[0] = cpu.r[14];
                cpu.r[1] = 0xffff_fffd;
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
                // 0x8c01a104  cmp/pl r0
                let t = (cpu.r[0] as i32) > 0;
                cpu.set_t(t);
                if t {
                    // 0x8c01a10c  mov.l 0x8c01a138,r0  (0xfffffffc)
                    cpu.r[0] = mem.read_u32(0x8c01a138).unwrap_or(0);
                    // 0x8c01a10e  add r0,r4
                    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
                    // 0x8c01a110  mov.l 0x8c01a13c,r0  (0x8c01a0d4)
                    cpu.r[0] = mem.read_u32(0x8c01a13c).unwrap_or(0);
                    // 0x8c01a112  jsr @r0, delay slot 0x8c01a114 is `nop`
                    crate::registry::p_8c01a0d4::run(cpu, mem);
                    // 0x8c01a116  mov r14,r1 ; 0x8c01a118 add #-3,r1
                    cpu.r[1] = cpu.r[14].wrapping_add(0xffff_fffd);
                    // 0x8c01a11a  add r0,r1
                    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
                    // 0x8c01a11c  mov.l 0x8c01a134,r0  (0xfffffffe)
                    cpu.r[0] = mem.read_u32(0x8c01a134).unwrap_or(0);
                    // 0x8c01a11e  mov r14,r3 ; 0x8c01a120 add r0,r3
                    cpu.r[3] = cpu.r[14].wrapping_add(cpu.r[0]);
                    // 0x8c01a122  add r1,r3
                    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
                } else {
                    // 0x8c01a108  bra 0x8c01a11c, delay slot 0x8c01a10a mov #0,r1
                    cpu.r[1] = 0;
                    // 0x8c01a11c  mov.l 0x8c01a134,r0  (0xfffffffe)
                    cpu.r[0] = mem.read_u32(0x8c01a134).unwrap_or(0);
                    // 0x8c01a11e  mov r14,r3 ; 0x8c01a120 add r0,r3
                    cpu.r[3] = cpu.r[14].wrapping_add(cpu.r[0]);
                    // 0x8c01a122  add r1,r3
                    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
                }
                // 0x8c01a124  mov r14,r0 ; 0x8c01a126 add #-1,r0
                cpu.r[0] = cpu.r[14].wrapping_add(0xffff_ffff);
                // 0x8c01a128  add r3,r0
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
            } else {
                // 0x8c01a0fa  bra 0x8c01a124, delay slot 0x8c01a0fc mov #0,r3
                cpu.r[3] = 0;
                // 0x8c01a124  mov r14,r0 ; 0x8c01a126 add #-1,r0
                cpu.r[0] = cpu.r[14].wrapping_add(0xffff_ffff);
                // 0x8c01a128  add r3,r0
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
            }
            // 0x8c01a12a  add r14,r0
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[14]);
        } else {
            // 0x8c01a0ec  bra 0x8c01a12a, delay slot 0x8c01a0ee mov #0,r0
            cpu.r[0] = 0;
            // 0x8c01a12a  add r14,r0
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[14]);
        }
    } else {
        // 0x8c01a0de  bra 0x8c01a12c, delay slot 0x8c01a0e0 mov #0,r0
        cpu.r[0] = 0;
    }
    // 0x8c01a12c  lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c01a12e  rts, delay slot 0x8c01a130 mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
