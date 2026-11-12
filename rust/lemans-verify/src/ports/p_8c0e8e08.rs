//! 0x8c0e8e08 — push a colour/material slot into a TA-style display list.
//!
//! CFG (no loops; branches are plain `bt/bf` except `bt.s` at 0x8c0e8e24 whose
//! delay slot `add r6,r1` runs on BOTH paths):
//!
//! ```text
//! r1 = 0x8c1d6e00 + r5*12 ; r3 = *0x8c27a950 (list head) ; r0 = [r4+0x74]
//! cmp/eq r0,r5 ; bt.s skip_update (delay: add r6,r1)
//!   update: [r4+0x74]=r5 ; *r3 &= 0x0ff300c0 ; r3[1] &= 0x7ffc0000
//!           *r3 |= r1[0] ; r3[1] |= r1[1] ; *r3 |= 4 ; r3[1] |= 0x01000000
//! skip_update: r0 = [r4+0x78] ; r1 = r1[2] (mode)
//!   if (r1 == r0) { r0 = extu.b [r4+0x70] ; if (r0 == 0) goto exit }
//!   if (r1 == 0) goto exit
//!   mode 1 -> ftrc-pack 8 floats from r4+0x50.. into r3[4..5]
//!   mode 2 -> copy r4+0x50..0x5c into r3[4..7]   (bra, delay stores r3[7])
//!   mode 3 -> copy r4+0x50..0x6c into r3[8..15]  (bra, delay stores r3[15])
//!   other  -> bra tail (nop delay)
//! tail: [r4+0x78] = r1 ; byte [r4+0x70] = 0
//! exit: pop r12,r13 ; rts ; pop r14 (delay slot)
//! ```
//!
//! Side effects the C hides: r5 is clobbered to 0x01000000 on the update path,
//! r6 ends as a masked list word (update path) or stays 0x8c1d6e00 (skip path),
//! r1 ends as the mode on early exits but 0 on the tail path, r0 ends as
//! 112 on the tail path but the tested mode/byte on early exits, T is the last
//! cmp/eq (#1/#2/#3) or tst, and the `bra` delay slots perform the final
//! `mov.l r0,@(12,r3)` stores of modes 2 and 3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14/r13/r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);

    // 8c0e8e0e..8c0e8e26
    cpu.r[0] = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8C0E8F68).unwrap(); // 0x8c27a950
    cpu.r[1] = cpu.r[5];
    cpu.r[0] = cpu.r[0].wrapping_add(116);
    cpu.r[1] = cpu.r[1] << 1; // shll r1
    let cached74 = mem.read_u32(cpu.r[0]).unwrap(); // [r4+0x74]
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]);
    cpu.r[6] = mem.read_u32(0x8C0E8F6C).unwrap(); // 0x8c1d6e00
    cpu.r[1] = cpu.r[1] << 2; // shll2 r1  (r1 = r5*12)
    cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap(); // r3 = list head
    cpu.set_t(cpu.r[5] == cached74); // cmp/eq r0,r5
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[6]); // delay slot of bt.s — always
    if !cpu.t() {
        // 8c0e8e28..8c0e8e58 — update path
        cpu.r[0] = 116;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(116), cpu.r[5]);
        cpu.r[6] = mem.read_u32(0x8C0E8F70).unwrap(); // 0x0ff300c0
        cpu.r[7] = mem.read_u32(cpu.r[3]).unwrap();
        let r0b = mem.read_u32(0x8C0E8F74).unwrap(); // 0x7ffc0000
        cpu.r[7] &= cpu.r[6];
        cpu.r[5] = mem.read_u32(0x8C0E8F78).unwrap(); // 0x01000000
        let _ = mem.write_u32(cpu.r[3], cpu.r[7]);
        // These loads target r6 itself (0x8c0e8e38/3e/46); shadowing them in
        // locals left cpu.r[6] at its entry value and only showed on the
        // update path when r6 is not overwritten later.
        cpu.r[6] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap() & r0b;
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[6]);
        cpu.r[6] = mem.read_u32(cpu.r[3]).unwrap();
        let r0c = mem.read_u32(cpu.r[1]).unwrap();
        cpu.r[6] |= r0c;
        let _ = mem.write_u32(cpu.r[3], cpu.r[6]);
        cpu.r[6] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap();
        let r0d = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap();
        cpu.r[6] |= r0d;
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[6]);
        let r0e = mem.read_u32(cpu.r[3]).unwrap() | 4;
        let _ = mem.write_u32(cpu.r[3], r0e);
        let r0f = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap() | cpu.r[5];
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), r0f);
    }
    // 8c0e8e5a..8c0e8e62
    cpu.r[0] = cpu.r[4].wrapping_add(120);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // r0 = [r4+0x78]
    cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap(); // mode = r1[2]
    cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r0,r1
    let mut exit_now = false;
    if cpu.t() {
        cpu.r[0] = cpu.r[4].wrapping_add(112);
        cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap() as u32;
        cpu.r[0] &= 0xFF; // extu.b r0,r0 (twice)
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            exit_now = true; // bt 0x8c0e8f60
        }
    }
    if !exit_now {
        cpu.set_t(cpu.r[1] == 0); // tst r1,r1
        if cpu.t() {
            exit_now = true; // bt 0x8c0e8f60
        }
    }
    if !exit_now {
        cpu.r[0] = cpu.r[1];
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.t() {
            mode1(cpu, mem);
        } else {
            cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
            if cpu.t() {
                // mode 2: 8c0e8e8a..8c0e8ea0
                cpu.r[6] = cpu.r[4].wrapping_add(80);
                cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap();
                cpu.r[3] = cpu.r[3].wrapping_add(16);
                cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap();
                let _ = mem.write_u32(cpu.r[3], cpu.r[5]);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
                cpu.r[5] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap();
                cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap();
                let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[5]);
                // delay slot of bra 0x8c0e8f56
                let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[0]);
            } else {
                cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
                if cpu.t() {
                    // mode 3: 8c0e8ea2..8c0e8ece
                    cpu.r[7] = cpu.r[4].wrapping_add(80);
                    cpu.r[5] = mem.read_u32(cpu.r[7]).unwrap();
                    cpu.r[6] = cpu.r[4];
                    cpu.r[0] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap();
                    cpu.r[3] = cpu.r[3].wrapping_add(32);
                    cpu.r[6] = cpu.r[6].wrapping_add(96);
                    let _ = mem.write_u32(cpu.r[3], cpu.r[5]);
                    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
                    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap();
                    cpu.r[0] = mem.read_u32(cpu.r[7].wrapping_add(12)).unwrap();
                    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[5]);
                    let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[0]);
                    cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap();
                    cpu.r[3] = cpu.r[3].wrapping_add(16);
                    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap();
                    let _ = mem.write_u32(cpu.r[3], cpu.r[5]);
                    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
                    cpu.r[5] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap();
                    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap();
                    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[5]);
                    // delay slot of bra 0x8c0e8f56
                    let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[0]);
                }
                // else: bra 0x8c0e8f56 with nop delay — nothing to do
            }
        }
        // tail: 8c0e8f56..8c0e8f5e
        cpu.r[0] = 120;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(120), cpu.r[1]);
        cpu.r[1] = 0;
        cpu.r[0] = 112;
        let _ = mem.write_u8(cpu.r[4].wrapping_add(112), cpu.r[1] as u8);
    }
    // exit: 8c0e8f60..8c0e8f66
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

