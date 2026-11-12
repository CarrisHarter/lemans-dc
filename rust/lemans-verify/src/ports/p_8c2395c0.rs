//! 0x8c2395c0 — build a conditional packet at 0x8c2c9dc0 from the struct at
//! `r4`: word0 = ([r4+4] & 0xffc30000) | (([r4] & 15) << 22), then words
//! copied from successive struct fields; fields guarded by bits 22/24/25 of
//! word0 (r7/r13/r14) are skipped when the bit is clear. `[dest+4]` gets the
//! byte count, returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[2] = 22;
    cpu.r[0] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[6] = 0;
    cpu.r[13] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
    cpu.r[1] = cpu.r[6];
    cpu.r[3] = mem.read_u32(0x8c239804).unwrap_or(0); // 0xffc30000
    cpu.r[0] &= 15;
    cpu.r[0] = cpu.r[0].wrapping_shl(22); // shld r2,r0
    cpu.r[5] = mem.read_u32(0x8c239800).unwrap_or(0); // 0x8c2c9dc0
    cpu.r[13] &= cpu.r[3];
    cpu.r[13] |= cpu.r[0];
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.r[1] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[1], cpu.r[13]);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    cpu.r[1] = cpu.r[6];
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.r[1] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.r[1] = cpu.r[6];
    cpu.r[0] = cpu.r[5];
    cpu.r[1] <<= 2; // shll2
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.r[14] = cpu.r[13];
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[2] = (cpu.r[2] as i32).wrapping_neg() as u32; // neg r2,r2
    cpu.r[7] = cpu.r[13];
    cpu.r[14] >>= 16; // shlr16
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.r[7] >>= 22; // shld r2,r7 (r2 = -22)
    cpu.r[1] = cpu.r[6];
    cpu.r[14] >>= 8; // shlr8
    cpu.r[2] = cpu.r[5];
    cpu.r[13] >>= 16; // shlr16
    cpu.r[13] >>= 8; // shlr8
    cpu.r[14] >>= 1; // shlr
    cpu.r[0] = 1;
    cpu.r[1] <<= 2; // shll2
    cpu.r[7] &= cpu.r[0];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[13] &= cpu.r[0];
    cpu.r[14] &= cpu.r[0];
    cpu.r[0] = cpu.r[4];
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[0].wrapping_add(16)).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
    cpu.r[2] = cpu.r[5];
    cpu.r[1] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[1] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[2]);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    cpu.r[2] = mem.read_u32(cpu.r[0].wrapping_add(20)).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    let _t14 = cpu.r[14] & cpu.r[14] == 0; // tst r14,r14 - overwritten below
    let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
    cpu.r[2] = cpu.r[5];
    cpu.r[1] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    // The bt.s at 0x8c239656 tests this shll2's carry (bit 30 of r6), not the
    // tst r14,r14 eleven instructions earlier.
    cpu.set_t(cpu.r[1] >> 30 & 1 != 0);
    cpu.r[1] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
    cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay slot
    if !cpu.t() {
        // r14 != 0: [r4+28] always, [r4+32] iff r7 == 0
        cpu.r[2] = cpu.r[5];
        cpu.r[0] = cpu.r[6];
        cpu.r[2] = cpu.r[2].wrapping_add(8);
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(28)).unwrap_or(0);
        cpu.r[0] <<= 2; // shll2
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]);
        cpu.set_t(cpu.r[7] & cpu.r[7] == 0); // tst r7,r7
        let _ = mem.write_u32(cpu.r[0], cpu.r[1]); // bf.s delay slot
        if cpu.t() {
            cpu.r[2] = cpu.r[5];
            cpu.r[0] = cpu.r[6];
            cpu.r[2] = cpu.r[2].wrapping_add(8);
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
            cpu.r[0] <<= 2; // shll2
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]);
            let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
            cpu.r[6] = cpu.r[6].wrapping_add(1);
        }
        cpu.r[2] = cpu.r[5];
        cpu.r[3] = cpu.r[6];
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
        cpu.r[2] = cpu.r[2].wrapping_add(8);
        cpu.r[3] <<= 2; // shll2
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
        cpu.set_t(cpu.r[13] & cpu.r[13] == 0); // tst r13,r13
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay slot
        if !cpu.t() {
            cpu.r[3] = cpu.r[5];
            cpu.r[2] = cpu.r[6];
            cpu.r[3] = cpu.r[3].wrapping_add(8);
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0);
            cpu.r[2] <<= 2; // shll2
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[2], cpu.r[1]); // bra delay slot
        }
    } else {
        // r14 == 0: [r4+36] directly
        cpu.r[1] = cpu.r[5];
        cpu.r[2] = cpu.r[6];
        cpu.r[1] = cpu.r[1].wrapping_add(8);
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
        cpu.r[2] <<= 2; // shll2
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[1]);
        let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
        cpu.r[6] = cpu.r[6].wrapping_add(1);
    }
    // join: [r4+44], [r4+48], [r4+52]
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[1] = cpu.r[4];
    cpu.r[3] <<= 2; // shll2
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[1].wrapping_add(44)).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[3] <<= 2; // shll2
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[1].wrapping_add(48)).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[3] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(52);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.set_t(cpu.r[14] & cpu.r[14] == 0); // tst r14,r14
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay slot
    if !cpu.t() {
        // r14 != 0: [r4+56] always, [r4+60] iff r7 == 0
        cpu.r[3] = cpu.r[5];
        cpu.r[1] = cpu.r[6];
        cpu.r[3] = cpu.r[3].wrapping_add(8);
        cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap_or(0);
        cpu.r[1] <<= 2; // shll2
        cpu.set_t(cpu.r[7] & cpu.r[7] == 0); // tst r7,r7
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
        let _ = mem.write_u32(cpu.r[1], cpu.r[2]); // bf.s delay slot
        if cpu.t() {
            cpu.r[3] = cpu.r[5];
            cpu.r[1] = cpu.r[6];
            cpu.r[3] = cpu.r[3].wrapping_add(8);
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(60)).unwrap_or(0);
            cpu.r[1] <<= 2; // shll2
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
            cpu.r[6] = cpu.r[6].wrapping_add(1);
        }
        cpu.r[2] = cpu.r[5];
        cpu.r[0] = 64;
        cpu.r[3] = cpu.r[6];
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(64)).unwrap_or(0);
        cpu.r[3] <<= 2; // shll2
        cpu.r[2] = cpu.r[2].wrapping_add(8);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
        cpu.set_t(cpu.r[13] & cpu.r[13] == 0); // tst r13,r13
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay slot
        if !cpu.t() {
            cpu.r[3] = cpu.r[5];
            cpu.r[0] = 68;
            cpu.r[2] = cpu.r[6];
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(68)).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(8);
            cpu.r[2] <<= 2; // shll2
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[2], cpu.r[1]); // bra delay slot
        }
    } else {
        // r14 == 0: [r4+64] directly
        cpu.r[0] = 64;
        cpu.r[3] = cpu.r[5];
        cpu.r[2] = cpu.r[6];
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(64)).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_add(8);
        cpu.r[2] <<= 2; // shll2
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
        let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
        cpu.r[6] = cpu.r[6].wrapping_add(1);
    }
    // join: [r4+72], [r4+76], [r4+80]
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[1] = cpu.r[4];
    cpu.r[3] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(72);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    cpu.r[1] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[3] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(76);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    cpu.r[1] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = cpu.r[6];
    cpu.r[2] = cpu.r[2].wrapping_add(8);
    cpu.r[3] <<= 2; // shll2
    cpu.r[1] = cpu.r[1].wrapping_add(80);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.set_t(cpu.r[14] & cpu.r[14] == 0); // tst r14,r14
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay slot
    if !cpu.t() {
        // r14 != 0: [r4+84] always, [r4+88] iff r7 == 0
        cpu.r[3] = cpu.r[5];
        cpu.r[1] = cpu.r[6];
        cpu.r[3] = cpu.r[3].wrapping_add(8);
        cpu.r[2] = cpu.r[4];
        cpu.r[1] <<= 2; // shll2
        cpu.r[2] = cpu.r[2].wrapping_add(84);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.set_t(cpu.r[7] & cpu.r[7] == 0); // tst r7,r7
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        let _ = mem.write_u32(cpu.r[1], cpu.r[3]); // bf.s delay slot
        if cpu.t() {
            cpu.r[0] = 88;
            cpu.r[3] = cpu.r[5];
            cpu.r[1] = cpu.r[6];
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(88)).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(8);
            cpu.r[1] <<= 2; // shll2
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
            cpu.r[6] = cpu.r[6].wrapping_add(1);
        }
        cpu.r[2] = cpu.r[5];
        cpu.r[0] = 92;
        cpu.r[3] = cpu.r[6];
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(92)).unwrap_or(0);
        cpu.r[3] <<= 2; // shll2
        cpu.r[2] = cpu.r[2].wrapping_add(8);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
        cpu.set_t(cpu.r[13] & cpu.r[13] == 0); // tst r13,r13
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        cpu.r[6] = cpu.r[6].wrapping_add(1); // bt.s delay slot
        if !cpu.t() {
            cpu.r[3] = cpu.r[5];
            cpu.r[0] = 96;
            cpu.r[2] = cpu.r[6];
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(96)).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(8);
            cpu.r[2] <<= 2; // shll2
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[2], cpu.r[1]); // bra delay slot
        }
    } else {
        // r14 == 0: [r4+92] directly
        cpu.r[0] = 92;
        cpu.r[3] = cpu.r[5];
        cpu.r[2] = cpu.r[6];
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(92)).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_add(8);
        cpu.r[2] <<= 2; // shll2
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
        let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
        cpu.r[6] = cpu.r[6].wrapping_add(1);
    }
    cpu.set_t(cpu.r[6] >> 30 & 1 != 0); // shll2 r6 at 0x8c2397f2 is the last T writer
    cpu.r[6] <<= 2; // shll2
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[6]);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
