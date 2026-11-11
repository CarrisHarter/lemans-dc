//! 0x8c0d0edc — pick a row pointer inside the struct at r4: returns
//! r4 + 68 + 12*min(r5, ~r5 & 7) chosen by comparing the two floats at
//! r4+76+12*r5 (fr1) and r4+76+12*(~r5&7) (fr0).
//!
//! Oracle encoding: `fcmp/gt fr1,fr0` sets T = fr0 > fr1, so the
//! `add r6,r4` path (r4 += 12*r5, then the bra delay slot adds 68) runs
//! when mem[r4+76+12*(~r5&7)] > mem[r4+76+12*r5]. Residue: r0 = new r4
//! (rts delay slot), r1 = 12*(~r5&7) on that path, r3 = r4+76+12*r5,
//! r6 = 12*r5, fr0/fr1 hold the two loaded floats, T = fr0 > fr1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r0 = (!r5) & 7; // not r5,r0 ; and #7,r0
    let mut r1 = r0 << 1; // mov r0,r1 ; shll r1
    let r6 = r5 << 1; // mov r5,r6 ; shll r6
    r1 = r1.wrapping_add(r0); // add r0,r1 -> 3*(~r5&7)
    let mut r3 = r4.wrapping_add(76); // mov r4,r3 ; add #76,r3
    r1 = r1 << 2; // shll2 r1 -> 12*(~r5&7)
    let r6 = r6.wrapping_add(r5); // add r5,r6 -> 3*r5
    let r0a = r1.wrapping_add(r3); // mov r1,r0 ; add r3,r0
    let r6 = r6 << 2; // shll2 r6 -> 12*r5
    r3 = r3.wrapping_add(r6); // add r6,r3 -> r4+76+12*r5
    cpu.fr[1] = f32::from_bits(mem.read_u32(r3).unwrap_or(0)); // fmov @r3,fr1
    cpu.fr[0] = f32::from_bits(mem.read_u32(r0a).unwrap_or(0)); // fmov @r0,fr0
    let t = cpu.fr[0] > cpu.fr[1]; // fcmp/gt fr1,fr0 -> T = fr0 > fr1
    cpu.set_t(t);
    let mut r4n = r4;
    if t {
        r4n = r4n.wrapping_add(r6); // add r6,r4
        r4n = r4n.wrapping_add(68); // bra delay slot: add #68,r4 (always runs)
    } else {
        r1 = r1.wrapping_add(r4); // add r4,r1
        r1 = r1.wrapping_add(68); // add #68,r1
        r4n = r1; // mov r1,r4
    }
    cpu.r[0] = r4n; // rts delay slot: mov r4,r0
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4n;
    cpu.r[6] = r6;
}
