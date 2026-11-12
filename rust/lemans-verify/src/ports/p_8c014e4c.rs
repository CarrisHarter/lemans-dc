//! 0x8c014e4c — load the 8-byte big-endian 5^i record, normalize it, and
//! report its position in bits.
//!
//! `r4` indexes the 8-byte table at `[0x8c014f00]` (0x8c1cd290, holding 5^i as
//! a big-endian 64-bit value) and 0x8c014ee4 copies that record into an 8-byte
//! scratch buffer. `*r5` is seeded with `r4 + 63`, the record's whole zero
//! bytes are skipped (8 subtracted from `*r5` each), then the leading zeros of
//! the first non-zero byte are counted and subtracted too. 0x8c014c1c
//! left-shifts the remaining `8 - zero_bytes` bytes so the mantissa's top bit
//! is set, and 0x8c010e84 copies them to `r6` (which was zero-filled first).
//!
//! Register side effects the C hides: `*r5` ends as
//! `r4 + 63 - 8*zero_bytes - zero_bits`, r0 is memmove's return (`r6`), r11
//! counts the skipped zero bytes, r12 the skipped zero bits, r13 is `r5`,
//! r14 walks the buffer, and `pr` is 0x8c014ec2 when the epilogue runs.
//! r8/r9/r10 are never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014e4c..0x8c014e56: push r14..r11 and pr, then 16 bytes of frame.
    for reg in [14, 13, 12, 11] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]); // mov.l rN,@-r15
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(16); // add #-16,r15

    cpu.r[3] = cpu.r[4].wrapping_add(63); // mov r4,r3 ; add #63,r3
    cpu.r[2] = mem.read_u32(0x8c014ed4).unwrap_or(0); // mov.l 0x8c014ed4,r2
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[5] = cpu.r[15]; // mov r15,r5
    let _ = mem.write_u32(cpu.r[15], cpu.r[6]); // mov.l r6,@r15
    let _ = mem.write_u16(cpu.r[13], cpu.r[3] as u16); // mov.w r3,@r13
    cpu.r[5] = cpu.r[5].wrapping_add(8); // delay slot: add #8,r5
    cpu.pr = 0x8c014e6a;
    crate::registry::p_8c014ee4::run(cpu, mem);

    cpu.r[3] = mem.read_u32(0x8c014ed8).unwrap_or(0); // mov.l 0x8c014ed8,r3
    cpu.r[5] = 0; // mov #0,r5
    cpu.r[6] = 8; // mov #8,r6
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot: mov.l @r15,r4
    cpu.pr = 0x8c014e74;
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r6, 0, 8)

    cpu.r[11] = 0; // mov #0,r11
    cpu.r[14] = cpu.r[15]; // mov r15,r14
    cpu.r[12] = cpu.r[11]; // mov r11,r12
    cpu.r[14] = cpu.r[14].wrapping_add(8); // delay slot of `bra`: add #8,r14

    // 0x8c014e88: skip whole zero bytes, one 8-bit step at a time.
    loop {
        cpu.r[2] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r14,r2
        cpu.set_t(cpu.r[2] == 0); // tst r2,r2
        if !cpu.t() {
            break; // bt taken only for a zero byte
        }
        cpu.r[3] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r13,r3
        cpu.r[11] = cpu.r[11].wrapping_add(1); // add #1,r11
        cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
        let _ = mem.write_u16(cpu.r[13], cpu.r[3].wrapping_sub(8) as u16); // add #-8,r3 ; mov.w r3,@r13
    }

    // 0x8c014e8e: count the leading zeros of the first non-zero byte.
    cpu.r[4] = mem.read_u16(0x8c014ed0).unwrap_or(0) as i16 as i32 as u32; // mov.w 0x8c014ed0,r4 (0x80)
    loop {
        cpu.r[2] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r14,r2
        cpu.r[3] = cpu.r[4] & 0xff; // extu.b r4,r3
        cpu.r[2] &= 0xff; // extu.b r2,r2
        cpu.set_t((cpu.r[3] & cpu.r[2]) == 0); // tst r3,r2
        if !cpu.t() {
            break; // bt 0x8c014e94 not taken
        }
        cpu.r[4] &= 0xff; // extu.b r4,r4
        cpu.r[12] = cpu.r[12].wrapping_add(1); // add #1,r12
        cpu.r[4] >>= 1; // shlr r4
    }

    cpu.r[5] = 8; // mov #8,r5
    cpu.r[3] = mem.read_u32(0x8c014edc).unwrap_or(0); // mov.l 0x8c014edc,r3
    cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[11]); // sub r11,r5
    cpu.r[6] = cpu.r[12]; // mov r12,r6
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r15)
    cpu.r[4] = cpu.r[14]; // delay slot: mov r14,r4
    cpu.pr = 0x8c014eb2;
    crate::registry::p_8c014c1c::run(cpu, mem); // left-shift the mantissa by r12

    cpu.r[2] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r13,r2
    cpu.r[5] = cpu.r[14]; // mov r14,r5
    cpu.r[3] = mem.read_u32(0x8c014ee0).unwrap_or(0); // mov.l 0x8c014ee0,r3
    cpu.r[2] = cpu.r[2].wrapping_sub(cpu.r[12]); // sub r12,r2
    let _ = mem.write_u16(cpu.r[13], cpu.r[2] as u16); // mov.w r2,@r13
    cpu.r[6] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r6
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot: mov.l @r15,r4
    cpu.pr = 0x8c014ec2;
    crate::registry::p_8c010e84::run(cpu, mem); // copy 8 - zero_bytes bytes to r6

    // 0x8c014ec2: pop the frame, then pr/r11/r12/r13 and the rts delay r14.
    cpu.r[15] = cpu.r[15].wrapping_add(16); // add #16,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
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
