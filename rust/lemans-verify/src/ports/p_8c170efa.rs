//! 0x8c170efa — set one bit in a two-word multi-play mask.
//!
//! ```text
//! sts.l pr,@-r15; tst r5,r5; bt null (plain)
//! tst r6,r6; bf checked (plain)
//! null: jsr 0x8c1725fc(101, "…upperMask or lowerMask is NULL"); bra epilogue
//! checked: cmp/hs r2=64,r4; bf bitpath (plain)
//! jsr 0x8c1725fc(102, "…port is out of range")
//! epilogue: lds.l @r15+,pr; rts; delay mov #0,r0
//! bitpath: cmp/hi r3=31,r4; bf.s lo; delay mov #1,r7
//! r4>=32: or 1<<(r4-32) into [r5]; r0=1; restore pr; rts
//! lo: or 1<<r4 into [r6]; r0=1; restore pr; rts
//! ```
//!
//! `r5` is the upper mask (`bf.s` taken: bit `r4-32` with `r4` in 32..64),
//! `r6` the lower mask (`bf.s` not taken: bit `r4` with `r4` in 0..31, `r7`
//! already 1 from the delay slot). Either mask pointer NULL, or the port
//! index `r4 >= 64` unsigned, reports through the engine logger and returns 0.
//! The shift counts are in range on both reachable paths, so `shad` is the
//! same left shift the oracle computes.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    let mut t = cpu.t(); // bt 0x8c170f04 (plain)
    if !t {
        cpu.set_t(cpu.r[6] == 0); // tst r6,r6
        t = cpu.t(); // bf 0x8c170f10 (plain): fall through when r6==0
    }
    if t {
        // 0x8c170f04: null-mask side.
        cpu.r[3] = mem.read_u32(0x8c170f8c).unwrap_or(0x8c1725fc);
        cpu.r[5] = mem.read_u32(0x8c170f98).unwrap_or(0x8c1de13c);
        cpu.pr = 0x8c170f0c; // jsr @r3
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c1725fc::run(cpu, mem);
        // bra 0x8c170f1e; delay nop
    } else {
        // 0x8c170f10: index-range side.
        cpu.r[2] = 64;
        cpu.set_t(cpu.r[4] >= cpu.r[2]); // cmp/hs r2,r4
        if cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c170f8c).unwrap_or(0x8c1725fc);
            cpu.r[5] = mem.read_u32(0x8c170f9c).unwrap_or(0x8c1de170);
            cpu.pr = 0x8c170f1e; // jsr @r1
            cpu.r[4] = 102; // delay slot: mov #102,r4
            crate::registry::p_8c1725fc::run(cpu, mem);
        } else {
            // 0x8c170f24: set the bit. bf.s with delay mov #1,r7.
            cpu.r[3] = 31;
            cpu.set_t(cpu.r[4] > cpu.r[3]); // cmp/hi r3,r4
            let to_lo = !cpu.t();
            cpu.r[7] = 1; // delay slot
            if to_lo {
                // 0x8c170f38: r4 <= 31, bit r4 in [r6].
                cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
                cpu.r[7] = cpu.r[7].wrapping_shl(cpu.r[4] & 31); // shad r4,r7
                cpu.r[1] |= cpu.r[7]; // or r7,r1
                let _ = mem.write_u32(cpu.r[6], cpu.r[1]);
            } else {
                // r4 in 32..64, bit (r4-32) in [r5].
                cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
                cpu.r[4] = cpu.r[4].wrapping_add((-32i32) as u32); // add #-32,r4
                cpu.r[7] = cpu.r[7].wrapping_shl(cpu.r[4] & 31); // shad r4,r7
                cpu.r[1] |= cpu.r[7]; // or r7,r1
                // bra 0x8c170f40; delay mov.l r1,@r5
                let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
            }
            cpu.r[0] = 1;
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            return; // rts; delay nop
        }
    }
    // 0x8c170f1e: logger paths return 0.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0; // delay slot of rts: mov #0,r0
}
