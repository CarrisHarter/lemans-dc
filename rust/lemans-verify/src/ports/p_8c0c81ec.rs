//! 0x8c0c81ec — LFSR warm-up: fills 256 words at 0x8c27a2a8 (Galois LFSR,
//! taps 0x80000000 / 0x04c11db7, stride 0x01000000), guarded by the two
//! one-byte flags at *0x8c27a6b0 and *0x8c27a6a8.
//!
//! The C hides the register residue: on the full path r4 ends as the last
//! inner-loop feedback word (0 or 0x04c11db7), r5 = 0, r6 = 256, r7 = 8,
//! r0 = 0x8c27a2a8 (table pointer stays live), r1 = 8, r3 = 0, and T = 0
//! (final cmp/gt 256 > 256). r8..r14 are pushed and popped, so they keep
//! their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = [
        cpu.r[8], cpu.r[9], cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14],
    ];
    let r4 = mem.read_u32(0x8c0c826c).unwrap_or(0); // literal -> 0x8c27a6b0
    let b0 = mem.read_u8(r4).unwrap_or(0); // mov.b @r4,r0 ; exts.b
    if b0 == 0 {
        // 8c0c8204: clear the second flag, set the first
        let r3 = mem.read_u32(0x8c0c8270).unwrap_or(0); // literal -> 0x8c27a6a8
        let _ = mem.write_u8(r3, 0); // mov.b r1,@r3 (r1 = 0)
        let _ = mem.write_u8(r4, 1); // mov.b r0,@r4 (r0 = 1)
        cpu.r[3] = r3;
        cpu.r[0] = 1;
    }
    let r1 = mem.read_u32(0x8c0c8270).unwrap_or(0);
    let b1 = mem.read_u8(r1).unwrap_or(0); // mov.b @r1,r0 ; extu.b
    cpu.r[1] = r1;
    cpu.r[0] = b1 as u32;
    if b1 == 0 {
        let _ = mem.write_u8(r1, 1); // mov.b r0,@r1 (r0 = 1)
        cpu.r[0] = 1;
        let r13 = mem.read_u32(0x8c0c8274).unwrap_or(0); // 0x80000000
        let r12 = mem.read_u32(0x8c0c8278).unwrap_or(0); // 0x04c11db7
        let r9 = mem.read_u32(0x8c0c827c).unwrap_or(0); // 0x100
        let r0 = mem.read_u32(0x8c0c8280).unwrap_or(0); // 0x8c27a2a8
        let r8 = mem.read_u32(0x8c0c8284).unwrap_or(0); // 0x01000000
        let r3: u32 = 0; // mov r6,r3 (r6 = 0)
        let r5: u32 = 0; // mov r6,r5
        let r1c: u32 = 8; // mov #8,r1
        let mut r6: u32 = 0;
        let mut r11: u32 = 0; // mov r6,r11
        let mut r10: u32 = 0; // mov r6,r10
        let mut r7: u32 = 0;
        let mut r4l: u32 = 0;
        let mut r14: u32 = 0;
        loop {
            r14 = r11; // mov r11,r14
            r7 = r5; // mov r5,r7
            loop {
                r4l = r14 & r13; // mov r14,r4 ; and r13,r4
                r4l = if r4l == 0 { r3 } else { r12 }; // tst + bt/bra select
                r14 = r14 << 1; // shll r14
                r7 = r7.wrapping_add(1); // add #1,r7
                let t = (r1c as i32) > (r7 as i32); // cmp/gt r7,r1
                r14 ^= r4l; // bt.s delay slot: xor r4,r14 (runs every iteration)
                if !t {
                    break;
                }
            }
            let _ = mem.write_u32(r0.wrapping_add(r10), r14); // mov.l r14,@(r0,r10)
            r6 = r6.wrapping_add(1); // add #1,r6
            let t = (r9 as i32) > (r6 as i32); // cmp/gt r6,r9
            r11 = r11.wrapping_add(r8); // add r8,r11
            r10 = r10.wrapping_add(4); // bt.s delay slot: add #4,r10 (runs every iteration)
            cpu.set_t(t);
            if !t {
                break;
            }
        }
        cpu.r[0] = r0; // mov.l 0x8c0c8280,r0 stays live -> 0x8c27a2a8
        cpu.r[1] = r1c; // mov #8,r1 stays live
        cpu.r[3] = r3;
        cpu.r[4] = r4l;
        cpu.r[5] = r5;
        cpu.r[6] = r6;
        cpu.r[7] = r7;
    } else {
        cpu.set_t(false); // tst at 8c0c8216 left T = 0; bf taken to the epilogue
    }
    cpu.r[8] = saved[0];
    cpu.r[9] = saved[1];
    cpu.r[10] = saved[2];
    cpu.r[11] = saved[3];
    cpu.r[12] = saved[4];
    cpu.r[13] = saved[5];
    cpu.r[14] = saved[6];
}
