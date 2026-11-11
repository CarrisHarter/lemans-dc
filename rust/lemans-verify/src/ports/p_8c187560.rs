//! 0x8c187560 — initialize an 88-byte object at r4 from two global tables.
//!
//! Original shape: `*r4 = 0x8c1f0498`, then 12 words copied from the table at
//! 0x8c1d3f68 into r4+4..12 and 16 words from the table at 0x8c1d0f28 into
//! r4+16..76 (four 16-byte blocks at +16/+32/+48/+64), fr1 (0.0) stored at
//! r4+80, byte 0 at r4+84; then `*r4` is overwritten with 0x8c1f0460 and
//! r4+80 with the literal float at 0x8c187668. Four floats are then pushed
//! through the stack frame (1.0 at [r15], then the literal at 0x8c18766c =
//! 0.125 at [r15+4/8/12]) and copied back over r4+16..28; finally the two
//! tables' first three words are re-stored at r4+4..12 and the 16-byte block
//! again at r4+32..44, and byte r4+84 is zeroed again.
//!
//! Exit state: r0 = r4, r1 = 0, r2 = r15_entry-32, r3 = r4+32,
//! r5 = 0x8c1d0f28, r6 = table2+8 word, r7 = 0x8c1d3f68, r8-r14 and r15
//! restored from the stack, fr0 = 0.125, fr1 = bits 0x0000f2ca, T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r15e = cpu.r[15];
    // push r14,r13,r12,r11,r10,r9,r8 then add #-16,r15
    let mut sp = r15e;
    for i in [14usize, 13, 12, 11, 10, 9, 8] {
        sp = sp.wrapping_sub(4);
        let _ = mem.write_u32(sp, cpu.r[i]);
    }
    let r15 = sp.wrapping_sub(16);

    let g1 = mem.read_u32(0x8c187658).unwrap_or(0); // 0x8c1f0498
    let r7 = mem.read_u32(0x8c18765c).unwrap_or(0); // 0x8c1d3f68
    let r5 = mem.read_u32(0x8c187660).unwrap_or(0); // 0x8c1d0f28
    let a0 = mem.read_u32(r7).unwrap_or(0);
    let a1 = mem.read_u32(r7.wrapping_add(4)).unwrap_or(0);
    let a2 = mem.read_u32(r7.wrapping_add(8)).unwrap_or(0);
    let b0 = mem.read_u32(r5).unwrap_or(0);
    let b1 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
    let b2 = mem.read_u32(r5.wrapping_add(8)).unwrap_or(0);
    let b3 = mem.read_u32(r5.wrapping_add(12)).unwrap_or(0);

    let _ = mem.write_u32(r4, g1); // mov.l r0,@r4
    let _ = mem.write_u32(r4.wrapping_add(4), a0);
    let _ = mem.write_u32(r4.wrapping_add(8), a1);
    let _ = mem.write_u32(r4.wrapping_add(12), a2);
    let _ = mem.write_u32(r4.wrapping_add(16), b0);
    let _ = mem.write_u32(r4.wrapping_add(20), b1);
    let _ = mem.write_u32(r4.wrapping_add(24), b2);
    let _ = mem.write_u32(r4.wrapping_add(28), b3);
    let _ = mem.write_u32(r4.wrapping_add(32), b0);
    let _ = mem.write_u32(r4.wrapping_add(36), b1);
    let _ = mem.write_u32(r4.wrapping_add(40), b2);
    let _ = mem.write_u32(r4.wrapping_add(44), b3);
    let _ = mem.write_u32(r4.wrapping_add(64), b0);
    let _ = mem.write_u32(r4.wrapping_add(68), b1);
    let _ = mem.write_u32(r4.wrapping_add(72), b2);
    let _ = mem.write_u32(r4.wrapping_add(76), b3);
    let _ = mem.write_u32(r4.wrapping_add(48), b0);
    let _ = mem.write_u32(r4.wrapping_add(52), b1);
    let _ = mem.write_u32(r4.wrapping_add(56), b2);
    let _ = mem.write_u32(r4.wrapping_add(60), b3);
    let _ = mem.write_u32(r4.wrapping_add(80), 0); // fmov fr1(0.0),@r12
    let _ = mem.write_u8(r4.wrapping_add(84), 0); // mov.b r1(0),@(r0,r4), r0=84
    let g2 = mem.read_u32(0x8c187664).unwrap_or(0); // 0x8c1f0460
    let _ = mem.write_u32(r4, g2);
    let fr1v = mem.read_f32(0x8c187668).unwrap_or(0.0); // mova 0x8c187668
    let _ = mem.write_f32(r4.wrapping_add(80), fr1v); // fmov fr1,@r12
    // stack round-trip: [r15] = fr0 (1.0 from fldi1), [r15+4/8/12] = fr0_new
    let _ = mem.write_f32(r15, 1.0); // fmov fr0,@r2 (r2 = r15)
    let fr0v = mem.read_f32(0x8c18766c).unwrap_or(0.0); // mova 0x8c18766c
    let _ = mem.write_f32(r15.wrapping_add(4), fr0v);
    let _ = mem.write_f32(r15.wrapping_add(8), fr0v);
    let _ = mem.write_f32(r15.wrapping_add(12), fr0v);
    let s0 = mem.read_u32(r15).unwrap_or(0);
    let s1 = mem.read_u32(r15.wrapping_add(4)).unwrap_or(0);
    let s2 = mem.read_u32(r15.wrapping_add(8)).unwrap_or(0);
    let s3 = mem.read_u32(r15.wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(16), s0);
    let _ = mem.write_u32(r4.wrapping_add(20), s1);
    let _ = mem.write_u32(r4.wrapping_add(24), s2);
    let _ = mem.write_u32(r4.wrapping_add(28), s3);
    let _ = mem.write_u32(r4.wrapping_add(4), a0); // re-read from r7
    let _ = mem.write_u32(r4.wrapping_add(8), a1);
    let _ = mem.write_u32(r4.wrapping_add(12), a2);
    let _ = mem.write_u32(r4.wrapping_add(32), b0); // re-read from r5
    let _ = mem.write_u32(r4.wrapping_add(36), b1);
    let _ = mem.write_u32(r4.wrapping_add(40), b2);
    let _ = mem.write_u32(r4.wrapping_add(44), b3);
    let _ = mem.write_u8(r4.wrapping_add(84), 0); // mov.b r1(0),@(r0,r4), r0=84

    // epilogue: add #16,r15; pop r8..r13; rts delay pops r14
    let mut sp2 = r15.wrapping_add(16);
    for i in [8usize, 9, 10, 11, 12, 13] {
        cpu.r[i] = mem.read_u32(sp2).unwrap_or(cpu.r[i]);
        sp2 = sp2.wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(sp2).unwrap_or(cpu.r[14]);

    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = 0;
    cpu.r[2] = r15.wrapping_add(12); // last: mov #12,r2; add r15,r2
    cpu.r[3] = r4.wrapping_add(32); // add #32,r3
    cpu.r[5] = r5;
    cpu.r[6] = b2; // mov.l @(8,r5),r6
    cpu.r[7] = r7;
    cpu.r[15] = r15e;
    cpu.fr[0] = fr0v;
    cpu.fr[1] = fr1v;
}
