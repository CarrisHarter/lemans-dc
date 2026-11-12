//! 0x8c05caac — stamp sequential depths onto a three-level child tree.
//!
//! Called with `r4` = node and `r5` = that node's depth. The routine writes
//! `depth` to `[node+0x30]`, then walks the node's child list at `[node+0x0c]`
//! (linked through `+0x08`) writing `depth+1`, each child's child list writing
//! `depth+2`, and each grandchild's child list by recursing with `depth+3`:
//!
//! ```text
//! push r14,r13 ; r14 = r5 ; push r12,r11,pr
//! [r4+0x30] = r14
//! r13 = [r4+0x0c] ; tst ; bt exit
//! outer:
//!   r0 = r14 + 1 ; [r13+0x30] = r0
//!   r12 = [r13+0x0c] ; tst ; bt next_outer
//!   middle:
//!     r0 = r14 + 2 ; [r12+0x30] = r0
//!     r11 = [r12+0x0c] ; tst ; bt next_middle
//!     inner:
//!       r0 = [0x8c05cb00]        (== 0x8c05caac, this routine)
//!       r5 = r14 ; r4 = r11 ; jsr @r0 ; r5 += 3 (delay slot)
//!       r11 = [r11+0x08] ; tst ; bf inner
//!   next_middle: r12 = [r12+0x08] ; tst ; bf middle
//! next_outer: r13 = [r13+0x08] ; tst ; bf outer
//! exit: lds.l pr ; pop r11,r12,r13,rts ; pop r14 (delay slot)
//! ```
//!
//! The `jsr` target is the routine itself (literal pool word at 0x8c05cb00),
//! so the third level is a real self-call: each frame saves/restores
//! r11..r14 and pr, and the caller resumes with the callee's `r0`/`r4`/`r5`
//! residue. Those residues are load-bearing:
//!
//! * `r0` is only rewritten after a `tst` has shown a child exists, so a frame
//!   that exits at 0x8c05cabe (or a callee that does) leaves the caller's
//!   loaded literal 0x8c05caac in place.
//! * `r4` is only written by the `jsr` setup, so with no recursion it still
//!   holds the argument node.
//! * `r5` is written by `mov r14,r5` plus the delay slot's `add #3,r5` at each
//!   call site, and is caller-saved inside the callee; the final value is
//!   inherited from the last frame that ran a `jsr`, or the argument depth if
//!   none did.
//!
//! Every exit is reached through a `tst reg,reg` whose register is zero, or
//! through the taken `bt` of such a `tst`; T is therefore 1 on every exit.
//! Only the stack (filtered by the verifier) is written below r15.

use sh4_oracle::{Cpu, Memory};

/// Literal pool word at 0x8c05cb00; the disassembler resolves it to this entry.
const SELF_LITERAL: u32 = 0x8c05cb00;
const SELF_ENTRY: u32 = 0x8c05caac;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05caac  mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05caae  mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05cab0  mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05cab2  mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c05cab4  mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c05cab6  sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c05cab8  mov.l r14,@(48,r4)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(48), cpu.r[14]);
    // 0x8c05caba  mov.l @(12,r4),r13
    cpu.r[13] = rd(mem, cpu.r[4].wrapping_add(12), 0);
    // 0x8c05cabc  tst r13,r13 ; 0x8c05cabe bt 0x8c05caf4
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        'outer: loop {
            // 0x8c05cac0  mov r14,r0 ; 0x8c05cac2 add #1,r0
            cpu.r[0] = cpu.r[14].wrapping_add(1);
            // 0x8c05cac4  mov.l r0,@(48,r13)
            let _ = mem.write_u32(cpu.r[13].wrapping_add(48), cpu.r[0]);
            // 0x8c05cac6  mov.l @(12,r13),r12
            cpu.r[12] = rd(mem, cpu.r[13].wrapping_add(12), 0);
            // 0x8c05cac8  tst r12,r12 ; 0x8c05caca bt 0x8c05caee
            cpu.set_t(cpu.r[12] == 0);
            if !cpu.t() {
                'middle: loop {
                    // 0x8c05cacc  mov r14,r0 ; 0x8c05cace add #2,r0
                    cpu.r[0] = cpu.r[14].wrapping_add(2);
                    // 0x8c05cad0  mov.l r0,@(48,r12)
                    let _ = mem.write_u32(cpu.r[12].wrapping_add(48), cpu.r[0]);
                    // 0x8c05cad2  mov.l @(12,r12),r11
                    cpu.r[11] = rd(mem, cpu.r[12].wrapping_add(12), 0);
                    // 0x8c05cad4  tst r11,r11 ; 0x8c05cad6 bt 0x8c05cae8
                    cpu.set_t(cpu.r[11] == 0);
                    if !cpu.t() {
                        loop {
                            // 0x8c05cad8  mov.l 0x8c05cb00,r0
                            cpu.r[0] = rd(mem, SELF_LITERAL, SELF_ENTRY);
                            // 0x8c05cada  mov r14,r5 ; 0x8c05cadc mov r11,r4
                            cpu.r[5] = cpu.r[14];
                            cpu.r[4] = cpu.r[11];
                            // 0x8c05cade  jsr @r0 ; 0x8c05cae0 add #3,r5 (delay)
                            cpu.pr = 0x8c05cae2;
                            cpu.r[5] = cpu.r[5].wrapping_add(3);
                            crate::registry::p_8c05caac::run(cpu, mem);
                            // 0x8c05cae2  mov.l @(8,r11),r11
                            cpu.r[11] = rd(mem, cpu.r[11].wrapping_add(8), 0);
                            // 0x8c05cae4  tst r11,r11 ; 0x8c05cae6 bf 0x8c05cad8
                            cpu.set_t(cpu.r[11] == 0);
                            if cpu.t() {
                                break;
                            }
                        }
                    }
                    // 0x8c05cae8  mov.l @(8,r12),r12
                    cpu.r[12] = rd(mem, cpu.r[12].wrapping_add(8), 0);
                    // 0x8c05caea  tst r12,r12 ; 0x8c05caec bf 0x8c05cacc
                    cpu.set_t(cpu.r[12] == 0);
                    if cpu.t() {
                        break 'middle;
                    }
                }
            }
            // 0x8c05caee  mov.l @(8,r13),r13
            cpu.r[13] = rd(mem, cpu.r[13].wrapping_add(8), 0);
            // 0x8c05caf0  tst r13,r13 ; 0x8c05caf2 bf 0x8c05cac0
            cpu.set_t(cpu.r[13] == 0);
            if cpu.t() {
                break 'outer;
            }
        }
    }
    // 0x8c05caf4  lds.l @r15+,pr
    cpu.pr = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05caf6  mov.l @r15+,r11
    cpu.r[11] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05caf8  mov.l @r15+,r12
    cpu.r[12] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05cafa  mov.l @r15+,r13
    cpu.r[13] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05cafc  rts ; 0x8c05cafe mov.l @r15+,r14 (delay slot)
    cpu.r[14] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
