//! 0x8c021064 — free a node and every node in its three-level child lists,
//! recursing into the third level.
//!
//! Node layout used here: `+0x08` = next sibling, `+0x0c` = child list head,
//! `+0x18` = block handed to free(). Every visited node has its `+0x18` word
//! freed (via the 0x8c01032c thunk to 0x8c159fc0) and then zeroed.
//!
//! ```text
//! push r14,r13,r12,pr
//! r12 = r4
//! free([r12+0x18]) ; [r12+0x18] = 0
//! r14 = [r12+0x0c]
//! bt  exit                       if r14 == 0
//! outer:
//!   free([r14+0x18]) ; [r14+0x18] = 0
//!   r13 = [r14+0x0c]
//!   bt  next_outer               if r13 == 0
//!   middle:
//!     free([r13+0x18]) ; [r13+0x18] = 0
//!     r12 = [r13+0x0c]
//!     bt  next_middle            if r12 == 0
//!     inner:
//!       free(r12)                self-recursion (jsr to 0x8c021064)
//!       r12 = [r12+0x08]
//!       bf  inner                while r12 != 0
//!   next_middle: r13 = [r13+0x08] ; bf middle while r13 != 0
//! next_outer: r14 = [r14+0x08] ; bf outer while r14 != 0
//! exit: lds.l pr ; pop r12,r13 ; rts ; pop r14 (delay slot)
//! ```
//!
//! All three `jsr` sites take their delay slot from the node register: the
//! free calls pass `[node+0x18]`, the self-call passes the node itself. The
//! thunk overwrites r0 with the free entry point and tail-calls it; after
//! every free return the routine sets r0 = 0 and stores that zero.
//! r12/r13/r14 and pr are restored from the frame, so only r0..r7 carry the
//! last callee's residue to the caller. T is 1 on every exit: the epilogue is
//! reachable only through a `tst` that compared a register with itself and
//! set T (taken `bt`, or fall-through of a not-taken `bf`).

use sh4_oracle::{Cpu, Memory};

/// Literal pool: 0x8c01032c (free thunk) at 0x8c0210c0, 0x8c021064 (self) at
/// 0x8c0210c4.
const FREE_LITERAL: u32 = 0x8c0210c0;
const SELF_LITERAL: u32 = 0x8c0210c4;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

/// `mov.l FREE_LITERAL,r0 ; jsr @r0 ; mov.l @(24,node),r4`, then
/// `mov #0,r0 ; mov.l r0,@(24,node)`.
#[inline]
fn free_payload(cpu: &mut Cpu, mem: &mut Memory, node: u32, return_addr: u32) {
    cpu.r[0] = rd(mem, FREE_LITERAL, 0x8c01032c);
    cpu.pr = return_addr;
    cpu.r[4] = rd(mem, node.wrapping_add(24), 0);
    crate::registry::p_8c01032c::run(cpu, mem);
    cpu.r[0] = 0;
    let _ = mem.write_u32(node.wrapping_add(24), 0);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c021064: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c021066: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c021068: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c02106a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c02106c: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 0x8c02106e..0x8c021074: free([r12+0x18]); [r12+0x18] = 0
    free_payload(cpu, mem, cpu.r[12], 0x8c021074);
    // 0x8c021078: mov.l @(12,r12),r14
    cpu.r[14] = rd(mem, cpu.r[12].wrapping_add(12), 0);
    // 0x8c02107a: tst r14,r14 ; 0x8c02107c: bt 0x8c0210b6
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        'outer: loop {
            // 0x8c02107e..0x8c021086: free([r14+0x18]); [r14+0x18] = 0
            free_payload(cpu, mem, cpu.r[14], 0x8c021082);
            // 0x8c021088: mov.l @(12,r14),r13
            cpu.r[13] = rd(mem, cpu.r[14].wrapping_add(12), 0);
            // 0x8c02108a: tst r13,r13 ; 0x8c02108c: bt 0x8c0210b0
            cpu.set_t(cpu.r[13] == 0);
            if !cpu.t() {
                'middle: loop {
                    // 0x8c02108e..0x8c021096: free([r13+0x18]); [r13+0x18] = 0
                    free_payload(cpu, mem, cpu.r[13], 0x8c021092);
                    // 0x8c021098: mov.l @(12,r13),r12
                    cpu.r[12] = rd(mem, cpu.r[13].wrapping_add(12), 0);
                    // 0x8c02109a: tst r12,r12 ; 0x8c02109c: bt 0x8c0210aa
                    cpu.set_t(cpu.r[12] == 0);
                    if !cpu.t() {
                        loop {
                            // 0x8c02109e: mov.l 0x8c0210c4,r0
                            cpu.r[0] = rd(mem, SELF_LITERAL, 0x8c021064);
                            // 0x8c0210a0: jsr @r0 ; 0x8c0210a2: mov r12,r4 (delay)
                            cpu.pr = 0x8c0210a4;
                            cpu.r[4] = cpu.r[12];
                            crate::registry::p_8c021064::run(cpu, mem);
                            // 0x8c0210a4: mov.l @(8,r12),r12
                            cpu.r[12] = rd(mem, cpu.r[12].wrapping_add(8), 0);
                            // 0x8c0210a6: tst r12,r12 ; 0x8c0210a8: bf 0x8c02109e
                            cpu.set_t(cpu.r[12] == 0);
                            if cpu.t() {
                                break;
                            }
                        }
                    }
                    // 0x8c0210aa: mov.l @(8,r13),r13
                    cpu.r[13] = rd(mem, cpu.r[13].wrapping_add(8), 0);
                    // 0x8c0210ac: tst r13,r13 ; 0x8c0210ae: bf 0x8c02108e
                    cpu.set_t(cpu.r[13] == 0);
                    if cpu.t() {
                        break 'middle;
                    }
                }
            }
            // 0x8c0210b0: mov.l @(8,r14),r14
            cpu.r[14] = rd(mem, cpu.r[14].wrapping_add(8), 0);
            // 0x8c0210b2: tst r14,r14 ; 0x8c0210b4: bf 0x8c02107e
            cpu.set_t(cpu.r[14] == 0);
            if cpu.t() {
                break 'outer;
            }
        }
    }
    // 0x8c0210b6: lds.l @r15+,pr
    cpu.pr = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0210b8: mov.l @r15+,r12
    cpu.r[12] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0210ba: mov.l @r15+,r13
    cpu.r[13] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0210bc: rts ; 0x8c0210be: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
