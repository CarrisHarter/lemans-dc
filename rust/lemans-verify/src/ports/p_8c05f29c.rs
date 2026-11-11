//! 0x8c05f29c — struct init: mask the header at +4/+5, splat 13 floats from
//! the tables at 0x8c1d0f18/0x8c1d0f28 into +12..+60, store the global pointer
//! at +4, zero +8, then (when r4 != r5) copy 64 bytes from r5 over r4.
//!
//! CFG: straight-line until `cmp/eq r5,r4 ; bt.s 0x8c05f3cc`. The delay slot
//! `mov.l r0,@r4` (r0 == 0) executes on BOTH paths; the fall-through path then
//! overwrites +0/+4/+8 and +12..+60 from r5+0..+60 (r5 advanced by 12 first).
//!
//! Side effects the decompiled C does not show:
//! - the +4/+5 header masks are dead stores: `mov.l r1,@r8` (0x8c05f37c) and,
//!   on the copy path, `mov.l r0,@r8` (0x8c05f390) overwrite +4 afterwards;
//! - r1 ends as the global *0x8c26cff0 (eq path) or as [r5+52] (copy path);
//! - r3/r6/r7 end as r4+56 / table+12 / r4+52, r5 ends advanced by 12 on the
//!   copy path, r8..r14 are restored by the epilogue pops;
//! - fr0 = +0.0 (fldi0), fr1 = table[3] bits, T = (r4 == r5).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    // Literal pool, read from the image so both sides always agree.
    let tbl_a = mem.read_u32(0x8c05f3e4).unwrap(); // 0x8c1d0f18
    let tbl_b = mem.read_u32(0x8c05f3e0).unwrap(); // 0x8c1d0f28
    let mask_w = mem.read_u32(0x8c05f3e8).unwrap(); // 0xfffffc7f
    let mask_l = mem.read_u32(0x8c05f3ec).unwrap(); // 0xfffc7fff
    let global = mem.read_u32(0x8c05f3f0).unwrap(); // 0x8c26cff0

    cpu.r[15] = cpu.r[15].wrapping_sub(40); // 7x mov.l @-r15 + add #-12,r15

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

    // Tail stores: global pointer at +4, zero at +8.
    let mut r1 = mem.read_u32(global).unwrap();
    mem.write_u32(r4 + 4, r1).unwrap(); // mov.l r1,@r8 (r8 = r4+4)
    mem.write_u32(r4 + 8, 0).unwrap(); // mov.l r0,@(8,r4), r0 = 0
    cpu.set_t(cpu.r[5] == r4); // cmp/eq r5,r4
    mem.write_u32(r4, 0).unwrap(); // bt.s delay slot: mov.l r0,@r4

    // Fall-through path (r4 != r5): copy 64 bytes from r5 over r4.
    let mut r5n = r5;
    if r4 != r5 {
        r1 = mem.read_u32(r5).unwrap();
        mem.write_u32(r4, r1).unwrap();
        let v = mem.read_u32(r5 + 4).unwrap();
        mem.write_u32(r4 + 4, v).unwrap();
        let v = mem.read_u32(r5 + 8).unwrap();
        mem.write_u32(r4 + 8, v).unwrap();
        r5n = r5 + 12; // add #12,r5
        for i in 0..13u32 {
            let v = mem.read_u32(r5n + i * 4).unwrap();
            mem.write_u32(r4 + 12 + i * 4, v).unwrap();
        }
        r1 = mem.read_u32(r5n + 40).unwrap(); // last `mov.l @(40,r5),r1`
    }

    // Epilogue: r0 = r4; add #12 + 7x mov.l @r15+ restore r15 AND r8..r14 to
    // their entry values (stack window, excluded from the write diff), so
    // only r0..r7 keep the body's side effects.
    cpu.r[0] = r4;
    cpu.r[1] = r1;
    cpu.r[3] = r4 + 56;
    cpu.r[5] = r5n;
    cpu.r[6] = tbl_a + 12;
    cpu.r[7] = r4 + 52;
    cpu.r[15] = cpu.r[15].wrapping_add(40);
}
