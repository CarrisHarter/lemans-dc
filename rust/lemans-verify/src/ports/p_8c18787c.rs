//! 0x8c18787c — initialize an 84-byte object at r4 from two global tables.
//!
//! Original shape: `*r4 = 0x8c1f0498`; the 0x8c1d3f68 table's first three
//! words go to r4+4..12; the 0x8c1d0f28 table's 16 bytes are copied four times
//! to r4+16/+32/+48/+64 (as four u32s each); fr0 is zeroed (`fldi0`) and
//! stored at r4+80; byte r4+84 is zeroed (`mov.b r3,@(r0,r4)` with r0 = 84,
//! r3 = 0). r11-r14 are saved and restored (r14 in the `rts` delay slot).
//!
//! Exit state: r0 = r4, r1 = table2+12 word, r3 = 0, r5 = r4+80,
//! r6 = table2+8 word, r7 = table2+0 word, r11-r15 restored, fr0 = 0.0,
//! T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r15e = cpu.r[15];
    // push r14,r13,r12,r11
    let mut sp = r15e;
    for i in [14usize, 13, 12, 11] {
        sp = sp.wrapping_sub(4);
        let _ = mem.write_u32(sp, cpu.r[i]);
    }

    let g1 = mem.read_u32(0x8c187908).unwrap_or(0); // 0x8c1f0498
    let r3 = mem.read_u32(0x8c18790c).unwrap_or(0); // 0x8c1d3f68
    let r1 = mem.read_u32(0x8c187910).unwrap_or(0); // 0x8c1d0f28
    let a0 = mem.read_u32(r3).unwrap_or(0);
    let a1 = mem.read_u32(r3.wrapping_add(4)).unwrap_or(0);
    let a2 = mem.read_u32(r3.wrapping_add(8)).unwrap_or(0);
    let b0 = mem.read_u32(r1).unwrap_or(0);
    let b1 = mem.read_u32(r1.wrapping_add(4)).unwrap_or(0);
    let b2 = mem.read_u32(r1.wrapping_add(8)).unwrap_or(0);
    let b3 = mem.read_u32(r1.wrapping_add(12)).unwrap_or(0);

    let _ = mem.write_u32(r4, g1);
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
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(r4.wrapping_add(80), 0.0); // fmov fr0,@r5 (r5 = r4+80)
    let _ = mem.write_u8(r4.wrapping_add(84), 0); // mov.b r3(0),@(r0,r4), r0 = 84

    // pops: r11, r12, r13 now; r14 in the rts delay slot
    let mut sp2 = sp;
    for i in [11usize, 12, 13] {
        cpu.r[i] = mem.read_u32(sp2).unwrap_or(cpu.r[i]);
        sp2 = sp2.wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(sp2).unwrap_or(cpu.r[14]);

    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = b3; // mov.l @(12,r1),r1
    cpu.r[3] = 0; // mov #0,r3
    cpu.r[5] = r4.wrapping_add(80); // add #80,r5
    cpu.r[6] = b2; // mov.l @(8,r1),r6
    cpu.r[7] = b0; // mov.l @r1,r7
    cpu.r[15] = r15e;
}
