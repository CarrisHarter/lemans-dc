//! 0x8c026900 — recursively free a binary tree of blocks.
//!
//! `r4` is a context value threaded through the recursion and never
//! dereferenced; `r5` is the tree root. For each of the root's two children the
//! routine recursively frees the child's two children (calling itself at
//! 0x8c026900) and then frees the child through the free thunk at 0x8c01032c.
//! A null child is skipped; the root itself is freed last, unconditionally.
//! The two-level unrolling is exactly the recursion the callee performs, so the
//! port calls itself for the recursive sites.
//!
//! Both call targets are literal-pool entries: 0x8c0269c8 holds this
//! function's own address and 0x8c0269cc holds 0x8c01032c. They are read from
//! memory like the original's `mov.l`, and `pr` is set to the instruction after
//! each `jsr` delay slot.

use sh4_oracle::{Cpu, Memory};

const SELF_PTR: u32 = 0x8c0269c8;
const FREE_PTR: u32 = 0x8c0269cc;

/// `tst value,value; bt` — returns true when the branch is taken (value == 0).
#[inline]
fn tst_zero(cpu: &mut Cpu, value: u32) -> bool {
    cpu.set_t(value == 0);
    value == 0
}

/// `mov.l SELF_PTR,r0; jsr @r0` with `ret` as the return address.
#[inline]
fn call_self(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = mem.read_u32(SELF_PTR).unwrap_or(0x8c026900);
    cpu.pr = ret;
    crate::registry::p_8c026900::run(cpu, mem);
}

/// `mov.l FREE_PTR,r0; jsr @r0` with `ret` as the return address.
#[inline]
fn call_free(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(0x8c01032c);
    cpu.pr = ret;
    crate::registry::p_8c01032c::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    // mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c02690e: first child list
    let node = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[14] = node;
    if !tst_zero(cpu, node) {
        let left = mem.read_u32(node).unwrap_or(0);
        cpu.r[11] = left;
        if !tst_zero(cpu, left) {
            let sub = mem.read_u32(left).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                call_self(cpu, mem, 0x8c026926); // delay slot: nop
            }
            let sub = mem.read_u32(left.wrapping_add(4)).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c026932);
            }
            cpu.r[4] = cpu.r[11]; // delay slot
            call_free(cpu, mem, 0x8c026938);
        }
        let right = mem.read_u32(node.wrapping_add(4)).unwrap_or(0);
        cpu.r[11] = right;
        if !tst_zero(cpu, right) {
            let sub = mem.read_u32(right).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c02694a);
            }
            let sub = mem.read_u32(right.wrapping_add(4)).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c026956);
            }
            cpu.r[4] = cpu.r[11]; // delay slot
            call_free(cpu, mem, 0x8c02695c);
        }
        cpu.r[4] = cpu.r[14]; // delay slot
        call_free(cpu, mem, 0x8c026962);
    }

    // 0x8c026962: second child list
    let node = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    cpu.r[14] = node;
    if !tst_zero(cpu, node) {
        let left = mem.read_u32(node).unwrap_or(0);
        cpu.r[11] = left;
        if !tst_zero(cpu, left) {
            let sub = mem.read_u32(left).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c02697a);
            }
            let sub = mem.read_u32(left.wrapping_add(4)).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c026986);
            }
            cpu.r[4] = cpu.r[11]; // delay slot
            call_free(cpu, mem, 0x8c02698c);
        }
        let right = mem.read_u32(node.wrapping_add(4)).unwrap_or(0);
        cpu.r[11] = right;
        if !tst_zero(cpu, right) {
            let sub = mem.read_u32(right).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c02699e);
            }
            let sub = mem.read_u32(right.wrapping_add(4)).unwrap_or(0);
            cpu.r[5] = sub;
            if !tst_zero(cpu, sub) {
                cpu.r[4] = cpu.r[12]; // delay slot
                call_self(cpu, mem, 0x8c0269aa);
            }
            cpu.r[4] = cpu.r[11]; // delay slot
            call_free(cpu, mem, 0x8c0269b0);
        }
        cpu.r[4] = cpu.r[14]; // delay slot
        call_free(cpu, mem, 0x8c0269b6);
    }

    // 0x8c0269b6: free the caller's own node, always.
    cpu.r[4] = cpu.r[13]; // delay slot
    call_free(cpu, mem, 0x8c0269bc);

    // lds.l @r15+,pr ; mov.l @r15+,r11 ; mov.l @r15+,r12 ; mov.l @r15+,r13
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = v;
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = v;
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = v;
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = v;
    // rts delay: mov.l @r15+,r14
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = v;
}
