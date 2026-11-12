//! 0x8c051a60 — range-checked fetch: copy one float to `*r6`, return a vtable slot.
//!
//! Saves r11-r14/pr, calls 0x8c04a578 (which returns node 0x8c23dc38, doc'd in
//! that port). If node[1] is zero, or `r5 + 1` is not strictly below node[1]
//! (unsigned `cmp/hs`), returns 0. Else pushes `*(r4 + 0x1d4)` as a stack
//! argument and calls 0x8c04ace0 with (node, sp, r5); that callee returns
//! `*(... + 0x10) + r5*4`. That sum (in bytes, `*4`) indexes a two-level table
//! off node[0]: copies the float there to `*r6` (fmov.s pair) and returns the
//! word at offset 12 of the same second-level entry.
//!
//! Hidden side effects: r0 is clobbered with 3 at the first jsr (literal load);
//! the literal r0 load for 0x1d4 happens on the fall-through path only; r3
//! keeps the callee-sum-shll2 index, r1 the second-level entry; fr0 is the
//! copied float; delay slots are nop / `mov r15,r5` / `mov r0,r0` / `r14`
//! restore — the stack slot and register restores are the only stack writes.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[11] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c051acc).unwrap_or(0); // 0x8c04a578
    cpu.pr = 0x8c051a78;
    crate::registry::p_8c04a578::run(cpu, mem); // delay slot: nop
    cpu.r[14] = cpu.r[0];
    cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[1] & cpu.r[1] == 0); // tst r1,r1
    if cpu.t() {
        // bra to epilogue; delay slot mov #0,r0
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = cpu.r[12];
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.set_t(cpu.r[0] >= cpu.r[1]); // cmp/hs r1,r0
        if cpu.t() {
            // bra to epilogue; delay slot mov #0,r0
            cpu.r[0] = 0;
        } else {
            cpu.r[0] = mem.read_u32(0x8c051ad0).unwrap_or(0); // 0x1d4
            cpu.r[6] = cpu.r[12];
            cpu.r[4] = cpu.r[14];
            cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[11])).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[0] = mem.read_u32(0x8c051ad4).unwrap_or(0); // 0x8c04ace0
            cpu.r[5] = cpu.r[15]; // delay slot of `jsr`
            cpu.pr = 0x8c051aa0;
            crate::registry::p_8c04ace0::run(cpu, mem);
            cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
            cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[3] <<= 2; // shll2 r3
            cpu.r[0] = mem.read_u32(0x8c051ad0).unwrap_or(0); // 0x1d4
            cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[11])).unwrap_or(0);
            cpu.r[0] <<= 2; // shll2 r0
            cpu.r[1] = mem
                .read_u32(cpu.r[1].wrapping_add(cpu.r[0]))
                .unwrap_or(0);
            cpu.r[0] = cpu.r[3];
            cpu.r[1] = mem
                .read_u32(cpu.r[1].wrapping_add(cpu.r[0]))
                .unwrap_or(0);
            cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
            let _ = mem.write_f32(cpu.r[13], cpu.fr[0]);
            cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[1] = mem
                .read_u32(cpu.r[1].wrapping_add(cpu.r[0]))
                .unwrap_or(0);
            cpu.r[0] = mem
                .read_u32(cpu.r[1].wrapping_add(cpu.r[0]))
                .unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0);
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
