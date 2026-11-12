//! 0x8c05dfc0 — binary-tree search/insertion-point lookup.
//!
//! `r4` (`param_1`) points at a tree header whose `+4` field is the root node.
//! `r5` (`param_2`) points at a two-level key handle: `[[param_2]]` is the key
//! string passed as `strcmp`'s first operand. Node layout: `+0` left, `+4`
//! right, `+12` key-pointer (the key string is `[[node+12]]`). The result slot
//! is the word at the caller's stack pointer (`@(20,r15)` after the 20-byte
//! prologue), receiving `param_1+4` or the last right-move node.
//!
//! ```text
//! push r14,r13,r12,r11,pr ; r12 = r5 ; r11 = r4 ; r13 = [r11+4] ; r14 = 0
//! while (r13) {
//!     t = strcmp([[*param_2]], [[r13+12]])       ; jsr 0x8c05e034
//!     if (t < 0) r13 = [r13]                     ; left, keep r14
//!     else       { r14 = r13 ; r13 = [r13+4] }   ; right, remember node
//! }
//! if (r14 && strcmp([[r14+12]], [[*param_2]]) >= 0) *out = r14
//! else { *out = param_1 + 4 }                    ; payload is r11 = r4+4
//! pop pr,r11,r12,r13,r14
//! ```
//!
//! The second `strcmp` swaps operands: node key first, query key second, and
//! only its sign matters. T ends 1 except on the second-call-negative arm,
//! where the `tst r0,r0` before the taken `bt` leaves 0. The calls pass the
//! callee address from the code literal at 0x8c05e030.
//!
//! Register residue: r0 ends as the output-slot pointer, r1 as 0 after any
//! call (entry value otherwise), r3-r5 are the callee's residue, r11-r14 and
//! pr are restored from the stack to their entry values.

use sh4_oracle::{Cpu, Memory};

/// Word at 0x8c05e030, the `mov.l` literal that names the callee.
const CALLEE_LITERAL: u32 = 0x8c05e030;
const CALLEE: u32 = 0x8c05e034;

/// `cmp/gt r0,r1 ; movt r0 ; extu.b r0,r0 ; tst r0,r0` on a signed return.
/// Returns true when the `bt` is taken (return >= 0) and leaves r0/T as the
/// original four instructions do.
fn test_nonnegative(cpu: &mut Cpu) -> bool {
    let negative = (cpu.r[0] as i32) < 0; // cmp/gt r0,r1 with r1 = 0
    cpu.set_t(negative);
    cpu.r[0] = u32::from(negative); // movt r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[0] == 0
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c05dfc0: mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 8c05dfc6: mov.l r11,@-r15 ; mov r5,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    // 8c05dfca: sts.l pr,@-r15 ; mov r4,r11 ; mov.l @(4,r11),r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[11] = cpu.r[4];
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
    // 8c05dfd0: tst r13,r13 ; bt.s 0x8c05dffa
    cpu.set_t(cpu.r[13] == 0);
    cpu.r[14] = 0; // delay slot: mov #0,r14
    while cpu.r[13] != 0 {
        // 8c05dfd6: mov.l @(12,r13),r3 ; mov.l @r12,r1 ; mov.l @0x8c05e030,r0
        cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(CALLEE_LITERAL).unwrap_or(CALLEE);
        // 8c05dfdc: mov.l @r3,r5 ; jsr @r0
        cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.pr = 0x8c05dfe2;
        // delay slot: mov.l @r1,r4
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        crate::registry::p_8c05e034::run(cpu, mem);
        // 8c05dfe2: mov #0,r1 ; cmp/gt r0,r1 ; movt r0 ; extu.b ; tst
        cpu.r[1] = 0;
        if test_nonnegative(cpu) {
            // bt 0x8c05dff2: mov r13,r14 ; mov.l @(4,r13),r13
            cpu.r[14] = cpu.r[13];
            cpu.r[13] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        } else {
            // bra 0x8c05dff6 ; delay slot: mov.l @r13,r13
            cpu.r[13] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        }
        // 8c05dff6: tst r13,r13 ; bf 0x8c05dfd6
        cpu.set_t(cpu.r[13] == 0);
    }

    // 8c05dffa: tst r14,r14 ; bt 0x8c05e016
    cpu.set_t(cpu.r[14] == 0);
    if cpu.r[14] != 0 {
        // 8c05dffe: mov.l @r12,r3 ; mov.l @(12,r14),r1 ; mov.l @0x8c05e030,r0
        cpu.r[3] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(CALLEE_LITERAL).unwrap_or(CALLEE);
        // 8c05e004: mov.l @r3,r5 ; jsr @r0
        cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.pr = 0x8c05e00a;
        // delay slot: mov.l @r1,r4
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        crate::registry::p_8c05e034::run(cpu, mem);
        // 8c05e00a: mov #0,r1 ; cmp/gt r0,r1 ; movt r0 ; extu.b ; tst
        cpu.r[1] = 0;
        if test_nonnegative(cpu) {
            // bt 0x8c05e01e: mov.l @(20,r15),r0 ; mov.l r14,@r0
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[0], cpu.r[14]);
        } else {
            // 8c05e016: mov.l @(20,r15),r0 ; add #4,r11
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
            cpu.r[11] = cpu.r[11].wrapping_add(4);
            let _ = mem.write_u32(cpu.r[0], cpu.r[11]); // bra delay slot
        }
    } else {
        // 8c05e016: mov.l @(20,r15),r0 ; add #4,r11 ; bra ; delay: mov.l r11,@r0
        cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
        cpu.r[11] = cpu.r[11].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[11]);
    }

    // 8c05e022: lds.l @r15+,pr ; mov.l @r15+,r11 ; mov.l @r15+,r12 ;
    //           mov.l @r15+,r13 ; rts ; delay: mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
