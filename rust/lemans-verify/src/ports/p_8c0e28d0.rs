//! 0x8c0e28d0 — fill a 16-word block from the current global list head.
//!
//! `r5` is the destination; `r4` and `r6` are forwarded to 0x8c0e8e08
//! (`r6` becomes its second argument via the `jsr` delay slot). The callee
//! may rewrite the first words of the list head (and, per mode, words 4..15)
//! before the block is copied, so the copy is deliberately made after the
//! call. `r0` ends as the last word loaded (src[15]), `r1` as src[14],
//! `r3` as src[12]; T is whatever the callee left behind.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0e28d0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0e28d2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0e28d4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0e28d6: mov.l 0x8c0e2928,r0 (0x8c0e8e08)
    cpu.r[0] = mem.read_u32(0x8C0E2928).unwrap_or(0);
    // 0x8c0e28d8: jsr @r0
    cpu.pr = 0x8C0E28DC;
    // 0x8c0e28da: mov r6,r5 (delay slot)
    cpu.r[5] = cpu.r[6];
    crate::registry::p_8c0e8e08::run(cpu, mem);
    // 0x8c0e28dc: mov.l 0x8c0e292c,r0 (0x8c27a950)
    cpu.r[0] = mem.read_u32(0x8C0E292C).unwrap_or(0);
    // 0x8c0e28de: mov.l @r0,r0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c0e28e0..0x8c0e2916: seven `mov.l @(n,r0),r3 / mov.l @(n+4,r0),r1 /
    // store pair` groups, n = 0,8,..,48.
    for off in (0..56).step_by(8) {
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(off)).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(off + 4)).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(off), cpu.r[3]);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(off + 4), cpu.r[1]);
    }
    // 0x8c0e2918: mov.l @(56,r0),r1 ; 0x8c0e291a: mov.l @(60,r0),r0
    // (the final word replaces r0 itself, so snapshot the base first)
    let src15 = mem.read_u32(cpu.r[0].wrapping_add(60)).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(56)).unwrap_or(0);
    cpu.r[0] = src15;
    // 0x8c0e291c: mov.l r1,@(56,r14) ; 0x8c0e291e: mov.l r0,@(60,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(56), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(60), cpu.r[0]);
    // 0x8c0e2920: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0e2922: rts ; 0x8c0e2924: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