/// Mode 1 (8c0e8ed0..8c0e8f54): multiply eight floats at r4+0x50..0x6c by the
/// constant at 0x8c0e8f7c (255.0), `ftrc` each to an integer and pack the four
/// byte-truncated values into r3[4] and r3[5] (big-endian byte order).
fn mode1(cpu: &mut Cpu, mem: &mut Memory) {
    let r7 = cpu.r[4].wrapping_add(80);
    cpu.r[0] = 0x8C0E8F7C; // mova 0x8c0e8f7c,r0
    cpu.fr[0] = mem.read_f32(r7).unwrap();
    cpu.r[13] = cpu.r[4].wrapping_add(84);
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[6] = cpu.r[4].wrapping_add(88);
    cpu.r[5] = cpu.r[4].wrapping_add(92);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3]; // fmul fr3,fr0
    cpu.r[7] = cpu.r[4];
    cpu.r[3] = cpu.r[3].wrapping_add(16);
    cpu.fr[2] = mem.read_f32(cpu.r[13]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc fr0,fpul
    cpu.fr[2] = cpu.fr[2] * cpu.fr[3];
    cpu.r[12] = cpu.fpul; // sts fpul,r12
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[12] = cpu.r[12] << 16; // shll16 r12
    cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc fr2,fpul
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3];
    cpu.r[13] = cpu.fpul; // sts fpul,r13
    cpu.fpul = cpu.fr[1] as i32 as u32; // ftrc fr1,fpul
    cpu.r[12] = cpu.r[12] << 8; // shll8 r12
    cpu.r[13] = cpu.r[13] << 16; // shll16 r13
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3];
    cpu.r[14] = cpu.fpul; // sts fpul,r14
    cpu.r[12] |= cpu.r[13];
    cpu.r[14] = cpu.r[14] << 8; // shll8 r14
    cpu.r[6] = cpu.r[4];
    cpu.r[5] = cpu.r[4];
    cpu.r[0] = cpu.r[4];
    cpu.r[12] |= cpu.r[14];
    cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc fr0,fpul
    cpu.r[7] = cpu.r[7].wrapping_add(96);
    cpu.r[6] = cpu.r[6].wrapping_add(100);
    cpu.r[5] = cpu.r[5].wrapping_add(104);
    cpu.r[14] = cpu.fpul; // sts fpul,r14
    cpu.r[12] |= cpu.r[14];
    let _ = mem.write_u32(cpu.r[3], cpu.r[12]); // mov.l r12,@r3
    cpu.r[0] = cpu.r[0].wrapping_add(108);
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap();
    cpu.fr[2] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3];
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fpul = cpu.fr[0] as i32 as u32;
    cpu.fr[2] = cpu.fr[2] * cpu.fr[3];
    cpu.r[6] = cpu.fpul; // sts fpul,r6
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[6] = cpu.r[6] << 16; // shll16 r6
    cpu.fpul = cpu.fr[2] as i32 as u32;
    cpu.fr[1] = cpu.fr[1] * cpu.fr[3];
    cpu.r[5] = cpu.fpul; // sts fpul,r5
    cpu.fpul = cpu.fr[1] as i32 as u32;
    cpu.r[6] = cpu.r[6] << 8; // shll8 r6
    cpu.r[5] = cpu.r[5] << 16; // shll16 r5
    cpu.fr[0] = cpu.fr[0] * cpu.fr[3];
    cpu.r[0] = cpu.fpul; // sts fpul,r0
    cpu.r[6] |= cpu.r[5];
    cpu.r[0] = cpu.r[0] << 8; // shll8 r0
    cpu.r[6] |= cpu.r[0];
    cpu.fpul = cpu.fr[0] as i32 as u32;
    cpu.r[0] = cpu.fpul; // sts fpul,r0
    cpu.r[6] |= cpu.r[0];
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[6]); // mov.l r6,@(4,r3)
}
