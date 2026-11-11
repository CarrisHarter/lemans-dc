//! 0x8c185ba4 — initialize a 17-word record at r4 via a 64-byte stack buffer.
//!
//! After 4 pushes and r15 -= 64, the local buffer L = r15-80 gets 1.0f at
//! L+16..28 and L+48..60, 0.0f at L+0..12 and L+32..44. The record at r4 is
//! then filled: r4+0 = 0x8c1f0418 (pool 0x8c185c84), r4+4..16 = 1.0f,
//! r4+20..32 = 0.0f, r4+36..48 = 1.0f, r4+52..64 = 0.0f (in that store
//! order). Final regs: r0 = r4, r1 = 0, r2 = L+12, r3 = 0, r5 = L,
//! r6 = r4+20, r7 = L+16, r11..r14 restored, fr0 = 0.0, fr1 = 1.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let l = cpu.r[15].wrapping_sub(80); // 4 pushes (16) then add #-64,r15
    let r4 = cpu.r[4];
    let g = mem.read_u32(0x8c185c84).unwrap(); // pool: 0x8c1f0418
    let one = 1.0f32.to_bits();
    let zero = 0.0f32.to_bits();

    let _ = mem.write_u32(r4, g); // mov.l r0,@r4
    // stores to r4+36..48 from L+48..60 (all 1.0f)
    for i in 0..4u32 {
        let _ = mem.write_u32(r4 + 36 + i * 4, one);
    }
    // stores to r4+52..64 from L+32..44 (all 0.0f)
    for i in 0..4u32 {
        let _ = mem.write_u32(r4 + 52 + i * 4, zero);
    }
    // stores to r4+4..16 from L+16..28 (all 1.0f)
    for i in 0..4u32 {
        let _ = mem.write_u32(r4 + 4 + i * 4, one);
    }
    // stores to r4+20..32 from L+0..12 (all 0.0f)
    for i in 0..4u32 {
        let _ = mem.write_u32(r4 + 20 + i * 4, zero);
    }

    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = 0; // r1 = *(L+12) = 0.0f bits
    cpu.r[2] = l.wrapping_add(12); // mov #12,r2; add r15,r2
    cpu.r[3] = 0; // r3 = *(L+8) = 0.0f bits
    cpu.r[5] = l; // mov r15,r5
    cpu.r[6] = r4.wrapping_add(20); // mov r4,r6; add #20,r6
    cpu.r[7] = l.wrapping_add(16); // mov r15,r7; add #16,r7
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.fr[1] = 1.0; // fldi1 fr1
}
