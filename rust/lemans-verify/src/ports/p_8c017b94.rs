//! 0x8c017b94 — variable-length bit unpack: reads a tag byte at [r4], emits a
//! u32 at [r5], returns the advanced pointer in r0.
//!
//! Branch polarity: `tst`+`bf` branches when the tested bit is SET, so the
//! single-byte path (r4+1) runs when bit0 == 0.
//! Side effects the C hides: r6 = unpacked value, r3/r1 = shifted source
//! bytes on the longer paths, and T from the last shift. Every path ends with a
//! shift whose carry bit is provably zero (bit0 of a tag already known clear, or
//! bit24 of a zero-extended byte), so T is always 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let b0 = u32::from(mem.read_u8(r4).unwrap_or(0));
    let (val, adv, t): (u32, u32, bool);
    if b0 & 1 == 0 {
        val = b0 >> 1;
        adv = 1;
        t = false; // shlr r6 shifted out bit0 = 0
    } else if b0 & 2 == 0 {
        let b1 = u32::from(mem.read_u8(r4 + 1).unwrap_or(0)); // mov.b @(1,r4),r0
        cpu.r[3] = b1;
        val = (b0 >> 2) << 8 | b1;
        adv = 2;
        t = false; // last shift is shll8 r6 with r6 = b0>>2, so bit24 is 0
    } else if b0 & 4 == 0 {
        let b1 = u32::from(mem.read_u8(r4 + 1).unwrap_or(0));
        let b2 = u32::from(mem.read_u8(r4 + 2).unwrap_or(0)); // mov.b @(2,r4),r0
        cpu.r[3] = b1 << 8; // shll8 r3
        cpu.r[1] = b2;
        val = (b0 >> 3) << 16 | b1 << 8 | b2;
        adv = 3;
        t = false; // shlr r6 (after shlr2) shifted out bit2 = 0
    } else {
        let b1 = u32::from(mem.read_u8(r4 + 1).unwrap_or(0));
        let b2 = u32::from(mem.read_u8(r4 + 2).unwrap_or(0));
        let b3 = u32::from(mem.read_u8(r4 + 3).unwrap_or(0)); // mov.b @(3,r4),r0
        cpu.r[3] = b1 << 16; // shll16 r3
        cpu.r[1] = b2 << 8; // shll8 r1
        val = (b0 >> 3) << 24 | b1 << 16 | b2 << 8 | b3;
        adv = 4;
        t = false; // last shift is shll8 r1 with r1 = b2 (a byte), so bit24 is 0
    }
    cpu.r[6] = val;
    cpu.r[4] = r4.wrapping_add(adv);
    let _ = mem.write_u32(r5, val);
    cpu.r[0] = cpu.r[4]; // bra delay slot: mov r4,r0
    cpu.set_t(t);
}
