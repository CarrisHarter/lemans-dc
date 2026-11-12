//! 0x8c05cb48 — unlink a node from its old parent, then release it or graft it
//! onto a new parent, stamping depths down its child tree.
//!
//! `r4` = node, `r5` = new parent (0 = release), `r6` = append flag.
//!
//! ```text
//! push r14,r13,r12,pr
//! r3 = [r4+4]                    ; old parent
//! if r3 != 0:
//!     r3 += 12                   ; address of the old parent's first-child slot
//!     do { r0 = *r3 ; if r0 == 0 break
//!          if r0 == r4 { *r3 = [r4+8] ; [r4+8] = 0 ; break }
//!          r0 += 8 ; r3 = r0 } while (1)
//! [r4+4] = r5
//! if r5 == 0:
//!     [r4+30] = 0
//!     for child in [r4+12]:
//!         [child+30] = 1
//!         for grandchild in [child+12]:
//!             call 0x8c05caac(grandchild, 2)
//! else:
//!     if r6 == 0 { [r4+8] = [r5+12] ; [r5+12] = r4 }
//!     else if [r5+12] == 0 { [r5+12] = r4 }
//!     else { walk [r5+12] via +8 to the last node; [last+8] = r4 }
//!     r13 = [r5+30] + 1 ; [r4+30] = r13
//!     for child in [r4+12]:
//!         [child+30] = r13 + 1
//!         for grandchild in [child+12]:
//!             call 0x8c05caac(grandchild, r13 + 3)
//! pop pr,r12,r13,r14
//! ```
//!
//! The depth bump for the node's immediate children is `r13 + 1`, i.e. the new
//! node's depth plus one: `mov r13,r0 ; add #1,r0`, not an incremented r13.
//!
//! Every `tst` that gates an exit tests a register that is zero on that path,
//! so T is 1 when the routine returns. The two `jsr @r0` sites reload the
//! literal word at 0x8c05cbfc (0x8c05caac) into r0; `r6` is never written, so
//! the callee sees the caller's `r6` at both call sites.

use sh4_oracle::{Cpu, Memory};

/// Literal pool word holding the callee entry; read like the original so the
/// `r0` residue before a call matches.
const CALLEE_LITERAL: u32 = 0x8c05cbfc;
const CALLEE_ENTRY: u32 = 0x8c05caac;

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

