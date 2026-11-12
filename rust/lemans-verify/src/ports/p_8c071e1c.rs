//! 0x8c071e1c — initialise a large object: two 92-byte sub-structs at +16 and
//! +108, then float fields and four-word blocks at +0xcc..+0x148, and finally
//! the header at +0..+0xc9.
//!
//! Calls 0x8c152ee0 at 0x8c071e32 (delay `add #16,r4`) and 0x8c071e3a (delay
//! `add #108,r4`). Everything else is straight-line. T is never written and the
//! callee preserves it, so T at exit equals T at entry.
//!
//! Leaked registers at exit: r0 = r4_entry, r1 = r3 = bits(0.25f),
//! r2 = r15_frame+44, r4 = r4_entry+0x13c, r5 = bits(1.0f),
//! r6 = r4_entry+0x130, r7 = r4_entry+0x12c; r8..r14 and pr are popped back
//! from the frame. fr0..fr6 = 0.8, 0.2, 0.25, 0.55, 0.1, 0.3, 1.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14,@-r15 ; mov r4,r14 ; push r13..r8 ;
    // sts.l pr,@-r15 ; add #-96,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    let p = cpu.r[4];
    cpu.r[14] = p;
    for n in [13usize, 12, 11, 10, 9, 8] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[n]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(96);
    let sp = cpu.r[15];

    // 8c071e30: mov.l 0x8c071fec,r0 ; jsr @r0 ; add #16,r4 (delay)
    cpu.r[0] = mem.read_u32(0x8c071fec).unwrap_or(0);
    cpu.pr = 0x8c071e36;
    cpu.r[4] = p.wrapping_add(16);
    crate::registry::p_8c152ee0::run(cpu, mem);

    // 8c071e36: mov.l 0x8c071fec,r0 ; mov r14,r4 ; jsr @r0 ; add #108,r4 (delay)
    cpu.r[0] = mem.read_u32(0x8c071fec).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.pr = 0x8c071e3e;
    cpu.r[4] = cpu.r[4].wrapping_add(108);
    crate::registry::p_8c152ee0::run(cpu, mem);

    // 8c071e3e: fldi1 fr6 ; mov #48,r0 ; fmov fr6,@(r0,r15)
    cpu.fr[6] = 1.0;
    let _ = mem.write_f32(sp.wrapping_add(48), cpu.fr[6]);
    // 8c071e44: mov #56,r2 ; mova 0x8c071ff0,r0 ; fmov @r0,fr5
    cpu.fr[5] = mem.read_f32(0x8c071ff0).unwrap_or(0.0);
    // 8c071e4e: mov #52,r0 ; fmov fr5,@(r0,r15)
    let _ = mem.write_f32(sp.wrapping_add(52), cpu.fr[5]);
    // 8c071e5a / 8c071e62: fmov fr5,@(56,r15) / fmov fr5,@(60,r15)
    let _ = mem.write_f32(sp.wrapping_add(56), cpu.fr[5]);
    let _ = mem.write_f32(sp.wrapping_add(60), cpu.fr[5]);

    // 8c071e6a..8c071e8e: copy those four words to p+0xdc..p+0xe8
    let v = mem.read_u32(sp.wrapping_add(48)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(52)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0xdc), v);
    let _ = mem.write_u32(p.wrapping_add(0xe0), w);
    let v = mem.read_u32(sp.wrapping_add(56)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(60)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0xe4), v);
    let _ = mem.write_u32(p.wrapping_add(0xe8), w);

    // 8c071e92: fmov fr6,@(64,r15)
    let _ = mem.write_f32(sp.wrapping_add(64), 1.0);
    // 8c071e96: mova 0x8c071ffc,r0 ; fmov @r0,fr4
    cpu.fr[4] = mem.read_f32(0x8c071ffc).unwrap_or(0.0);
    // 8c071ea2 / 8c071eaa / 8c071eb2: fmov fr4,@(68|72|76,r15)
    let _ = mem.write_f32(sp.wrapping_add(68), cpu.fr[4]);
    let _ = mem.write_f32(sp.wrapping_add(72), cpu.fr[4]);
    let _ = mem.write_f32(sp.wrapping_add(76), cpu.fr[4]);

    // 8c071eb8..8c071ed4: copy those four words to p+0xcc..p+0xd8
    let v = mem.read_u32(sp.wrapping_add(64)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(68)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0xcc), v);
    let _ = mem.write_u32(p.wrapping_add(0xd0), w);
    let v = mem.read_u32(sp.wrapping_add(72)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(76)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0xd4), v);
    let _ = mem.write_u32(p.wrapping_add(0xd8), w);

    // 8c071ed6: mova 0x8c07200c,r0 ; fmov @r0,fr3 ; fmov fr3,@r12 (p+0xec)
    cpu.fr[3] = mem.read_f32(0x8c07200c).unwrap_or(0.0);
    let _ = mem.write_f32(p.wrapping_add(0xec), cpu.fr[3]);
    // 8c071edc: mova 0x8c072024,r0 ; fmov @r0,fr2 ; fmov fr2,@r13 (p+0xf0)
    cpu.fr[2] = mem.read_f32(0x8c072024).unwrap_or(0.0);
    let _ = mem.write_f32(p.wrapping_add(0xf0), cpu.fr[2]);
    // 8c071ee0: mova 0x8c072028,r0 ; fmov @r0,fr1 ; fmov fr1,@r7 (p+0xf4)
    cpu.fr[1] = mem.read_f32(0x8c072028).unwrap_or(0.0);
    let _ = mem.write_f32(p.wrapping_add(0xf4), cpu.fr[1]);
    // 8c071ee6: mova 0x8c07202c,r0 ; fmov @r0,fr0 ; fmov fr0,@r6 (p+0xf8)
    cpu.fr[0] = mem.read_f32(0x8c07202c).unwrap_or(0.0);
    let _ = mem.write_f32(p.wrapping_add(0xf8), cpu.fr[0]);

    // 8c071eec: fmov fr6,@(80,r15)
    let _ = mem.write_f32(sp.wrapping_add(80), 1.0);
    // 8c071ef2 / 8c071ef8 / 8c071efe: fmov fr2,@(84|88|92,r15)
    let _ = mem.write_f32(sp.wrapping_add(84), cpu.fr[2]);
    let _ = mem.write_f32(sp.wrapping_add(88), cpu.fr[2]);
    let _ = mem.write_f32(sp.wrapping_add(92), cpu.fr[2]);

    // 8c071f02..8c071f12: copy those four words to p+0xfc..p+0x108
    let v = mem.read_u32(sp.wrapping_add(80)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(84)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0xfc), v);
    let _ = mem.write_u32(p.wrapping_add(0x100), w);
    let v = mem.read_u32(sp.wrapping_add(88)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(92)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x104), v);
    let _ = mem.write_u32(p.wrapping_add(0x108), w);

    // 8c071f14: fmov fr6,@r15 ; 8c071f1a/20/26: fmov fr5,@(4|8|12,r15)
    let _ = mem.write_f32(sp, 1.0);
    let _ = mem.write_f32(sp.wrapping_add(4), cpu.fr[5]);
    let _ = mem.write_f32(sp.wrapping_add(8), cpu.fr[5]);
    let _ = mem.write_f32(sp.wrapping_add(12), cpu.fr[5]);

    // 8c071f2a..8c071f4e: copy those four words to p+0x11c..p+0x128
    let v = mem.read_u32(sp).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x11c), v);
    let _ = mem.write_u32(p.wrapping_add(0x120), w);
    let v = mem.read_u32(sp.wrapping_add(8)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x124), v);
    let _ = mem.write_u32(p.wrapping_add(0x128), w);

    // 8c071f52: fmov fr6,@(16,r15) ; 8c071f5c/64/6c: fmov fr4,@(20|24|28,r15)
    let _ = mem.write_f32(sp.wrapping_add(16), 1.0);
    let _ = mem.write_f32(sp.wrapping_add(20), cpu.fr[4]);
    let _ = mem.write_f32(sp.wrapping_add(24), cpu.fr[4]);
    let _ = mem.write_f32(sp.wrapping_add(28), cpu.fr[4]);

    // 8c071f74..8c071f96: copy those four words to p+0x10c..p+0x118
    let v = mem.read_u32(sp.wrapping_add(16)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(20)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x10c), v);
    let _ = mem.write_u32(p.wrapping_add(0x110), w);
    let v = mem.read_u32(sp.wrapping_add(24)).unwrap_or(0);
    let w = mem.read_u32(sp.wrapping_add(28)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x114), v);
    let _ = mem.write_u32(p.wrapping_add(0x118), w);

    // 8c071f9a: fmov fr3,@r7 (p+0x12c) ; fmov fr2,@r6 (p+0x130) ;
    // fmov fr1,@r5 (p+0x134) ; fmov fr0,@r3 (p+0x138)
    let _ = mem.write_f32(p.wrapping_add(0x12c), cpu.fr[3]);
    let _ = mem.write_f32(p.wrapping_add(0x130), cpu.fr[2]);
    let _ = mem.write_f32(p.wrapping_add(0x134), cpu.fr[1]);
    let _ = mem.write_f32(p.wrapping_add(0x138), cpu.fr[0]);

    // 8c071fa2: fmov fr6,@(32,r15) ; 8c071fa8/ae/b4: fmov fr2,@(36|40|44,r15)
    let _ = mem.write_f32(sp.wrapping_add(32), 1.0);
    let _ = mem.write_f32(sp.wrapping_add(36), cpu.fr[2]);
    let _ = mem.write_f32(sp.wrapping_add(40), cpu.fr[2]);
    let _ = mem.write_f32(sp.wrapping_add(44), cpu.fr[2]);

    // 8c071fb6..8c071fc4: copy sp+32..44 to p+0x13c..p+0x148, keeping the
    // loaded values for the registers the routine leaks.
    let r5v = mem.read_u32(sp.wrapping_add(32)).unwrap_or(0);
    let mut r3v = mem.read_u32(sp.wrapping_add(36)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x13c), r5v);
    let _ = mem.write_u32(p.wrapping_add(0x140), r3v);
    r3v = mem.read_u32(sp.wrapping_add(40)).unwrap_or(0);
    let r1v = mem.read_u32(sp.wrapping_add(44)).unwrap_or(0);
    let _ = mem.write_u32(p.wrapping_add(0x144), r3v);
    let _ = mem.write_u32(p.wrapping_add(0x148), r1v);

    // 8c071fc6: header
    let _ = mem.write_u32(p, 3);
    let _ = mem.write_u32(p.wrapping_add(4), 2);
    let _ = mem.write_u32(p.wrapping_add(8), 0);
    let _ = mem.write_u32(p.wrapping_add(12), 0);
    let _ = mem.write_u8(p.wrapping_add(0xc8), 0);
    let _ = mem.write_u8(p.wrapping_add(0xc9), 0);

    // epilogue: add #96,r15 ; lds.l @r15+,pr ; pop r8..r13 ; rts ; pop r14
    cpu.r[15] = sp.wrapping_add(96);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for n in [8usize, 9, 10, 11, 12, 13, 14] {
        cpu.r[n] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }

    // leaked registers after the final stores
    cpu.r[0] = p; // mov r14,r0
    cpu.r[1] = r1v;
    cpu.r[2] = sp.wrapping_add(44);
    cpu.r[3] = r3v;
    cpu.r[4] = p.wrapping_add(0x13c);
    cpu.r[5] = r5v;
    cpu.r[6] = p.wrapping_add(0x130);
    cpu.r[7] = p.wrapping_add(0x12c);
}
