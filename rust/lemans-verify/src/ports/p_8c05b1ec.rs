//! 0x8c05b1ec — recursive scene-node lookup by name.
//!
//! Compares `node->name` (offset 0x24) against the requested C string and, when
//! that differs, walks `node->first_child` (offset 0x0c) and each child's
//! sibling chain (offset 0x08). A child whose own name matches is returned;
//! otherwise the routine recurses into that child's children (the recursive
//! `jsr` goes through the literal at 0x8c05b2a8, which holds this address).
//!
//! ```text
//! mov.l @(36,r4),r5 ; tst r5,r5 ; bt use_default   ; name or 0x8c1f7328
//! compare r5 with r6 = param_2  (byte loop, zero-extended)
//! equal -> return param_1
//! r13 = node->first_child ; null -> return 0
//! per r13: compare [r13+0x24] with param_2
//!          equal  -> return r13
//!          differ -> r12 = [r13+0x0c]
//!                    for each r12: result = f(r12, param_2)
//!                                  result != 0 -> return result
//!                                  r12 = [r12+8]
//!                    r13 = [r13+8] ; repeat
//! ```
//!
//! The prologue saves r14/r13/r12/pr and the epilogue restores them, so those
//! registers and pr keep their entry values; r15 is unchanged once the frame is
//! popped. `mov.b` sign-extends but every load is followed by `extu.b`, so the
//! comparison consumes zero-extended bytes.

use sh4_oracle::{Cpu, Memory};

/// Literal-pool word holding the fallback name pointer.
const DEFAULT_NAME_LITERAL: u32 = 0x8c05b2a4;
/// Literal-pool word holding the routine's own entry address.
const SELF_LITERAL: u32 = 0x8c05b2a8;

/// `mov.b @addr,rN` + `extu.b` collapses to the zero-extended byte.
fn load_byte(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as u32
}

