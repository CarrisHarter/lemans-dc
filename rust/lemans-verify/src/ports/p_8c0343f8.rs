//! 0x8c0343f8 — initialise a large (~0x3e0-byte) object at `r4`.
//!
//! Writes nothing but constants: 1.0 in the header floats, three-word zeros
//! via 0x8c0b4ff0 (r4+0x24 and r4+0x30), five memsets via 0x8c010f88
//! (0x50/32, 0x70/0x90, 0x110/0x90, 0x1a0/0xd0, 0x274/0x108), -0.5 and 0.5
//! triples at 0x3c0/0x3cc, and a drift of trailing 1.0 scalars. The object is
//! never read, so the whole routine is a store sequence in program order.
//!
//! Note the deliberate gap at r4+0x3a4 (0x3a0 and 0x3a8 are zeroed, 0x3a4 is
//! not) and that 0x270 is a single float zero before the 0x274 memset.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: push r14, r13, pr; 48-byte frame
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(48);

    // r4+8 = 1.0; stage 1.0 at sp+36/40/44 (untyped float stores)
    cpu.r[0] = cpu.r[14].wrapping_add(8);
    cpu.fr[0] = 1.0;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[3] = cpu.r[15];
    cpu.r[0] = 36;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);
    cpu.r[7] = cpu.r[14];
    cpu.r[0] = 40;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);
    cpu.r[2] = 24;
    cpu.r[0] = 44;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);

    // copy the staged 1.0s to r4+0xc/0x10/0x14, then stage/copy to 0x18..0x20
    cpu.r[3] = cpu.r[3].wrapping_add(36);
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[7] = cpu.r[7].wrapping_add(12);
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
    cpu.r[5] = cpu.r[15];
    cpu.r[6] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[7], cpu.r[1]);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[0]);
    cpu.r[5] = cpu.r[5].wrapping_add(24);
    cpu.r[0] = mem.read_u32(0x8c034600).unwrap_or(0); // 0x8c0b4ff0
    cpu.r[6] = cpu.r[6].wrapping_add(24);
    cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(36);
    let _ = mem.write_u32(cpu.r[7].wrapping_add(8), cpu.r[1]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[2] = 28;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[2] = 32;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.pr = 0x8c034456; // jsr @r0 (delay slot below)
    let _ = mem.write_u32(cpu.r[6].wrapping_add(8), cpu.r[1]);
    crate::registry::p_8c0b4ff0::run(cpu, mem); // zero r4+0x24..0x2c

    cpu.r[0] = mem.read_u32(0x8c034600).unwrap_or(0); // 0x8c0b4ff0
    cpu.r[4] = cpu.r[14];
    cpu.pr = 0x8c03445e;
    cpu.r[4] = cpu.r[4].wrapping_add(48);
    crate::registry::p_8c0b4ff0::run(cpu, mem); // zero r4+0x30..0x38

    // zero floats at 0x3c..0x4c
    cpu.r[6] = cpu.r[14].wrapping_add(60);
    cpu.fr[0] = 0.0;
    cpu.r[5] = cpu.r[14].wrapping_add(64);
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]);
    cpu.r[3] = cpu.r[14].wrapping_add(68);
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]);
    cpu.r[0] = cpu.r[14].wrapping_add(72);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[1] = cpu.r[14].wrapping_add(76);
    cpu.r[4] = cpu.r[14].wrapping_add(80);
    cpu.r[0] = mem.read_u32(0x8c034604).unwrap_or(0); // 0x8c010f88 (memset)
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // r4+0x4c
    cpu.r[6] = 32;
    cpu.r[5] = 0; // jsr delay slot
    cpu.pr = 0x8c03448a;
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4+0x50, 0, 32)

    // zero floats at 0x70..0x7c, then memset(r4+0x80, 0, 0x90)
    cpu.r[4] = cpu.r[14].wrapping_add(112);
    cpu.fr[0] = 0.0;
    cpu.r[0] = cpu.r[14].wrapping_add(116);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[3] = cpu.r[14].wrapping_add(120);
    cpu.r[0] = mem.read_u32(0x8c034608).unwrap_or(0); // 0x80
    cpu.r[4] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[1] = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c03460c).unwrap_or(0); // 0x90
    cpu.r[1] = cpu.r[1].wrapping_add(124);
    cpu.r[0] = mem.read_u32(0x8c034604).unwrap_or(0); // memset
    cpu.r[5] = 0;
    cpu.pr = 0x8c0344b2;
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // jsr delay slot, r4+0x7c
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4+0x80, 0, 0x90)

    cpu.r[0] = mem.read_u32(0x8c034610).unwrap_or(0); // 0x110
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c03460c).unwrap_or(0); // 0x90
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034604).unwrap_or(0); // memset
    cpu.pr = 0x8c0344c0;
    cpu.r[5] = 0; // jsr delay slot
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4+0x110, 0, 0x90)

    cpu.r[0] = mem.read_u32(0x8c034614).unwrap_or(0); // 0x1a0
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c034618).unwrap_or(0); // 0xd0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034604).unwrap_or(0); // memset
    cpu.pr = 0x8c0344ce;
    cpu.r[5] = 0; // jsr delay slot
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4+0x1a0, 0, 0xd0)

    cpu.r[0] = mem.read_u32(0x8c03461c).unwrap_or(0); // 0x274
    cpu.r[4] = cpu.r[14];
    cpu.r[3] = mem.read_u32(0x8c034620).unwrap_or(0); // 0x270
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[6] = mem.read_u32(0x8c034624).unwrap_or(0); // 0x108
    cpu.r[1] = cpu.r[14];
    cpu.r[0] = mem.read_u32(0x8c034604).unwrap_or(0); // memset
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    cpu.fr[0] = 0.0;
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // r4+0x270
    cpu.r[5] = 0; // jsr delay slot
    cpu.pr = 0x8c0344e6;
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4+0x274, 0, 0x108)

    // zero floats 0x37c..0x3a0, skip 0x3a4, 0x3a8..0x3bc, then 0x3b4
    cpu.r[0] = mem.read_u32(0x8c034628).unwrap_or(0); // 0x37c
    cpu.r[1] = cpu.r[14];
    cpu.r[2] = 16;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c03462c).unwrap_or(0); // 0x380
    cpu.fr[1] = 0.0;
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034630).unwrap_or(0); // 0x384
    cpu.r[1] = cpu.r[14];
    cpu.r[7] = cpu.r[15];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034634).unwrap_or(0); // 0x388
    cpu.r[3] = cpu.r[14];
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034638).unwrap_or(0); // 0x38c
    cpu.r[1] = cpu.r[14];
    cpu.r[7] = cpu.r[7].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c03463c).unwrap_or(0); // 0x390
    cpu.r[3] = cpu.r[14];
    cpu.r[13] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034640).unwrap_or(0); // 0x394
    cpu.r[1] = cpu.r[14];
    cpu.r[4] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034644).unwrap_or(0); // 0x398
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034648).unwrap_or(0); // 0x39c
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c03464c).unwrap_or(0); // 0x3a0
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034650).unwrap_or(0); // 0x3a8
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]); // 0x3a0; 0x3a4 stays untouched
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034654).unwrap_or(0); // 0x3ac
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034658).unwrap_or(0); // 0x3b0
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c03465c).unwrap_or(0); // 0x3bc
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[1] = mem.read_u32(0x8c034660).unwrap_or(0); // 0x3b4
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);

    // -0.5 triple staged on the stack, copied to r4+0x3c0..0x3c8
    cpu.r[3] = cpu.r[15];
    cpu.r[0] = 0x8c034664; // mova
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // -0.5
    cpu.r[0] = 12;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);
    cpu.r[0] = mem.read_u32(0x8c034668).unwrap_or(0); // 0x3c0
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]);
    cpu.r[2] = 20;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[0] = 0x8c03466c; // mova
    cpu.r[1] = mem.read_u32(0x8c034670).unwrap_or(0); // 0x3cc
    cpu.r[2] = 0;
    cpu.r[6] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_u32(cpu.r[13], cpu.r[6]);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[5]);
    cpu.r[1] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[1]);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0.5
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[2] = 4;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);

    // tail scalars: 0x3d8/0x3dc zero, then 1.0 at 0xd0, 0xd4, 0xcc, 0x160, 0x164, 0x15c
    cpu.r[0] = mem.read_u32(0x8c034674).unwrap_or(0); // 0x3d8
    cpu.r[1] = cpu.r[14];
    cpu.r[3] = cpu.r[14];
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034678).unwrap_or(0); // 0x3dc
    cpu.fr[0] = 1.0;
    cpu.r[4] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034618).unwrap_or(0); // 0xd0
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c03467c).unwrap_or(0); // 0xd4
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034680).unwrap_or(0); // 0xcc
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c034684).unwrap_or(0); // 0x160
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[3] = mem.read_u32(0x8c034688).unwrap_or(0); // 0x164
    cpu.r[1] = cpu.r[14];
    cpu.r[0] = mem.read_u32(0x8c03468c).unwrap_or(0); // 0x15c
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);

    // epilogue
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
