//! 0x8c02d5e4 — single-precision FPU update of a struct at r4.
//!
//! Index normalization (`[0x3b8+r4]`: -1 -> 0, 0 -> 1, else unchanged) is
//! scaled by 4 into r3 and selects two table rows. The C hides: T comes from
//! the last cmp/eq/tst of the normalization (1 for idx -1 and 0, else 0);
//! r4 ends up r4+0x3d8; r0 is the `mova` constant address 0x8c02d6b4; r1/r6
//! are table pointers left advanced; fr2 is the value stored at +0x3d4 and
//! fr0/fr1 are the last loads; four successive stores hit +0x3dc (only the
//! last survives in memory, but all four appear in the write log).

use sh4_oracle::{Cpu, Memory};

const CONST_ADDR: u32 = 0x8c02d6b4; // mova target, holds 1.234f

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    let raw = mem.read_u32(0x3b8u32.wrapping_add(s)).unwrap_or(0);
    let r3_raw = if raw == 0xFFFF_FFFF {
        0u32 // bra delay: mov #0,r3
    } else if raw == 0 {
        1u32 // mov #1,r3
    } else {
        raw
    };
    // shll2 r3 at 0x8c02d5fc is the last T-writing instruction, so T is its
    // carry (bit 30), not the cmp/eq or tst that selected r3.
    cpu.set_t(r3_raw >> 30 & 1 != 0);
    let r3 = r3_raw << 2;
    cpu.r[3] = r3;

    let w = mem.read_u32(s.wrapping_add(4)).unwrap_or(0);
    let r1 = w.wrapping_add(0x274); // base for all table rows

    let mut fr0 = mem.read_f32(r1.wrapping_add(0x90).wrapping_add(r3)).unwrap_or(0.0);
    let fr3 = mem.read_f32(r1.wrapping_add(0xe4)).unwrap_or(0.0);
    fr0 = fr0 * fr3; // fmul fr3,fr0
    let fr4 = mem.read_f32(w.wrapping_add(0x11c)).unwrap_or(0.0); // r14 = w (set before r1 += 0x274) + 0x11c

    let store_3d4 = s.wrapping_add(0x3d4);
    let _ = mem.write_f32(store_3d4, fr0);
    let fr2 = mem.read_f32(store_3d4).unwrap_or(0.0); // fmov @r7,fr2 reload

    let store_3dc = s.wrapping_add(0x3dc);
    let mut fr1 = mem
        .read_f32(r1.wrapping_add(0xac).wrapping_add(r3))
        .unwrap_or(0.0);
    let fr0b = mem.read_f32(r1.wrapping_add(0xf0)).unwrap_or(0.0);
    fr1 = fr1 + fr0b; // fadd
    fr1 = fr1 * fr2; // fmul
    fr1 = fr1 * fr2; // fmul
    let _ = mem.write_f32(store_3dc, fr1);

    fr1 = mem.read_f32(r1.wrapping_add(0xe8)).unwrap_or(0.0);
    let fr0 = mem.read_f32(store_3dc).unwrap_or(0.0);
    fr1 = fr1 * fr3;
    fr1 = fr1 * fr3;
    fr1 = fr1 + fr0; // fadd
    let _ = mem.write_f32(store_3dc, fr1);

    let fr0c = mem.read_f32(CONST_ADDR).unwrap_or(0.0);
    let fr1 = mem.read_f32(store_3dc).unwrap_or(0.0) + fr0c; // fadd
    let _ = mem.write_f32(store_3dc, fr1);

    let r1f = r1.wrapping_add(0xec);
    let mut fr0 = mem.read_f32(store_3dc).unwrap_or(0.0);
    fr0 = fr0 / fr4; // fdiv fr4,fr0
    let _ = mem.write_f32(store_3dc, fr0);

    let fr1 = mem.read_f32(r1.wrapping_add(0xc8).wrapping_add(r3)).unwrap_or(0.0);
    let fr0 = mem.read_f32(r1f).unwrap_or(0.0);
    let fr1 = fr1 * fr0; // fmul
    let _ = mem.write_f32(s.wrapping_add(0x3d8), fr1);

    cpu.r[0] = CONST_ADDR;
    cpu.r[1] = r1f;
    cpu.r[4] = s.wrapping_add(0x3d8);
    cpu.r[5] = 0xec;
    cpu.r[6] = r1.wrapping_add(0xc8).wrapping_add(r3);
    cpu.r[7] = 0x3d8;
    cpu.fr[0] = fr0;
    cpu.fr[1] = fr1;
    cpu.fr[2] = fr2;
    cpu.fr[3] = fr3;
    cpu.fr[4] = fr4;
}