/// One invocation of 0x8c05b1ec: r4 = node, r5 = name; result in r0.
fn search(cpu: &mut Cpu, mem: &mut Memory) -> u32 {
    // sts.l pr,@-r15 ; mov.l r12,@-r15 ; mov.l r13,@-r15 ; mov.l r14,@-r15
    let saved_r12 = cpu.r[12];
    let saved_r13 = cpu.r[13];
    let saved_r14 = cpu.r[14];

    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[5] = mem
        .read_u32(cpu.r[4].wrapping_add(36))
        .unwrap_or(0); // mov.l @(36,r4),r5
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        // mov.l 0x8c05b2a4,r5
        cpu.r[5] = mem.read_u32(DEFAULT_NAME_LITERAL).unwrap_or(0);
    }
    cpu.r[6] = cpu.r[14]; // delay slot of `bra 0x8c05b210`

    // 0x8c05b210..0x8c05b224 — compare node->name with param_2.
    let equal_first;
    loop {
        cpu.r[3] = load_byte(mem, cpu.r[5]); // mov.b @r5,r3 ; extu.b r3,r3
        cpu.r[0] = load_byte(mem, cpu.r[6]); // mov.b @r6,r0 ; extu.b r0,r0
        cpu.r[1] = cpu.r[3]; // extu.b r3,r1
        cpu.r[5] = cpu.r[5].wrapping_add(1); // add #1,r5
        let same = cpu.r[1] == cpu.r[0]; // cmp/eq r0,r1
        cpu.set_t(same);
        if same {
            cpu.r[6] = cpu.r[6].wrapping_add(1); // delay slot of `bt.s`
            cpu.r[0] = cpu.r[3]; // extu.b r3,r0 at 0x8c05b206
            let end = cpu.r[0] == 0; // tst r0,r0
            cpu.set_t(end);
            if end {
                equal_first = true;
                break;
            }
        } else {
            cpu.r[6] = cpu.r[6].wrapping_add(1); // delay slot of `bt.s`
            cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]); // sub r0,r1
            equal_first = false;
            break;
        }
    }

    if equal_first {
        cpu.r[1] = 0; // delay slot of `bra 0x8c05b226` (0x8c05b20e)
    }
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1 at 0x8c05b226
    if equal_first {
        cpu.r[0] = cpu.r[4]; // delay slot of `bra 0x8c05b29a`
        cpu.r[12] = saved_r12;
        cpu.r[13] = saved_r13;
        cpu.r[14] = saved_r14;
        return cpu.r[0];
    }

    // 0x8c05b22e — node->first_child.
    cpu.r[13] = mem
        .read_u32(cpu.r[4].wrapping_add(12))
        .unwrap_or(0); // mov.l @(12,r4),r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] == 0 {
        cpu.r[0] = 0; // mov #0,r0 at 0x8c05b298
        cpu.r[12] = saved_r12;
        cpu.r[13] = saved_r13;
        cpu.r[14] = saved_r14;
        return cpu.r[0];
    }

    'outer: loop {
        // 0x8c05b234 — compare this child's name with param_2.
        cpu.r[4] = mem
            .read_u32(cpu.r[13].wrapping_add(36))
            .unwrap_or(0); // mov.l @(36,r13),r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] == 0 {
            cpu.r[4] = mem.read_u32(DEFAULT_NAME_LITERAL).unwrap_or(0);
        }
        cpu.r[5] = cpu.r[14]; // delay slot of `bra 0x8c05b24e`

        let equal_second;
        loop {
            cpu.r[3] = load_byte(mem, cpu.r[4]); // mov.b @r4,r3 ; extu.b r3,r3
            cpu.r[0] = load_byte(mem, cpu.r[5]); // mov.b @r5,r0 ; extu.b r0,r0
            cpu.r[1] = cpu.r[3]; // extu.b r3,r1
            cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
            let same = cpu.r[1] == cpu.r[0]; // cmp/eq r0,r1
            cpu.set_t(same);
            if same {
                cpu.r[5] = cpu.r[5].wrapping_add(1); // delay slot of `bt.s`
                cpu.r[0] = cpu.r[3]; // extu.b r3,r0 at 0x8c05b244
                let end = cpu.r[0] == 0; // tst r0,r0
                cpu.set_t(end);
                if end {
                    equal_second = true;
                    break;
                }
            } else {
                cpu.r[5] = cpu.r[5].wrapping_add(1); // delay slot of `bt.s`
                cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]); // sub r0,r1
                equal_second = false;
                break;
            }
        }

        if equal_second {
            cpu.r[1] = 0; // delay slot of `bra 0x8c05b264` (0x8c05b24c)
        }
        cpu.set_t(cpu.r[1] == 0); // tst r1,r1 at 0x8c05b264
        if equal_second {
            cpu.r[0] = cpu.r[13]; // delay slot of `bra 0x8c05b28a`
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0 at 0x8c05b28a
            cpu.r[12] = saved_r12;
            cpu.r[13] = saved_r13;
            cpu.r[14] = saved_r14;
            return cpu.r[0];
        }

        // 0x8c05b26c — search this child's own child list.
        cpu.r[12] = mem
            .read_u32(cpu.r[13].wrapping_add(12))
            .unwrap_or(0); // mov.l @(12,r13),r12
        cpu.set_t(cpu.r[12] == 0); // tst r12,r12
        if cpu.r[12] == 0 {
            cpu.r[0] = 0; // mov #0,r0 at 0x8c05b288
        } else {
            loop {
                cpu.r[0] = mem.read_u32(SELF_LITERAL).unwrap_or(0x8c05b1ec);
                cpu.r[5] = cpu.r[14]; // mov r14,r5
                cpu.r[4] = cpu.r[12]; // delay slot of `jsr @r0`
                let result = search(cpu, mem);
                cpu.set_t(result == 0); // tst r0,r0 at 0x8c05b27a
                if result != 0 {
                    // 0x8c05b28a — tst r0,r0, result is non-zero
                    cpu.set_t(false);
                    cpu.r[12] = saved_r12;
                    cpu.r[13] = saved_r13;
                    cpu.r[14] = saved_r14;
                    return result;
                }
                cpu.r[12] = mem
                    .read_u32(cpu.r[12].wrapping_add(8))
                    .unwrap_or(0); // mov.l @(8,r12),r12
                cpu.set_t(cpu.r[12] == 0); // tst r12,r12
                if cpu.r[12] != 0 {
                    continue;
                }
                cpu.r[0] = 0; // mov #0,r0 at 0x8c05b288
                break;
            }
        }

        // 0x8c05b28a..0x8c05b296 — r0 is 0 here; advance the child chain.
        cpu.set_t(true); // tst r0,r0 at 0x8c05b28a
        cpu.r[13] = mem
            .read_u32(cpu.r[13].wrapping_add(8))
            .unwrap_or(0); // mov.l @(8,r13),r13
        cpu.set_t(cpu.r[13] == 0); // tst r13,r13
        if cpu.r[13] != 0 {
            continue 'outer;
        }
        cpu.r[0] = 0; // mov #0,r0 at 0x8c05b298
        cpu.r[12] = saved_r12;
        cpu.r[13] = saved_r13;
        cpu.r[14] = saved_r14;
        return cpu.r[0];
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    search(cpu, mem);
}
