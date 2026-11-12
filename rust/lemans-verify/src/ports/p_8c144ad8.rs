//! 0x8c144ad8 — recursively destroy a two-slot container.
//!
//! ```text
//! 8c144ad8  mov.l r14,@-r15
//! 8c144ada  mov.l r13,@-r15
//! 8c144adc  mov.l r12,@-r15
//! 8c144ade  mov   r5,r13        r13 = param_2 (the container)
//! 8c144ae0  mov.l r11,@-r15
//! 8c144ae2  mov   r4,r12        r12 = param_1 (context, passed to every call)
//! 8c144ae4  sts.l pr,@-r15
//! 8c144ae6  mov.l @r13,r14      r14 = container->slot[0]
//! 8c144ae8  tst   r14,r14
//! 8c144aea  bt    0x8c144b3a    NULL: skip the whole first half
//! 8c144aec  mov.l @r14,r11      r11 = slot0->holder
//! 8c144aee  tst   r11,r11
//! 8c144af0  bt    0x8c144b10    NULL: skip the two grandchildren
//! 8c144af2  mov.l @r11,r5       r5 = holder->child[0]
//! 8c144af4  tst   r5,r5
//! 8c144af6  bt    0x8c144afe    NULL: skip the recursion
//! 8c144af8  mov.l 0x8c144ba0,r0 r0 = 0x8c144ad8 (self)
//! 8c144afa  jsr   @r0
//! 8c144afc  nop                 delay: r4 is still the context
//! 8c144afe  mov.l @(4,r11),r5   r5 = holder->child[1]
//! 8c144b00  tst   r5,r5
//! 8c144b02  bt    0x8c144b0a    NULL: skip the recursion
//! 8c144b04  mov.l 0x8c144ba0,r0
//! 8c144b06  jsr   @r0
//! 8c144b08  mov   r12,r4        delay: recurse(context, child[1])
//! 8c144b0a  mov.l 0x8c144ba4,r0 r0 = 0x8c01032c (free thunk)
//! 8c144b0c  jsr   @r0
//! 8c144b0e  mov   r11,r4        delay: free(holder)
//! 8c144b10  mov.l @(4,r14),r11  r11 = slot0->holder[1]
//! 8c144b12  tst   r11,r11
//! 8c144b14  bt    0x8c144b34
//! 8c144b16  ...                same two-grandchild block through 0x8c144b32
//! 8c144b34  mov.l 0x8c144ba4,r0
//! 8c144b36  jsr   @r0
//! 8c144b38  mov   r14,r4        delay: free(slot0)
//! 8c144b3a  mov.l @(4,r13),r14  r14 = container->slot[1]
//! 8c144b3c  ...                identical half, call sites 0x8c144b4c..0x8c144b92
//! 8c144b8e  mov.l 0x8c144ba4,r0
//! 8c144b90  jsr   @r0
//! 8c144b92  mov   r13,r4        delay: free(container)
//! 8c144b94  lds.l @r15+,pr
//! 8c144b96  mov.l @r15+,r11
//! 8c144b98  mov.l @r15+,r12
//! 8c144b9a  mov.l @r15+,r13
//! 8c144b9c  rts
//! 8c144b9e  mov.l @r15+,r14     delay
//! ```
//!
//! The pool word at 0x8c144ba0 is the routine's own address, so the four `jsr`
//! recursion sites are direct self-calls through this registry entry. The word
//! at 0x8c144ba4 is the free thunk 0x8c01032c; every node reached through a
//! slot is freed after its two children, and the container itself is freed on
//! every path (the NULL first-slot exit lands on the same final call).
//!
//! Registers r11..r14 and pr only survive through the pushed frame; r0..r7 and
//! T are the free implementation's residue, and the routine's own `tst`s set T
//! nowhere near the exit. `param_1` is dead apart from being forwarded as the
//! first argument of every recursive call, and the first recursion site's delay
//! slot is a `nop`, so it forwards the entry value of r4 unchanged.

use sh4_oracle::{Cpu, Memory};

