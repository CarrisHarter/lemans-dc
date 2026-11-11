//! 0x8c06fdc0 — large struct-shuffle leaf, 258 instructions, straight-line
//! (no branches, no loops; Ghidra's FPSCR_SZ split is not real control flow,
//! the oracle always runs the single-precision path).
//!
//! CFG: prologue (push r14..r8, r15 -= 48) -> one basic block of mov.l
//! copies chasing pointers through a 48-byte stack frame -> epilogue
//! (r15 += 48, pop r8..r13, rts, delay slot pops r14). One path only.
//!
//! r4 = dest struct, r5 = src struct. The body copies words from src (r5+0
//! .. r5+0xd8) into scattered dest offsets (r4+4..0xf8), using the stack
//! frame at r15+0..r15+44 as a pointer scratchpad, then two fmov copies:
//! [r4+0x58] = f32[r5+0xbc], [r4+0x5c] = f32[r5+0xc0], then three more
//! copies from src+0xc4/0xcc/0xd0 into r4+0xb0.. and (r4+0xbc)+0/4/8.
//! Literal-pool offsets (image at 0x8c06ffc4..0x8c06ffe8): 0x80, 0xcc,
//! 0x8c, 0x98, 0xd8, 0xbc, 0xc0, 0xb0, 0xc4, 0xd0.
//!
//! Hidden side effects the decompiled C does not show:
//! - r4 and r5 are *advanced* at the end (r4 += 0xbc at 8c06ff96,
//!   r5 += 0xd0 at 8c06ff9a); the last three stores land at (old r4)+0xbc..,
//!   not at the base the C suggests.
//! - leftover working registers: r0 = src[0xd8], r1 = src[0xd0],
//!   r2 = r15_entry-44 (r15+32 of the final frame), r3 = src[0xc8],
//!   r6 = src[0xc4], r7 = old r5+0xc4, r8 = old r4+0xb0, r9 = old r4+60,
//!   r10 = old r4+0xd8; r11..r14 are restored by the pops.
//! - only fr0 is clobbered (ends as f32[old r5+0xc0]); fr1..fr15 untouched.
//! - no flag op runs, so T/sr are unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14/r13/r12/r11/r10/r9/r8,@-r15; add #-48,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(48);

    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap();
    cpu.r[3] = cpu.r[4];
    cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap();
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[9] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]).unwrap();
    cpu.r[10] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[3] + 4, cpu.r[1]).unwrap();
    cpu.r[0] = cpu.r[4];
    cpu.r[1] = mem.read_u32(cpu.r[5] + 8).unwrap();
    cpu.r[2] = cpu.r[15]; // 8c06fde4
    cpu.r[9] = cpu.r[9].wrapping_add(12); // 8c06fde6
    let _ = mem.write_u32(cpu.r[3] + 8, cpu.r[1]).unwrap(); // 8c06fde8: [r4+12] = [r5+8]
    cpu.r[14] = cpu.r[5]; // 8c06fdea
    cpu.r[3] = mem.read_u32(cpu.r[9]).unwrap(); // 8c06fdec
    cpu.r[0] = cpu.r[0].wrapping_add(44);
    cpu.r[1] = mem.read_u32(cpu.r[9] + 4).unwrap();
    cpu.r[13] = cpu.r[4];
    cpu.r[6] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[0], cpu.r[3]).unwrap();
    cpu.r[7] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[0] + 4, cpu.r[1]).unwrap();
    cpu.r[12] = cpu.r[4];
    cpu.r[8] = mem.read_u32(cpu.r[9] + 8).unwrap();
    cpu.r[1] = cpu.r[5];
    cpu.r[1] = cpu.r[1].wrapping_add(104); // 8c06fe24: add #104,r1 -> r1 = r5+0x68
    cpu.r[3] = mem.read_u32(cpu.r[9] + 12).unwrap();
    cpu.r[10] = cpu.r[10].wrapping_add(28);
    cpu.r[2] = cpu.r[2].wrapping_add(20);
    let _ = mem.write_u32(cpu.r[0] + 8, cpu.r[8]).unwrap();
    cpu.r[9] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[0] + 12, cpu.r[3]).unwrap();
    cpu.r[14] = cpu.r[14].wrapping_add(76);
    cpu.r[0] = mem.read_u32(0x8c06ffc4).unwrap(); // literal 0x80
    cpu.r[3] = cpu.r[4];
    cpu.r[8] = mem.read_u32(cpu.r[10]).unwrap();
    cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[10] + 4).unwrap();
    cpu.r[13] = cpu.r[13].wrapping_add(16);
    cpu.r[6] = cpu.r[6].wrapping_add(88);
    let _ = mem.write_u32(cpu.r[12], cpu.r[8]).unwrap();
    cpu.r[7] = cpu.r[7].wrapping_add(96);
    let _ = mem.write_u32(cpu.r[12] + 4, cpu.r[0]).unwrap();
    cpu.r[8] = mem.read_u32(cpu.r[10] + 8).unwrap();
    cpu.r[3] = cpu.r[3].wrapping_add(32);
    cpu.r[0] = mem.read_u32(cpu.r[10] + 12).unwrap();
    cpu.r[9] = cpu.r[9].wrapping_add(60);
    let _ = mem.write_u32(cpu.r[12] + 8, cpu.r[8]).unwrap();
    let _ = mem.write_u32(cpu.r[12] + 12, cpu.r[0]).unwrap();
    cpu.r[8] = cpu.r[5];
    cpu.r[8] = cpu.r[8].wrapping_add(100);
    cpu.r[0] = 24;
    let _ = mem.write_u32(cpu.r[15] + cpu.r[0], cpu.r[8]).unwrap(); // mov.l r8,@(r0,r15)
    cpu.r[8] = cpu.r[4];
    cpu.r[0] = mem.read_u32(cpu.r[10] + 16).unwrap();
    cpu.r[8] = cpu.r[8].wrapping_add(108);
    let _ = mem.write_u32(cpu.r[2], cpu.r[8]).unwrap(); // r2 = r15+20
    cpu.r[11] = mem.read_u32(cpu.r[10] + 20).unwrap();
    cpu.r[8] = cpu.r[5];
    cpu.r[2] = cpu.r[15];
    let _ = mem.write_u32(cpu.r[12] + 16, cpu.r[0]).unwrap();
    cpu.r[8] = cpu.r[8].wrapping_add(116);
    cpu.r[2] = cpu.r[2].wrapping_add(44);
    let _ = mem.write_u32(cpu.r[2], cpu.r[8]).unwrap();
    let _ = mem.write_u32(cpu.r[12] + 20, cpu.r[11]).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[0] = mem.read_u32(0x8c06ffc4).unwrap(); // literal 0x80
    cpu.r[8] = cpu.r[5];
    cpu.r[11] = mem.read_u32(cpu.r[10] + 24).unwrap();
    cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[0]); // r5+0x80
    cpu.r[2] = cpu.r[2].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[2], cpu.r[8]).unwrap();
    cpu.r[8] = mem.read_u32(cpu.r[10] + 28).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(40);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]).unwrap(); // mov.l r8,@r15
    let _ = mem.write_u32(cpu.r[12] + 24, cpu.r[11]).unwrap();
    cpu.r[8] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c06ffc8).unwrap(); // literal 0xcc
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[0]); // r4+0xcc
    let _ = mem.write_u32(cpu.r[12] + 28, cpu.r[11]).unwrap();
    cpu.r[11] = mem.read_u32(cpu.r[10] + 32).unwrap();
    let _ = mem.write_u32(cpu.r[2], cpu.r[11]).unwrap(); // r2 = r15+40
    cpu.r[11] = mem.read_u32(cpu.r[10] + 36).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(36);
    let _ = mem.write_u32(cpu.r[15] + 4, cpu.r[11]).unwrap();
    cpu.r[0] = mem.read_u32(0x8c06ffcc).unwrap(); // literal 0x8c
    cpu.r[11] = cpu.r[5];
    cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[0]); // r5+0x8c
    let _ = mem.write_u32(cpu.r[2], cpu.r[11]).unwrap();
    cpu.r[0] = cpu.r[5];
    cpu.r[11] = cpu.r[4];
    cpu.r[2] = cpu.r[15];
    cpu.r[11] = cpu.r[11].wrapping_add(76);
    cpu.r[2] = cpu.r[2].wrapping_add(32);
    let _ = mem.write_u32(cpu.r[2], cpu.r[11]).unwrap(); // [r15+32] = r4+76
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(40);
    cpu.r[11] = mem.read_u32(cpu.r[2]).unwrap();
    let _ = mem.write_u32(cpu.r[12] + 32, cpu.r[11]).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[11] = mem.read_u32(cpu.r[15] + 4).unwrap();
    cpu.r[2] = cpu.r[2].wrapping_add(16);
    let _ = mem.write_u32(cpu.r[12] + 36, cpu.r[11]).unwrap();
    cpu.r[11] = mem.read_u32(cpu.r[10] + 40).unwrap();
    let _ = mem.write_u32(cpu.r[15] + 8, cpu.r[11]).unwrap();
    cpu.r[11] = mem.read_u32(0x8c06ffd0).unwrap(); // literal 0x98
    cpu.r[10] = mem.read_u32(cpu.r[10] + 44).unwrap();
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[11]); // r5+0x98
    let _ = mem.write_u32(cpu.r[15] + 12, cpu.r[10]).unwrap();
    cpu.r[11] = mem.read_u32(cpu.r[15] + 8).unwrap();
    cpu.r[10] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[12] + 40, cpu.r[11]).unwrap();
    cpu.r[11] = mem.read_u32(0x8c06ffd4).unwrap(); // literal 0xd8
    cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[11]); // r4+0xd8
    cpu.r[11] = mem.read_u32(cpu.r[15] + 12).unwrap();
    let _ = mem.write_u32(cpu.r[12] + 44, cpu.r[11]).unwrap();
    cpu.r[12] = mem.read_u32(cpu.r[14]).unwrap(); // r14 = r5+76
    cpu.r[11] = mem.read_u32(cpu.r[14] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[2], cpu.r[11]).unwrap(); // r2 = r15+40
    let _ = mem.write_u32(cpu.r[13], cpu.r[12]).unwrap(); // r13 = r4+16
    cpu.r[2] = cpu.r[15];
    let _ = mem.write_u32(cpu.r[13] + 4, cpu.r[11]).unwrap();
    cpu.r[2] = cpu.r[2].wrapping_add(24);
    cpu.r[11] = mem.read_u32(cpu.r[14] + 8).unwrap();
    let _ = mem.write_u32(cpu.r[13] + 8, cpu.r[11]).unwrap();
    cpu.r[12] = mem.read_u32(cpu.r[6]).unwrap(); // r6 = r5+88
    cpu.r[11] = mem.read_u32(cpu.r[6] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[7], cpu.r[12]).unwrap(); // r7 = r4+96
    let _ = mem.write_u32(cpu.r[7] + 4, cpu.r[11]).unwrap();
    cpu.r[6] = mem.read_u32(cpu.r[6] + 8).unwrap();
    let _ = mem.write_u32(cpu.r[7] + 8, cpu.r[6]).unwrap();
    cpu.r[6] = mem.read_u32(cpu.r[2]).unwrap(); // r2 = r15+24 -> r5+100
    cpu.r[2] = cpu.r[15];
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0
    cpu.r[2] = cpu.r[2].wrapping_add(20);
    cpu.r[6] = mem.read_u32(cpu.r[2]).unwrap(); // r15+20 -> r4+108
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap(); // fmov fr0,@r6
    cpu.r[2] = cpu.r[15];
    cpu.r[7] = mem.read_u32(cpu.r[1]).unwrap(); // r1 = r5
    cpu.r[2] = cpu.r[2].wrapping_add(44);
    cpu.r[6] = mem.read_u32(cpu.r[1] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[3], cpu.r[7]).unwrap(); // r3 = r4+32
    let _ = mem.write_u32(cpu.r[3] + 4, cpu.r[6]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[1] + 8).unwrap();
    let _ = mem.write_u32(cpu.r[3] + 8, cpu.r[1]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r15+44 -> r5+116
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(44);
    cpu.r[1] = mem.read_u32(cpu.r[1] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[9], cpu.r[3]).unwrap(); // r9 = r4+60
    let _ = mem.write_u32(cpu.r[9] + 4, cpu.r[1]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r5+116
    cpu.r[1] = mem.read_u32(cpu.r[1] + 8).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(28);
    let _ = mem.write_u32(cpu.r[9] + 8, cpu.r[1]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r15+28 -> r5+0x80
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(28);
    cpu.r[1] = mem.read_u32(cpu.r[1] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[8], cpu.r[3]).unwrap(); // r8 = r4+0xcc
    let _ = mem.write_u32(cpu.r[8] + 4, cpu.r[1]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r5+0x80
    cpu.r[1] = mem.read_u32(cpu.r[1] + 8).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(36);
    let _ = mem.write_u32(cpu.r[8] + 8, cpu.r[1]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r15+36 -> r5+0x8c
    cpu.r[2] = cpu.r[15];
    cpu.r[6] = mem.read_u32(cpu.r[1]).unwrap();
    cpu.r[2] = cpu.r[2].wrapping_add(32);
    cpu.r[3] = mem.read_u32(cpu.r[1] + 4).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r15+32 -> r4+76
    let _ = mem.write_u32(cpu.r[1], cpu.r[6]).unwrap();
    cpu.r[2] = cpu.r[15];
    let _ = mem.write_u32(cpu.r[1] + 4, cpu.r[3]).unwrap();
    cpu.r[2] = cpu.r[2].wrapping_add(36);
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r5+0x8c
    cpu.r[3] = mem.read_u32(cpu.r[1] + 8).unwrap();
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(32);
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap(); // r4+76
    let _ = mem.write_u32(cpu.r[1] + 8, cpu.r[3]).unwrap();
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap(); // r0 = r5+0x98
    cpu.r[1] = mem.read_u32(cpu.r[0] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[10], cpu.r[3]).unwrap(); // r10 = r4+0xd8
    let _ = mem.write_u32(cpu.r[10] + 4, cpu.r[1]).unwrap();
    cpu.r[3] = mem.read_u32(cpu.r[0] + 8).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[0] + 12).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 8, cpu.r[3]).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 12, cpu.r[1]).unwrap();
    cpu.r[3] = mem.read_u32(cpu.r[0] + 16).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[0] + 20).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 16, cpu.r[3]).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 20, cpu.r[1]).unwrap();
    cpu.r[3] = mem.read_u32(cpu.r[0] + 24).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[0] + 28).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 24, cpu.r[3]).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 28, cpu.r[1]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[0] + 32).unwrap();
    let _ = mem.write_u32(cpu.r[10] + 32, cpu.r[0]).unwrap();
    cpu.r[0] = mem.read_u32(0x8c06ffd8).unwrap(); // literal 0xbc
    cpu.r[1] = cpu.r[5];
    cpu.r[7] = cpu.r[4];
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // r5+0xbc
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
    cpu.r[6] = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8c06ffdc).unwrap(); // literal 0xc0
    cpu.r[7] = cpu.r[7].wrapping_add(88); // r4+0x58
    cpu.r[1] = cpu.r[5];
    let _ = mem.write_f32(cpu.r[7], cpu.fr[0]).unwrap(); // fmov fr0,@r7
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // r5+0xc0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
    cpu.r[8] = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8c06ffe0).unwrap(); // literal 0xb0
    cpu.r[6] = cpu.r[6].wrapping_add(92); // r4+0x5c
    cpu.r[1] = mem.read_u32(0x8c06ffe4).unwrap(); // literal 0xc4
    cpu.r[7] = cpu.r[5];
    cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[3]); // r4+0xb0 (old r4)
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap(); // fmov fr0,@r6
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]); // r5+0xc4 (old r5)
    cpu.r[1] = mem.read_u32(0x8c06ffe8).unwrap(); // literal 0xd0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // r4 += 0xbc
    cpu.r[6] = mem.read_u32(cpu.r[7]).unwrap();
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]); // r5 += 0xd0
    cpu.r[3] = mem.read_u32(cpu.r[7] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[8], cpu.r[6]).unwrap();
    let _ = mem.write_u32(cpu.r[8] + 4, cpu.r[3]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[7] + 8).unwrap();
    let _ = mem.write_u32(cpu.r[8] + 8, cpu.r[0]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[5] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]).unwrap();
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[5] + 8).unwrap();
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]).unwrap();

    // epilogue: add #48,r15; mov.l @r15+,r8/r9/r10/r11/r12/r13; rts; delay pops r14
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
