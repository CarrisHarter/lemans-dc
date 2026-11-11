//! 0x8c027580 — six-iteration copy loop between the r4 and r5 structs.
//!
//! Per iteration (byte offset `off` = 0,8,…,40, dword offset `dw` = off/2) the
//! loop copies four halfwords at each of +0xb4, +0xe4, +0x114 from the r5
//! struct to the r4 struct, plus one u32 at +0x144. The u32 move runs in the
//! `bt.s` delay slot, so it executes on all six iterations.
//!
//! Hidden side effects: r4/r5 end as the +0x144 pointers, r3/r6/r7 as the
//! +0x114/+0xe4 pointers advanced by the last offset (40), r0 is the
//! pre-increment r9 (20), r1 is the last u32 moved, T=0 (final cmp/hi is
//! 6 vs 6), and r8..r15 are restored by the epilogue so they keep their
//! entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let p2 = cpu.r[5];
    let d_b4 = p1.wrapping_add(0xb4);
    let s_b4 = p2.wrapping_add(0xb4);
    let d_e4 = p1.wrapping_add(0xe4);
    let s_e4 = p2.wrapping_add(0xe4);
    let d_114 = p1.wrapping_add(0x114);
    let s_114 = p2.wrapping_add(0x114);
    let d_144 = p1.wrapping_add(0x144);
    let s_144 = p2.wrapping_add(0x144);

    let mut last = 0u32; // r1 on exit
    let mut off = 0u32; // r10
    let mut dw = 0u32; // r9
    for _ in 0..6 {
        for k in 0..4u32 {
            let v = mem.read_u16(s_b4.wrapping_add(off + 2 * k)).unwrap_or(0);
            let _ = mem.write_u16(d_b4.wrapping_add(off + 2 * k), v);
        }
        for k in 0..4u32 {
            let v = mem.read_u16(s_e4.wrapping_add(off + 2 * k)).unwrap_or(0);
            let _ = mem.write_u16(d_e4.wrapping_add(off + 2 * k), v);
        }
        for k in 0..4u32 {
            let v = mem.read_u16(s_114.wrapping_add(off + 2 * k)).unwrap_or(0);
            let _ = mem.write_u16(d_114.wrapping_add(off + 2 * k), v);
        }
        // delay slot of `bt.s`: runs on every iteration
        last = mem.read_u32(s_144.wrapping_add(dw)).unwrap_or(0);
        let _ = mem.write_u32(d_144.wrapping_add(dw), last);
        off = off.wrapping_add(8);
        dw = dw.wrapping_add(4);
    }

    cpu.r[0] = dw - 4; // mov r9,r0 on the last iteration (before add #4,r9)
    cpu.r[1] = last;
    cpu.r[3] = d_114.wrapping_add(off - 8);
    cpu.r[4] = d_144;
    cpu.r[5] = s_144;
    cpu.r[6] = s_e4.wrapping_add(off - 8);
    cpu.r[7] = d_e4.wrapping_add(off - 8);
    // r8..r15 keep their entry values (restored by the epilogue pops)
    cpu.set_t(false); // cmp/hi r11,r0 with r11 == r0 == 6
}