/// `mov.l 0x8c144ba0,r0` — the routine's own address.
const SELF_LITERAL: u32 = 0x8c144ba0;
/// `mov.l 0x8c144ba4,r0` — 0x8c01032c, the free thunk.
const FREE_THUNK_LITERAL: u32 = 0x8c144ba4;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c144ad8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c144ada: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c144adc: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c144ade: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c144ae0: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c144ae2: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 0x8c144ae4: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c144ae6: mov.l @r13,r14
    cpu.r[14] = rd(mem, cpu.r[13], 0);
    // 0x8c144ae8: tst r14,r14 ; 0x8c144aea: bt 0x8c144b3a
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        // 0x8c144aec: mov.l @r14,r11
        cpu.r[11] = rd(mem, cpu.r[14], 0);
        // 0x8c144aee: tst r11,r11 ; 0x8c144af0: bt 0x8c144b10
        cpu.set_t(cpu.r[11] == 0);
        if !cpu.t() {
            // 0x8c144af2: mov.l @r11,r5
            cpu.r[5] = rd(mem, cpu.r[11], 0);
            // 0x8c144af4: tst r5,r5 ; 0x8c144af6: bt 0x8c144afe
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144af8: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144afa: jsr @r0 ; delay 0x8c144afc: nop (r4 = context,
                // r5 = child[0], both still in place)
                cpu.pr = 0x8c144afe;
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144afe: mov.l @(4,r11),r5
            cpu.r[5] = rd(mem, cpu.r[11].wrapping_add(4), 0);
            // 0x8c144b00: tst r5,r5 ; 0x8c144b02: bt 0x8c144b0a
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b04: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b06: jsr @r0 ; delay 0x8c144b08: mov r12,r4
                cpu.pr = 0x8c144b0a;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b0a: mov.l 0x8c144ba4,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c144b0c: jsr @r0 ; delay 0x8c144b0e: mov r11,r4
            cpu.pr = 0x8c144b10;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c144b10: mov.l @(4,r14),r11
        cpu.r[11] = rd(mem, cpu.r[14].wrapping_add(4), 0);
        // 0x8c144b12: tst r11,r11 ; 0x8c144b14: bt 0x8c144b34
        cpu.set_t(cpu.r[11] == 0);
        if !cpu.t() {
            // 0x8c144b16: mov.l @r11,r5
            cpu.r[5] = rd(mem, cpu.r[11], 0);
            // 0x8c144b18: tst r5,r5 ; 0x8c144b1a: bt 0x8c144b22
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b1c: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b1e: jsr @r0 ; delay 0x8c144b20: mov r12,r4
                cpu.pr = 0x8c144b22;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b22: mov.l @(4,r11),r5
            cpu.r[5] = rd(mem, cpu.r[11].wrapping_add(4), 0);
            // 0x8c144b24: tst r5,r5 ; 0x8c144b26: bt 0x8c144b2e
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b28: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b2a: jsr @r0 ; delay 0x8c144b2c: mov r12,r4
                cpu.pr = 0x8c144b2e;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b2e: mov.l 0x8c144ba4,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c144b30: jsr @r0 ; delay 0x8c144b32: mov r11,r4
            cpu.pr = 0x8c144b34;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c144b34: mov.l 0x8c144ba4,r0
        cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
        // 0x8c144b36: jsr @r0 ; delay 0x8c144b38: mov r14,r4
        cpu.pr = 0x8c144b3a;
        cpu.r[4] = cpu.r[14];
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c144b3a: mov.l @(4,r13),r14
    cpu.r[14] = rd(mem, cpu.r[13].wrapping_add(4), 0);
    // 0x8c144b3c: tst r14,r14 ; 0x8c144b3e: bt 0x8c144b8e
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        // 0x8c144b40: mov.l @r14,r11
        cpu.r[11] = rd(mem, cpu.r[14], 0);
        // 0x8c144b42: tst r11,r11 ; 0x8c144b44: bt 0x8c144b64
        cpu.set_t(cpu.r[11] == 0);
        if !cpu.t() {
            // 0x8c144b46: mov.l @r11,r5
            cpu.r[5] = rd(mem, cpu.r[11], 0);
            // 0x8c144b48: tst r5,r5 ; 0x8c144b4a: bt 0x8c144b52
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b4c: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b4e: jsr @r0 ; delay 0x8c144b50: mov r12,r4
                cpu.pr = 0x8c144b52;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b52: mov.l @(4,r11),r5
            cpu.r[5] = rd(mem, cpu.r[11].wrapping_add(4), 0);
            // 0x8c144b54: tst r5,r5 ; 0x8c144b56: bt 0x8c144b5e
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b58: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b5a: jsr @r0 ; delay 0x8c144b5c: mov r12,r4
                cpu.pr = 0x8c144b5e;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b5e: mov.l 0x8c144ba4,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c144b60: jsr @r0 ; delay 0x8c144b62: mov r11,r4
            cpu.pr = 0x8c144b64;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c144b64: mov.l @(4,r14),r11
        cpu.r[11] = rd(mem, cpu.r[14].wrapping_add(4), 0);
        // 0x8c144b66: tst r11,r11 ; 0x8c144b68: bt 0x8c144b88
        cpu.set_t(cpu.r[11] == 0);
        if !cpu.t() {
            // 0x8c144b6a: mov.l @r11,r5
            cpu.r[5] = rd(mem, cpu.r[11], 0);
            // 0x8c144b6c: tst r5,r5 ; 0x8c144b6e: bt 0x8c144b76
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b70: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b72: jsr @r0 ; delay 0x8c144b74: mov r12,r4
                cpu.pr = 0x8c144b76;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b76: mov.l @(4,r11),r5
            cpu.r[5] = rd(mem, cpu.r[11].wrapping_add(4), 0);
            // 0x8c144b78: tst r5,r5 ; 0x8c144b7a: bt 0x8c144b82
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                // 0x8c144b7c: mov.l 0x8c144ba0,r0
                cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c144ad8);
                // 0x8c144b7e: jsr @r0 ; delay 0x8c144b80: mov r12,r4
                cpu.pr = 0x8c144b82;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c144ad8::run(cpu, mem);
            }
            // 0x8c144b82: mov.l 0x8c144ba4,r0
            cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
            // 0x8c144b84: jsr @r0 ; delay 0x8c144b86: mov r11,r4
            cpu.pr = 0x8c144b88;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c144b88: mov.l 0x8c144ba4,r0
        cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
        // 0x8c144b8a: jsr @r0 ; delay 0x8c144b8c: mov r14,r4
        cpu.pr = 0x8c144b8e;
        cpu.r[4] = cpu.r[14];
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c144b8e: mov.l 0x8c144ba4,r0
    cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
    // 0x8c144b90: jsr @r0 ; delay 0x8c144b92: mov r13,r4
    cpu.pr = 0x8c144b94;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c01032c::run(cpu, mem);
    // 0x8c144b94: lds.l @r15+,pr
    cpu.pr = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c144b96: mov.l @r15+,r11
    cpu.r[11] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c144b98: mov.l @r15+,r12
    cpu.r[12] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c144b9a: mov.l @r15+,r13
    cpu.r[13] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c144b9c: rts ; delay 0x8c144b9e: mov.l @r15+,r14
    cpu.r[14] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
