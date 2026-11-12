//! 0x8c0614b8 — initialise a render/work block and allocate its two buffers.
//!
//! r4 = block base P, r5 = param_2 (stored at P+20), r6 = param_3 / size B
//! (stored at P+0x268 and P+48). Zeroes P+0x260/P+0x264/P+0x268 first, then
//! writes the header fields P+0..P+0x2c, including the optional flag bit
//! ORs of 0x100 (byte at 0x8c23ecb0) and 0x40 (byte at 0x8c23ecb8).
//!
//! Calls the allocator at 0x8c15a068 twice (static pointer from the literal
//! pool): first for `24*[0x8c1f7a38] + 76*[0x8c1f7a40] + 31` (the `mul.l`
//! result lands in MACL and the jsr delay slot adds the 31), then for
//! `B + 31`. The callee aligns both requests itself and returns a pointer in
//! r0 (0 on failure); the first return is stored at P+0x260, aligned up to 32
//! and kept at P+32, and the second return goes to P+0x264 and its aligned
//! value to P+44 (masked to 0xa0000000 when the byte at 0x8c1f7a48 is set).
//!
//! Finally the 56-byte record at P+56 is filled once (the `bt.s` trip count
//! is constant: r13 increments 0→1 while r14 = 1), copying five floats from
//! 0x8c1f7a18..0x8c1f7a28 and writing the constants at P+68..P+108.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // --- prologue: push r14, r13, r12, r11, pr, then one 4-byte local ---
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[6]; // mov r6,r12 — param_3
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // local slot holds P

    // --- 0x8c0614c8: zero P+0x260..0x268, write the fixed header fields ---
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // P
    cpu.r[7] = 0;
    cpu.r[0] = 0x260;
    cpu.r[3] = 49;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[7]);
    cpu.r[4] = 0x1f8;
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 0x264;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), 0);
    cpu.r[1] = 0x8c23_ecb0; // flag byte 1
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 0x268;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[6]), 0);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // P+0 = 0x1f8
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[3]); // P+4 = 0x31
    cpu.r[0] = mem.read_u8(cpu.r[1]).unwrap_or(0) as u32;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        // 0x8c0614f4: P+4 |= 0x100
        cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[0] = 0x100;
        cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
        cpu.r[1] |= cpu.r[0];
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[1]);
    }

    // 0x8c0614fe: flag byte 2 at 0x8c23ecb8
    cpu.r[0] = 0x8c23_ecb8;
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
        cpu.r[0] |= 0x40;
        let _ = mem.write_u32(cpu.r[1].wrapping_add(4), cpu.r[0]);
    }

    // --- 0x8c061512: linkage fields, then the first allocator call ---
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0); // P
    cpu.r[7] = 2;
    cpu.r[1] = 0x1f8;
    cpu.r[3] = cpu.r[6];
    cpu.r[4] = 0x200;
    cpu.r[0] = cpu.r[1];
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[6]), cpu.r[3]); // P+0x1f8
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = 0x220;
    cpu.r[3] = cpu.r[6];
    cpu.r[0] = 0x1fc;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[6]), cpu.r[3]); // P+0x1fc
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = 76;
    cpu.r[6] = 0x8c1f_7a40; // &m28
    cpu.r[0] = cpu.r[11];
    cpu.r[4] = 0x8c1f_7a38; // &m24
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(cpu.r[11].wrapping_add(8), cpu.r[0]); // P+8 = P+0x1f8
    cpu.r[0] = 0x8c15_a068; // allocator
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1].wrapping_add(12), cpu.r[7]); // P+12 = 2
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1].wrapping_add(20), cpu.r[5]); // P+20 = param_2
    cpu.r[6] = mem.read_u32(cpu.r[6]).unwrap_or(0); // m28
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // P
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // m24
    let _ = mem.write_u32(cpu.r[5].wrapping_add(24), cpu.r[1]); // P+24 = m24
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1].wrapping_add(28), cpu.r[6]); // P+28 = m28
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(28)).unwrap_or(0);
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
    cpu.macl = cpu.r[3].wrapping_mul(cpu.r[1]); // mul.l r3,r1 (76 * m28)
    cpu.r[1] = cpu.macl;
    cpu.r[4] = cpu.r[5]; // 24 * m24
    cpu.r[4] = cpu.r[4] << 1;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4] << 1;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
    cpu.pr = 0x8c06_156c;
    cpu.r[4] = cpu.r[4].wrapping_add(31); // delay slot: add #31,r4
    crate::registry::p_8c15a068::run(cpu, mem);

    // --- 0x8c06156c: first return value, alignment, second call site ---
    cpu.r[3] = cpu.r[0];
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[12]; // param_3
    cpu.r[0] = 0x260;
    cpu.r[5] = 0xffff_ffe0;
    cpu.r[4] = cpu.r[4].wrapping_add(31);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[3]); // P+0x260
    cpu.r[7] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[6] = 0x240;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0);
    cpu.r[3] = 0x8c1f_7a30;
    cpu.r[0] = cpu.r[0].wrapping_add(31);
    cpu.r[1] = 0x8c15_a068;
    cpu.r[0] &= cpu.r[5];
    let _ = mem.write_u32(cpu.r[7].wrapping_add(32), cpu.r[0]); // P+32
    cpu.r[7] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 0x268;
    cpu.r[5] = cpu.r[7];
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
    let _ = mem.write_u32(cpu.r[7].wrapping_add(36), cpu.r[5]); // P+36 = P+0x240
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(40), cpu.r[3]); // P+40
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.pr = 0x8c06_15a0;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[12]); // P+0x268 = B
    crate::registry::p_8c15a068::run(cpu, mem);

    // --- 0x8c0615a0: second return value and alignment ---
    cpu.r[4] = cpu.r[0];
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = 0xffff_ffe0;
    cpu.r[0] = 0x264;
    let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4]); // P+0x264
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[3] = cpu.r[3].wrapping_add(31);
    cpu.r[3] &= cpu.r[1];
    cpu.r[0] = 0x8c1f_7a48; // flag byte 3
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[0] = 0x0fff_ffff;
        cpu.r[3] &= cpu.r[0];
        cpu.r[0] = 0xa000_0000;
        cpu.r[3] |= cpu.r[0];
    }
    let _ = mem.write_u32(cpu.r[4].wrapping_add(44), cpu.r[3]); // P+44

    // --- 0x8c0615c8: fixed fields and the 56-byte record fill ---
    cpu.r[13] = 0;
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[14] = 1;
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[1].wrapping_add(48), cpu.r[12]); // P+48 = B
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1].wrapping_add(52), cpu.r[14]); // P+52 = 1
    cpu.r[6] = 0x8c1f_7a18;
    cpu.r[7] = cpu.r[13];
    cpu.r[11] = 3;
    cpu.r[5] = cpu.r[6];
    cpu.r[4] = cpu.r[6];
    cpu.r[3] = cpu.r[6];
    cpu.r[1] = cpu.r[6];
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.r[3] = cpu.r[3].wrapping_add(12);
    cpu.r[1] = cpu.r[1].wrapping_add(16);
    loop {
        // 0x8c0615ec: the body runs exactly once (r14 = 1 is never > r13 = 1)
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        let t = (cpu.r[14] as i32) > (cpu.r[13] as i32); // cmp/gt r13,r14
        cpu.set_t(t);
        cpu.r[12] = cpu.r[12].wrapping_add(56);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[7]); // +56
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(60);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[7]); // +60
        for (off, src) in [
            (0u32, 0x8c1f_7a18u32),
            (4, 0x8c1f_7a1c),
            (8, 0x8c1f_7a20),
            (12, 0x8c1f_7a24),
            (16, 0x8c1f_7a28),
        ] {
            cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.fr[0] = mem.read_f32(src).unwrap_or(0.0);
            cpu.r[12] = cpu
                .r[12]
                .wrapping_add(72)
                .wrapping_add(cpu.r[0])
                .wrapping_add(off);
            let _ = mem.write_f32(cpu.r[12], cpu.fr[0]);
        }
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(68);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[7]); // +68
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(92);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[7]); // +92
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(96);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[14]); // +96 = 1
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(100);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[11]); // +100 = 3
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(104);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[14]); // +104 = 1
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(108);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[11]); // +108 = 3
        // The instruction after a delayed branch is executed whether or not
        // the branch is taken; only the next PC differs.
        cpu.r[0] = cpu.r[0].wrapping_add(56); // add #56,r0 at 0x8c06165c
        if !t {
            break;
        }
    }

    // --- 0x8c06165e: epilogue ---
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
