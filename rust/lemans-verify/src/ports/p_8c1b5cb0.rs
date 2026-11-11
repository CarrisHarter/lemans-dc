//! 0x8c1b5cb0 — build a local 48-byte frame on the stack, then scatter it
//! and four ROM constants into the struct at r4.
//!
//! Frame layout (r15 after `add #-48,r15` = sp):
//!   [sp+32] = 1.0f (fldi1), [sp+36..48] = f32[0x8c1b5d88] x3
//!   [sp+16] = 1.0f, [sp+20..28] = f32[0x8c1b5d8c] x3
//!   [sp+0..16]  = 1.0f, f32[0x8c1b5d94] x3
//! Stores to r4: [r4+16..28) <- u32[sp+32..44), [r4+0..16) <- u32[sp+16..28),
//! f32[0x8c1b5d90] -> [r4+32], f32[0x8c1b5d94] -> [r4+36],
//! f32[0x8c1b5d98] -> [r4+40], f32[0x8c1b5d9c] -> [r4+44],
//! [r4+48..64) <- u32[sp+0..16).
//! r10..r14 are pushed and popped, r15 restored. Finals: r0 = r4,
//! r1 = u32[sp+12], r2 = sp+12, r3 = u32[sp+8], r5 = sp, r6 = r4+48,
//! r7 = r4+40, fr0 = f32[0x8c1b5d9c], fr1 = f32[0x8c1b5d94], fr2 = 1.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = [cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14]];
    let r4 = cpu.r[4];
    // 5 pushes (r14,r13,r12,r11,r10) + add #-48,r15: frame starts 68 below entry
    let sp = cpu.r[15].wrapping_sub(20).wrapping_sub(48);

    cpu.fr[2] = 1.0; // fldi1
    mem.write_f32(sp + 32, cpu.fr[2]).unwrap();
    cpu.fr[0] = mem.read_f32(0x8C1B5D88).unwrap();
    mem.write_f32(sp + 36, cpu.fr[0]).unwrap();
    mem.write_f32(sp + 40, cpu.fr[0]).unwrap();
    mem.write_f32(sp + 44, cpu.fr[0]).unwrap();

    let r10 = sp + 32;
    cpu.r[0] = mem.read_u32(r10).unwrap();
    let r1 = r4 + 16;
    cpu.r[5] = mem.read_u32(r10 + 4).unwrap();
    let r13 = r4 + 32;
    mem.write_u32(r1, cpu.r[0]).unwrap();
    let r14 = r4 + 36;
    mem.write_u32(r1 + 4, cpu.r[5]).unwrap();
    let r7 = r4 + 40;
    let r12 = mem.read_u32(r10 + 8).unwrap();
    let r3 = r4 + 44;
    cpu.r[5] = mem.read_u32(r10 + 12).unwrap();
    mem.write_u32(r1 + 8, r12).unwrap();
    let r6 = r4 + 48;
    mem.write_u32(r1 + 12, cpu.r[5]).unwrap();

    let r11 = sp + 16;
    mem.write_f32(sp + 16, cpu.fr[2]).unwrap();
    cpu.fr[0] = mem.read_f32(0x8C1B5D8C).unwrap();
    mem.write_f32(sp + 20, cpu.fr[0]).unwrap();
    mem.write_f32(sp + 24, cpu.fr[0]).unwrap();
    mem.write_f32(sp + 28, cpu.fr[0]).unwrap();
    cpu.r[12] = mem.read_u32(r11).unwrap();
    cpu.r[1] = mem.read_u32(r11 + 4).unwrap();
    mem.write_u32(r4, cpu.r[12]).unwrap();
    cpu.r[5] = sp;
    mem.write_u32(r4 + 4, cpu.r[1]).unwrap();
    cpu.r[12] = mem.read_u32(r11 + 8).unwrap();
    cpu.r[1] = mem.read_u32(r11 + 12).unwrap();
    mem.write_u32(r4 + 8, cpu.r[12]).unwrap();
    mem.write_u32(r4 + 12, cpu.r[1]).unwrap();

    cpu.fr[0] = mem.read_f32(0x8C1B5D90).unwrap();
    mem.write_f32(r13, cpu.fr[0]).unwrap();
    cpu.fr[1] = mem.read_f32(0x8C1B5D94).unwrap();
    mem.write_f32(r14, cpu.fr[1]).unwrap();
    cpu.fr[0] = mem.read_f32(0x8C1B5D98).unwrap();
    mem.write_f32(r7, cpu.fr[0]).unwrap();
    cpu.fr[0] = mem.read_f32(0x8C1B5D9C).unwrap();
    mem.write_f32(r3, cpu.fr[0]).unwrap();
    cpu.r[0] = r4;
    mem.write_f32(sp, cpu.fr[2]).unwrap();
    mem.write_f32(sp + 4, cpu.fr[1]).unwrap();
    mem.write_f32(sp + 8, cpu.fr[1]).unwrap();
    mem.write_f32(sp + 12, cpu.fr[1]).unwrap();
    cpu.r[3] = mem.read_u32(sp).unwrap();
    cpu.r[1] = mem.read_u32(sp + 4).unwrap();
    mem.write_u32(r6, cpu.r[3]).unwrap();
    mem.write_u32(r6 + 4, cpu.r[1]).unwrap();
    cpu.r[3] = mem.read_u32(sp + 8).unwrap();
    cpu.r[1] = mem.read_u32(sp + 12).unwrap();
    mem.write_u32(r6 + 8, cpu.r[3]).unwrap();
    mem.write_u32(r6 + 12, cpu.r[1]).unwrap();

    cpu.r[2] = sp + 12;
    cpu.r[5] = sp;
    cpu.r[15] = sp.wrapping_add(68);
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[10] = saved[0];
    cpu.r[11] = saved[1];
    cpu.r[12] = saved[2];
    cpu.r[13] = saved[3];
    cpu.r[14] = saved[4];
}
