//! 0x8c05f3f4 — struct init, single-path variant of 0x8c05f29c: mask the
//! header at +4/+5, splat 13 floats from 0x8c1d0f18/0x8c1d0f28 into
//! +12..+60, store the global pointer at +4, zero +0/+8. No branches and no
//! flag-setting instructions, so T and the rest of sr survive untouched.
//!
//! Side effects the decompiled C does not show:
//! - the +4/+5 header masks are dead stores: `mov.l r0,@r1` (0x8c05f4cc)
//!   overwrites +4 at the end;
//! - r1/r3/r5/r6/r7 end as r4+4 / r4+56 / 0x8c1d0f24 / 0 / 0x8c26cff0,
//!   r8..r14 are restored by the epilogue pops;
//! - fr0 = +0.0 (fldi0), fr1 = table[3] bits, T unchanged (no cmp/tst).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    // Literal pool, read from the image so both sides always agree.
    let tbl_a = mem.read_u32(0x8c05f4e8).unwrap(); // 0x8c1d0f18
    let tbl_b = mem.read_u32(0x8c05f4e4).unwrap(); // 0x8c1d0f28
    let mask_w = mem.read_u32(0x8c05f4ec).unwrap(); // 0xfffffc7f
    let mask_l = mem.read_u32(0x8c05f4f0).unwrap(); // 0xfffc7fff
    let global = mem.read_u32(0x8c05f4f4).unwrap(); // 0x8c26cff0

    cpu.r[15] = cpu.r[15].wrapping_sub(28); // 7x mov.l @-r15, no local frame

    // Header masking at r4+4 / r4+5 (store order preserved).
    let b = mem.read_u8(r4 + 4).unwrap() & 0xf0; // and #-16
    mem.write_u8(r4 + 4, b).unwrap();
    let b = mem.read_u8(r4 + 4).unwrap() & 0x8f; // and #-113
    mem.write_u8(r4 + 4, b).unwrap();
    let h = mem.read_u16(r4 + 4).unwrap() & mask_w as u16;
    mem.write_u16(r4 + 4, h).unwrap();
    let b = mem.read_u8(r4 + 5).unwrap() & 0xe3; // and #-29
    mem.write_u8(r4 + 5, b).unwrap();
    let b = mem.read_u8(r4 + 5).unwrap() & 0x9f; // and #-97
    mem.write_u8(r4 + 5, b).unwrap();
    let w = mem.read_u32(r4 + 4).unwrap() & mask_l;
    mem.write_u32(r4 + 4, w).unwrap();

    // 13 fmov.s copies into r4+12..r4+60, in original store order.
    let mut fr0 = mem.read_f32(tbl_a).unwrap();
    mem.write_f32(r4 + 12, fr0).unwrap();
    fr0 = mem.read_f32(tbl_a + 4).unwrap();
    mem.write_f32(r4 + 16, fr0).unwrap();
    fr0 = mem.read_f32(tbl_a + 8).unwrap();
    mem.write_f32(r4 + 20, fr0).unwrap();
    fr0 = mem.read_f32(tbl_a + 12).unwrap();
    mem.write_f32(r4 + 24, fr0).unwrap();
    fr0 = mem.read_f32(tbl_b).unwrap();
    mem.write_f32(r4 + 28, fr0).unwrap();
    fr0 = mem.read_f32(tbl_b + 4).unwrap();
    mem.write_f32(r4 + 32, fr0).unwrap();
    fr0 = mem.read_f32(tbl_b + 8).unwrap();
    mem.write_f32(r4 + 36, fr0).unwrap();
    fr0 = mem.read_f32(tbl_b + 12).unwrap();
    mem.write_f32(r4 + 40, fr0).unwrap();
    fr0 = mem.read_f32(tbl_a).unwrap();
    mem.write_f32(r4 + 44, fr0).unwrap();
    let mut fr1 = mem.read_f32(tbl_a + 4).unwrap();
    mem.write_f32(r4 + 48, fr1).unwrap();
    cpu.fr[0] = 0.0; // fldi0 fr0
    fr1 = mem.read_f32(tbl_a + 8).unwrap();
    mem.write_f32(r4 + 52, fr1).unwrap();
    fr1 = mem.read_f32(tbl_a + 12).unwrap();
    mem.write_f32(r4 + 56, fr1).unwrap();
    mem.write_f32(r4 + 60, cpu.fr[0]).unwrap();
    cpu.fr[1] = fr1;

    // Tail stores: global pointer at +4, zeros at +8 and +0.
    let gp = mem.read_u32(global).unwrap();
    mem.write_u32(r4 + 4, gp).unwrap(); // mov.l r0,@r1 (r1 = r4+4)
    mem.write_u32(r4 + 8, 0).unwrap(); // mov.l r6,@(8,r4), r6 = 0
    mem.write_u32(r4, 0).unwrap(); // mov.l r6,@r4

    // Epilogue: r0 = r4; 7x mov.l @r15+ restore r15 AND r8..r14 to their entry
    // values (stack window, excluded from the write diff), so only r0..r7
    // keep the body's side effects.
    cpu.r[0] = r4;
    cpu.r[1] = r4 + 4;
    cpu.r[3] = r4 + 56;
    cpu.r[5] = tbl_a + 12;
    cpu.r[6] = 0;
    cpu.r[7] = global;
    cpu.r[15] = cpu.r[15].wrapping_add(28);
}
