//! 0x8c0bfb4c — look up a key in a binary tree and return the associated word.
//!
//! ```text
//! push r14 ; push pr ; add #-16,r15 ; r14 = param_1
//! local[0] = param_2 (key)                 ; [r15]
//! local[1] = *(0x8c1fd870)                 ; [r15+4]
//! r4 = param_1 + 0x1c ; r5 = local ; push &local[2] ; call 0x8c0c06e4
//! r4 = local ; add #12,r4 ; r5 = param_1 + 0x20 ; call 0x8c0bef98
//! r0 = local[3] ; if r0 == r14 -> r0 = 0
//!                else           -> r0 = *(r14 + 0x10)
//! pop pr, r14
//! ```
//!
//! The first callee walks the tree rooted at `[param_1+0x20]` for the key in
//! `local[0]` and stores its find/insertion pointer into `local[2]` (the callee
//! reads that output slot from the stack top). The second callee stores
//! `param_1+0x20` into `local[3]`. When the walk found nothing (its pointer
//! equals `param_1+0x20`) the function returns 0; otherwise it loads the word
//! attached at `node+0x10`. The delay slot of the first `jsr` already advances
//! r4 by 0x1c, and the delay slot of the second `jsr` loads r14 from `local[2]`.
//!
//! Exit state: r0 is the result, T is set iff the two local pointers matched,
//! r4 = local + 12, r5 = param_1+0x20, r15/pr and r14 restored. Both call
//! targets come from the literal pool (0x8c0bfb98 / 0x8c0bfb9c) and are
//! dispatched through the registry with `pr` set to the return address.

use sh4_oracle::{Cpu, Memory};

fn call(cpu: &mut Cpu, mem: &mut Memory, target: u32, from: u32) {
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x{from:08x}: call to unported target 0x{target:08x}"),
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0bfb4c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 8c0bfb4e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c0bfb50: add #-16,r15 ; 8c0bfb52: mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[14] = cpu.r[4];
    // 8c0bfb54: mov.l 0x8c0bfb94,r0 ; 8c0bfb56: mov.l @r0,r1
    cpu.r[0] = mem.read_u32(0x8c0bfb94).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 8c0bfb58: mov.l r5,@r15 ; 8c0bfb5a: mov r15,r0
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    cpu.r[0] = cpu.r[15];
    // 8c0bfb5c: mov.l r1,@(4,r15) ; 8c0bfb5e: add #8,r0
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    // 8c0bfb60: mov r15,r5 ; 8c0bfb62: mov.l r0,@-r15
    cpu.r[5] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 8c0bfb64: mov.l 0x8c0bfb98,r0
    cpu.r[0] = mem.read_u32(0x8c0bfb98).unwrap_or(0);
    // 8c0bfb66: jsr @r0 ; delay 8c0bfb68: add #28,r4
    let target = cpu.r[0];
    cpu.pr = 0x8c0bfb6a;
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    call(cpu, mem, target, 0x8c0bfb66);
    // 8c0bfb6a: add #4,r15 ; 8c0bfb6c: add #28,r14 ; 8c0bfb6e: add #4,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = cpu.r[14].wrapping_add(28);
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    // 8c0bfb70: mov r14,r5 ; 8c0bfb72: mov r15,r4
    cpu.r[5] = cpu.r[14];
    cpu.r[4] = cpu.r[15];
    // 8c0bfb74: mov.l 0x8c0bfb9c,r0
    cpu.r[0] = mem.read_u32(0x8c0bfb9c).unwrap_or(0);
    // 8c0bfb76: add #12,r4 (delay slot)
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    // 8c0bfb78: jsr @r0 ; delay 8c0bfb7a: mov.l @(8,r15),r14
    let target = cpu.r[0];
    cpu.pr = 0x8c0bfb7c;
    cpu.r[14] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    call(cpu, mem, target, 0x8c0bfb78);
    // 8c0bfb7c: mov.l @(12,r15),r0 ; 8c0bfb7e: cmp/eq r0,r14
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    let same = cpu.r[0] == cpu.r[14];
    cpu.set_t(same);
    if same {
        // 8c0bfb80: bt 0x8c0bfb88 ; 8c0bfb88: mov #0,r0
        cpu.r[0] = 0;
    } else {
        // 8c0bfb82: add #12,r14 ; 8c0bfb84: bra 0x8c0bfb8a
        // delay 8c0bfb86: mov.l @(4,r14),r0
        cpu.r[14] = cpu.r[14].wrapping_add(12);
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    }
    // 8c0bfb8a: add #16,r15 ; 8c0bfb8c: lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c0bfb8e: rts ; delay 8c0bfb90: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
