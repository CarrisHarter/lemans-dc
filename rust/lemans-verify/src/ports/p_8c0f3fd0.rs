//! 0x8c0f3fd0 — empty a circular list, freeing every node after its sentinel.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
//! mov   r4,r13                 ; r13 = list argument
//! mov.l @r13,r0                ; r0 = *arg = sentinel
//! bra   0x8c0f3fe6
//! mov.l @r0,r14                ; delay slot: r14 = sentinel->next
//! 0x8c0f3fde: mov r14,r4
//!             mov.l 0x8c0f3ffc,r0 ; r0 = 0x8c01032c (free thunk)
//!             jsr @r0
//!             mov.l @r14,r14      ; delay slot: next = node->next, read BEFORE free
//! 0x8c0f3fe6: mov.l @r13,r0 ; cmp/eq r0,r14 ; bf 0x8c0f3fde
//! 0x8c0f3fec: mov.l r0,@r0          ; sentinel->next = sentinel
//!             mov.l @r13,r0
//!             mov.l r0,@(4,r0)      ; sentinel->prev = sentinel
//! lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! `*arg` is the sentinel node itself, so the walk starts at `sentinel->next`
//! and stops when a successor is the sentinel, leaving the sentinel allocated.
//! The successor load sits in the `jsr` delay slot: it happens before `free`
//! runs, so the port reads it before delegating to the callee. `r0` is always
//! reloaded from `*arg` after the loop, so the exit value is the sentinel
//! pointer; `T` is set by the loop's last `cmp/eq`, which is always an equal
//! comparison on the exit path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0f3fd0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0f3fd2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0f3fd4: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[13] = cpu.r[4]; // 0x8c0f3fd6: mov r4,r13
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // 0x8c0f3fd8: mov.l @r13,r0
    // 0x8c0f3fda bra 0x8c0f3fe6 / 0x8c0f3fdc delay: mov.l @r0,r14
    cpu.r[14] = mem.read_u32(cpu.r[0]).unwrap_or(0);

    loop {
        // 0x8c0f3fe6: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        let at_sentinel = cpu.r[14] == cpu.r[0]; // 0x8c0f3fe8: cmp/eq r0,r14
        cpu.set_t(at_sentinel);
        if at_sentinel {
            break; // 0x8c0f3fea: bf 0x8c0f3fde not taken
        }
        // 0x8c0f3fde: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c0f3fe0: mov.l 0x8c0f3ffc,r0
        cpu.r[0] = mem.read_u32(0x8c0f3ffc).unwrap_or(0x8c01032c);
        cpu.pr = 0x8c0f3fe6; // 0x8c0f3fe2: jsr @r0 returns past the delay slot
        // 0x8c0f3fe4 delay slot: mov.l @r14,r14 — the successor is live in r14
        // before `free` can touch the node.
        cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    // 0x8c0f3fec: mov.l r0,@r0 — r0 is the sentinel loaded by the last cmp.
    let _ = mem.write_u32(cpu.r[0], cpu.r[0]);
    // 0x8c0f3fee: mov.l @r13,r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c0f3ff0: mov.l r0,@(4,r0)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[0]);

    // 0x8c0f3ff2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0f3ff4: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0f3ff6: rts / 0x8c0f3ff8 delay: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
