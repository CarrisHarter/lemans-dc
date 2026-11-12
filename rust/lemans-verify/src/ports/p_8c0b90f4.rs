//! 0x8c0b90f4 — unlink every node of a chain whose key matches, freeing it.
//!
//! r4 = list header (`head @4`), r5 = first node to scan, r6 = the predecessor
//! node that owns this chain (`prev`, may be 0), r7 = key. A node is
//! `next @0`, `kind @4`, `key @8`. `kind == -1` marks a directory: the routine
//! recurses into `node->key` with `prev = node`, then — if that child link
//! ended up null — drops the directory node itself. Any other node is freed
//! when its key equals r7.
//!
//! Unlinking is the same code twice: with `prev` it patches `prev->key`, else
//! it patches `list->head` — but only when the link actually points at the
//! node being freed. After a free the walking predecessor r14 (0 while no node
//! has been skipped) is patched to the successor, so a chain of matches stays
//! linked. r9..r14 and pr come back from the frame; the last `tst r10,r10`
//! before either exit sees a null node, so T ends set.

use sh4_oracle::{Cpu, Memory};

use super::p_8c01032c;

/// 0x8c0b91a4 — free(), via the thunk at 0x8c01032c.
const LIT_FREE: u32 = 0x8c01032c;
/// 0x8c0b91a8 — this routine's own address, used for the directory descent.
const LIT_SELF: u32 = 0x8c0b90f4;

#[inline]
fn push(cpu: &mut Cpu, mem: &mut Memory, reg: usize) {
    let sp = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(sp, cpu.r[reg]);
    cpu.r[15] = sp;
}

#[inline]
fn pop(cpu: &mut Cpu, mem: &mut Memory, reg: usize) {
    cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

/// 0x8c0b9192 — restore pr and r9..r14, then return. Both entry points here
/// came through `tst r10,r10` with a null node, so T is set.
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [9usize, 10, 11, 12, 13, 14] {
        pop(cpu, mem, reg); // r14 pops in the rts delay slot
    }
    cpu.set_t(true);
}

/// 0x8c0b911e (and the identical copy at 0x8c0b9160): unlink r10 from the
/// chain, free it, and re-point r14 at the successor. The `bra` delay slot is
/// skipped when r14 is still 0.
fn unlink_and_free(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[11] == 0); // 0x8c0b911e: tst r11,r11
    if cpu.t() {
        // 0x8c0b912e: patch the list head.
        cpu.r[0] = mem.read_u32(cpu.r[9].wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[10]); // cmp/eq r10,r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[10]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[9].wrapping_add(4), cpu.r[0]);
        }
    } else {
        // 0x8c0b9122: patch the predecessor's child link.
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[10]); // cmp/eq r10,r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[10]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[11].wrapping_add(8), cpu.r[0]);
        }
    }
    // 0x8c0b9138: free the node; the delay slot advances r10 first.
    cpu.r[4] = cpu.r[10]; // mov r10,r4
    cpu.r[0] = LIT_FREE; // mov.l 0x8c0b91a4,r0
    cpu.pr = 0x8c0b9140;
    cpu.r[10] = mem.read_u32(cpu.r[10]).unwrap_or(0); // delay: mov.l @r10,r10
    p_8c01032c::run(cpu, mem);
    cpu.set_t(cpu.r[14] == 0); // 0x8c0b9140: tst r14,r14
    if !cpu.t() {
        let _ = mem.write_u32(cpu.r[14], cpu.r[10]); // 0x8c0b9146: bra delay
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, 14); // 0x8c0b90f4
    push(cpu, mem, 13); // 0x8c0b90f6
    push(cpu, mem, 12); // 0x8c0b90f8
    push(cpu, mem, 11); // 0x8c0b90fa
    cpu.r[12] = cpu.r[7]; // 0x8c0b90fc: mov r7,r12
    push(cpu, mem, 10); // 0x8c0b90fe
    cpu.r[11] = cpu.r[6]; // 0x8c0b9100: mov r6,r11
    push(cpu, mem, 9); // 0x8c0b9102
    cpu.r[10] = cpu.r[5]; // 0x8c0b9104: mov r5,r10
    {
        // 0x8c0b9106: sts.l pr,@-r15
        let sp = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(sp, cpu.pr);
        cpu.r[15] = sp;
    }
    cpu.r[9] = cpu.r[4]; // 0x8c0b9108: mov r4,r9

    cpu.set_t(cpu.r[10] == 0); // 0x8c0b910a: tst r10,r10
    cpu.r[14] = 0; // 0x8c0b910e: bt.s delay, runs whether or not the branch is taken
    if cpu.t() {
        epilogue(cpu, mem);
        return;
    }
    cpu.r[13] = u32::MAX; // 0x8c0b9110: mov #-1,r13

    loop {
        // 0x8c0b9112: a directory node?
        cpu.r[0] = mem.read_u32(cpu.r[10].wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[13]); // 0x8c0b9114: cmp/eq r13,r0
        if cpu.t() {
            // 0x8c0b914e: recurse over node->key with prev = this node.
            cpu.r[5] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
            cpu.r[7] = cpu.r[12]; // mov r12,r7
            cpu.r[0] = LIT_SELF;
            cpu.r[6] = cpu.r[10]; // mov r10,r6
            cpu.pr = 0x8c0b915a;
            cpu.r[4] = cpu.r[9]; // delay: mov r9,r4
            run(cpu, mem);
            // 0x8c0b915a: did the descent leave the directory empty?
            cpu.r[0] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if !cpu.t() {
                cpu.r[14] = cpu.r[10]; // 0x8c0b918a
                cpu.r[10] = mem.read_u32(cpu.r[10]).unwrap_or(0);
            } else {
                unlink_and_free(cpu, mem);
            }
        } else {
            // 0x8c0b9118: key match?
            cpu.r[0] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == cpu.r[12]); // 0x8c0b911a: cmp/eq r12,r0
            if !cpu.t() {
                // 0x8c0b9148: skip this node, remembering it as the link.
                cpu.r[14] = cpu.r[10];
                cpu.r[10] = mem.read_u32(cpu.r[10]).unwrap_or(0); // bra delay
            } else {
                unlink_and_free(cpu, mem);
            }
        }
        // 0x8c0b918e
        cpu.set_t(cpu.r[10] == 0); // tst r10,r10
        if cpu.t() {
            break;
        }
    }
    epilogue(cpu, mem);
}
