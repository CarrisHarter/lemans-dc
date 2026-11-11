//! 0x8c027580 — six-iteration copy loop between the r4 and r5 structs.
//!
//! Store pattern per iteration (byte offset `off` = 0,8,…,40, dword offset
//! `dw` = off/2), exactly as the oracle executes it:
//!   u16 p2+0xb4+off -> p1+0xb4+off
//!   u16 p2+0x76/78/7a+off -> p2+0xb6/b8/ba+off   (the `@(2/4/6,…)` slots land
//!   u16 p2+0xe4+off -> p1+0xe4+off                 on r1-based addresses in
//!   u16 p2+0x1a6/1a8/1aa+off -> p2+0x1f6/1f8/1fa+off  the oracle's decode)
//!   u16 p2+0x114+off -> p1+0x114+off
//!   u16 p2+0x136/138/13a+off -> p2+0x176/178/17a+off
//!   u32 p2+0x144+dw -> p1+0x144+dw   (bt.s delay slot, runs every iteration)
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
        let v = mem.read_u16(s_b4.wrapping_add(off)).unwrap_or(0);
        let _ = mem.write_u16(d_b4.wrapping_add(off), v);
        for (i, k) in [0x76u32, 0x78, 0x7a].iter().enumerate() {
            let v = mem.read_u16(s_b4.wrapping_add(off + k - 0xb4)).unwrap_or(0);
            let _ = mem.write_u16(s_b4.wrapping_add(off + 2 + 2 * i as u32), v);
        }
        let v = mem.read_u16(s_e4.wrapping_add(off)).unwrap_or(0);
        let _ = mem.write_u16(d_e4.wrapping_add(off), v);
        for (i, k) in [0x1a6u32, 0x1a8, 0x1aa].iter().enumerate() {
            let v = mem.read_u16(s_e4.wrapping_add(off + k - 0xe4)).unwrap_or(0);
            let _ = mem.write_u16(p2.wrapping_add(0x1f6 + off + 2 * i as u32), v);
        }
        let v = mem.read_u16(s_114.wrapping_add(off)).unwrap_or(0);
        let _ = mem.write_u16(d_114.wrapping_add(off), v);
        for (i, k) in [0x136u32, 0x138, 0x13a].iter().enumerate() {
            let v = mem.read_u16(s_114.wrapping_add(off + k - 0x114)).unwrap_or(0);
            let _ = mem.write_u16(p2.wrapping_add(0x176 + off + 2 * i as u32), v);
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
