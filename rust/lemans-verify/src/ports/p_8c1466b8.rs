//! 0x8c1466b8 — publish a mode word and clear flags on each live element.
//!
//! `[r4+0xa0] = r5` (the mode), then for i = 0..39: `e = [r4 + 4i]`; when e is
//! non-null, call 0x8c0bd8c8(e, mode == 1 ? 1 : 0). The loop test is
//! `cmp/gt r14,r0` with r0 = 40, so it runs 40 times and leaves T=0, r0=40.
//! r11..r14 are pushed, so they end at entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4]; // mov r4,r11
    let mode = cpu.r[5]; // mov r5,r12
    let mode_off = mem.read_u32(0x8c146704).unwrap_or(0xa0); // mov.l 0x8c146704,r0
    let _ = mem.write_u32(base.wrapping_add(mode_off), mode); // mov.l r12,@(r0,r11)

    let callee = mem.read_u32(0x8c146708).unwrap_or(0x8c0bd8c8); // mov.l 0x8c146708,r0
    for i in 0..40u32 {
        let elem = mem.read_u32(base.wrapping_add(i.wrapping_mul(4))).unwrap_or(0); // mov.l @(r0,r11),r4
        cpu.r[4] = elem;
        cpu.set_t(elem == 0); // tst r4,r4
        if elem != 0 {
            cpu.r[0] = callee;
            cpu.r[5] = if mode == 1 { 1 } else { 0 }; // mov #1/#0,r5 (delay slot)
            cpu.pr = if mode == 1 { 0x8c1466e2 } else { 0x8c1466ec };
            crate::registry::p_8c0bd8c8::run(cpu, mem); // jsr @r0
        }
    }
    cpu.r[0] = 40; // mov #40,r0
    cpu.set_t(false); // cmp/gt r14,r0 with r14 == 40
}