#[inline]
fn callee_literal(mem: &Memory) -> u32 {
    mem.read_u32(CALLEE_LITERAL).unwrap_or(CALLEE_ENTRY)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05cb48  mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05cb4a  mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05cb4c  mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c05cb4e  sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c05cb50  mov.l @(4,r4),r3
    cpu.r[3] = rd(mem, cpu.r[4].wrapping_add(4));
    // 0x8c05cb52  tst r3,r3 ; 0x8c05cb54 bt 0x8c05cb72
    cpu.set_t(cpu.r[3] == 0);
    if !cpu.t() {
        // 0x8c05cb56 bra 0x8c05cb6c ; 0x8c05cb58 add #12,r3 (delay slot)
        cpu.r[3] = cpu.r[3].wrapping_add(12);
        loop {
            // 0x8c05cb6c  mov.l @r3,r0 ; 0x8c05cb6e tst r0,r0 ; 0x8c05cb70 bf 0x8c05cb5a
            cpu.r[0] = rd(mem, cpu.r[3]);
            cpu.set_t(cpu.r[0] == 0);
            if cpu.t() {
                break;
            }
            // 0x8c05cb5a  cmp/eq r4,r0 ; 0x8c05cb5c bf 0x8c05cb68
            cpu.set_t(cpu.r[0] == cpu.r[4]);
            if !cpu.t() {
                // 0x8c05cb68  add #8,r0 ; 0x8c05cb6a mov r0,r3
                cpu.r[0] = cpu.r[0].wrapping_add(8);
                cpu.r[3] = cpu.r[0];
                continue;
            }
            // 0x8c05cb5e  mov.l @(8,r4),r1
            cpu.r[1] = rd(mem, cpu.r[4].wrapping_add(8));
            // 0x8c05cb60  mov #0,r0
            cpu.r[0] = 0;
            // 0x8c05cb62  mov.l r1,@r3
            let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
            // 0x8c05cb64 bra 0x8c05cb72 ; 0x8c05cb66 mov.l r0,@(8,r4) (delay slot)
            let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
            break;
        }
    }

    // 0x8c05cb72  tst r5,r5 ; 0x8c05cb74 bt.s 0x8c05cbca
    cpu.set_t(cpu.r[5] == 0);
    let release = cpu.t();
    // 0x8c05cb76  mov.l r5,@(4,r4) (delay slot)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[5]);

    if release {
        // 0x8c05cbca  mov #0,r0 ; 0x8c05cbcc mov.l r0,@(48,r4)
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(48), cpu.r[0]);
        // 0x8c05cbce  mov.l @(12,r4),r13
        cpu.r[13] = rd(mem, cpu.r[4].wrapping_add(12));
        // 0x8c05cbd0  tst r13,r13 ; 0x8c05cbd2 bt 0x8c05cbf2
        cpu.set_t(cpu.r[13] == 0);
        if !cpu.t() {
            loop {
                // 0x8c05cbd4  mov #1,r0 ; 0x8c05cbd6 mov.l r0,@(48,r13)
                cpu.r[0] = 1;
                let _ = mem.write_u32(cpu.r[13].wrapping_add(48), cpu.r[0]);
                // 0x8c05cbd8  mov.l @(12,r13),r12
                cpu.r[12] = rd(mem, cpu.r[13].wrapping_add(12));
                // 0x8c05cbda  tst r12,r12 ; 0x8c05cbdc bt 0x8c05cbec
                cpu.set_t(cpu.r[12] == 0);
                if !cpu.t() {
                    loop {
                        // 0x8c05cbde  mov.l 0x8c05cbfc,r0
                        cpu.r[0] = callee_literal(mem);
                        // 0x8c05cbe0  mov r12,r4
                        cpu.r[4] = cpu.r[12];
                        // 0x8c05cbe2  jsr @r0 ; 0x8c05cbe4 mov #2,r5 (delay slot)
                        cpu.pr = 0x8c05cbe6;
                        cpu.r[5] = 2;
                        crate::registry::p_8c05caac::run(cpu, mem);
                        // 0x8c05cbe6  mov.l @(8,r12),r12
                        cpu.r[12] = rd(mem, cpu.r[12].wrapping_add(8));
                        // 0x8c05cbe8  tst r12,r12 ; 0x8c05cbea bf 0x8c05cbde
                        cpu.set_t(cpu.r[12] == 0);
                        if cpu.t() {
                            break;
                        }
                    }
                }
                // 0x8c05cbec  mov.l @(8,r13),r13
                cpu.r[13] = rd(mem, cpu.r[13].wrapping_add(8));
                // 0x8c05cbee  tst r13,r13 ; 0x8c05cbf0 bf 0x8c05cbd4
                cpu.set_t(cpu.r[13] == 0);
                if cpu.t() {
                    break;
                }
            }
        }
    } else {
        // 0x8c05cb78  tst r6,r6 ; 0x8c05cb7a bf 0x8c05cb84
        cpu.set_t(cpu.r[6] == 0);
        if cpu.t() {
            // 0x8c05cb7c  mov.l @(12,r5),r0 ; 0x8c05cb7e mov.l r0,@(8,r4)
            cpu.r[0] = rd(mem, cpu.r[5].wrapping_add(12));
            let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
            // 0x8c05cb80 bra 0x8c05cb98 ; 0x8c05cb82 mov.l r4,@(12,r5) (delay slot)
            let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[4]);
        } else {
            // 0x8c05cb84  mov.l @(12,r5),r0
            cpu.r[0] = rd(mem, cpu.r[5].wrapping_add(12));
            // 0x8c05cb86  tst r0,r0 ; 0x8c05cb88 bf 0x8c05cb90
            cpu.set_t(cpu.r[0] == 0);
            if !cpu.t() {
                loop {
                    // 0x8c05cb90  mov.l @(8,r0),r1 ; 0x8c05cb92 tst r1,r1 ; 0x8c05cb94 bf 0x8c05cb8e
                    cpu.r[1] = rd(mem, cpu.r[0].wrapping_add(8));
                    cpu.set_t(cpu.r[1] == 0);
                    if cpu.t() {
                        break;
                    }
                    // 0x8c05cb8e  mov r1,r0
                    cpu.r[0] = cpu.r[1];
                }
                // 0x8c05cb96  mov.l r4,@(8,r0)
                let _ = mem.write_u32(cpu.r[0].wrapping_add(8), cpu.r[4]);
            } else {
                // 0x8c05cb8a bra 0x8c05cb98 ; 0x8c05cb8c mov.l r4,@(12,r5) (delay slot)
                let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[4]);
            }
        }

        // 0x8c05cb98  mov.l @(48,r5),r13 ; 0x8c05cb9a add #1,r13
        cpu.r[13] = rd(mem, cpu.r[5].wrapping_add(48)).wrapping_add(1);
        // 0x8c05cb9c  mov.l r13,@(48,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(48), cpu.r[13]);
        // 0x8c05cb9e  mov.l @(12,r4),r14
        cpu.r[14] = rd(mem, cpu.r[4].wrapping_add(12));
        // 0x8c05cba0  tst r14,r14 ; 0x8c05cba2 bt 0x8c05cbf2
        cpu.set_t(cpu.r[14] == 0);
        if !cpu.t() {
            loop {
                // 0x8c05cba4  mov r13,r0 ; 0x8c05cba6 add #1,r0
                cpu.r[0] = cpu.r[13].wrapping_add(1);
                // 0x8c05cba8  mov.l r0,@(48,r14)
                let _ = mem.write_u32(cpu.r[14].wrapping_add(48), cpu.r[0]);
                // 0x8c05cbaa  mov.l @(12,r14),r12
                cpu.r[12] = rd(mem, cpu.r[14].wrapping_add(12));
                // 0x8c05cbac  tst r12,r12 ; 0x8c05cbae bt 0x8c05cbc0
                cpu.set_t(cpu.r[12] == 0);
                if !cpu.t() {
                    loop {
                        // 0x8c05cbb0  mov.l 0x8c05cbfc,r0
                        cpu.r[0] = callee_literal(mem);
                        // 0x8c05cbb2  mov r13,r5 ; 0x8c05cbb4 mov r12,r4
                        cpu.r[5] = cpu.r[13];
                        cpu.r[4] = cpu.r[12];
                        // 0x8c05cbb6  jsr @r0 ; 0x8c05cbb8 add #2,r5 (delay slot)
                        cpu.pr = 0x8c05cbba;
                        cpu.r[5] = cpu.r[5].wrapping_add(2);
                        crate::registry::p_8c05caac::run(cpu, mem);
                        // 0x8c05cbba  mov.l @(8,r12),r12
                        cpu.r[12] = rd(mem, cpu.r[12].wrapping_add(8));
                        // 0x8c05cbbc  tst r12,r12 ; 0x8c05cbbe bf 0x8c05cbb0
                        cpu.set_t(cpu.r[12] == 0);
                        if cpu.t() {
                            break;
                        }
                    }
                }
                // 0x8c05cbc0  mov.l @(8,r14),r14
                cpu.r[14] = rd(mem, cpu.r[14].wrapping_add(8));
                // 0x8c05cbc2  tst r14,r14 ; 0x8c05cbc4 bf 0x8c05cba4
                cpu.set_t(cpu.r[14] == 0);
                if cpu.t() {
                    break;
                }
            }
        }
    }

    // 0x8c05cbf2  lds.l @r15+,pr
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05cbf4  mov.l @r15+,r12
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05cbf6  mov.l @r15+,r13
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05cbf8  rts ; 0x8c05cbfa mov.l @r15+,r14 (delay slot)
    cpu.r[14] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
