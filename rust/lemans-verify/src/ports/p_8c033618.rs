//! 0x8c033618 — normalize/scale a block of float fields of the struct in r4.
//!
//! Straight-line, no branches. Side effects the C does not show:
//!   * r4 is advanced by 0x11c before the final loads/stores (in/out cursor),
//!   * r13/r14 are saved and restored through r15, but r14 is also used as a
//!     scratch offset (0x3e4) and r13 ends at base+0x364 before being restored,
//!   * the last two stores both target [base+0x3e4] (write, re-read, scale by
//!     1000, write again),
//!   * every PC-relative constant (0x8c03371c/0x8c03372c/0x8c033750/0x8c033764/
//!     0x8c033768 and the offset pool) is read from the image, not hardcoded.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];

    // prologue: mov.l r14,@-r15 ; mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);

    // [base+60] = 1 / [base+60]
    cpu.r[0] = base.wrapping_add(60);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[2] = 1.0; // fldi1 fr2
    cpu.fr[0] = cpu.fr[2];
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1]; // fdiv fr1,fr0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);

    // [base+112] = 1 / [base+112]
    cpu.r[3] = base.wrapping_add(112);
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.fr[0] = cpu.fr[2];
    cpu.r[0] = mem.read_u32(0x8c033718).unwrap(); // 0x270
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1];
    cpu.r[1] = base;
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);

    // [base+0x270] = 1 / [base+0x270]
    cpu.r[1] = base.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[5] = base;
    cpu.r[0] = 0x8c03371c; // mova
    cpu.fr[2] = cpu.fr[2] / cpu.fr[0]; // fdiv fr0,fr2
    let _ = mem.write_f32(cpu.r[1], cpu.fr[2]);

    // C1 = [0x8c03371c]; scale [base+116], [base+0xec], [base+0x17c] by C1
    cpu.r[5] = base.wrapping_add(116);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[3] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[1] = base;
    cpu.r[0] = mem.read_u32(0x8c033720).unwrap(); // 0xec
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    cpu.r[6] = base;
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]);
    cpu.r[3] = base.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[5] = base;
    cpu.r[0] = mem.read_u32(0x8c033724).unwrap(); // 0x17c
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = base.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[0] = mem.read_u32(0x8c033728).unwrap(); // 0x388
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);

    // C2 = [0x8c03372c]; scale [base+0x388],[base+0x38c],[base+0x390],[base+0x394] by C2
    cpu.r[5] = base.wrapping_add(cpu.r[0]);
    cpu.r[0] = 0x8c03372c; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[1] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[3] = mem.read_u32(0x8c033730).unwrap(); // 0x38c
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    cpu.r[0] = mem.read_u32(0x8c033734).unwrap(); // 0x390
    cpu.r[1] = base.wrapping_add(cpu.r[3]);
    cpu.r[3] = base;
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]);
    cpu.r[3] = base.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[5] = base;
    cpu.r[0] = mem.read_u32(0x8c033738).unwrap(); // 0x394
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[6] = base.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[0] = base;
    cpu.r[1] = mem.read_u32(0x8c03373c).unwrap(); // 0x370
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[0] = base.wrapping_add(cpu.r[1]);
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.r[3] = mem.read_u32(0x8c033740).unwrap(); // 0x278
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]);

    // [base+0x370] = [base+0x278] * [base+0x370]
    cpu.r[5] = base.wrapping_add(cpu.r[3]);
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);

    // [base+0x374] *= [base+0x278]; [base+0x378] *= [base+0x278]
    cpu.r[0] = mem.read_u32(0x8c033744).unwrap(); // 0x374
    cpu.r[3] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[1] = base;
    cpu.r[3] = base.wrapping_add(cpu.r[0]);
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[13] = base;
    cpu.r[0] = mem.read_u32(0x8c033748).unwrap(); // 0x378
    cpu.r[7] = base;
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = base.wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[5] = base;
    cpu.r[0] = mem.read_u32(0x8c03374c).unwrap(); // 0x364
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);

    // [base+0x364] /= C3, C3 = [0x8c033750]
    cpu.r[13] = base.wrapping_add(cpu.r[0]);
    cpu.r[0] = 0x8c033750; // mova
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[1] = base;
    cpu.fr[1] = mem.read_f32(cpu.r[13]).unwrap();
    cpu.r[14] = mem.read_u32(0x8c033754).unwrap(); // 0x3e4
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    cpu.r[6] = mem.read_u32(0x8c033758).unwrap(); // 0x31c
    cpu.r[7] = base.wrapping_add(cpu.r[14]);
    cpu.r[3] = mem.read_u32(0x8c03375c).unwrap(); // 0x358
    cpu.r[5] = base.wrapping_add(cpu.r[6]);
    cpu.r[0] = mem.read_u32(0x8c033760).unwrap(); // 0x11c
    cpu.r[1] = base.wrapping_add(cpu.r[3]);
    let _ = mem.write_f32(cpu.r[13], cpu.fr[1]);
    cpu.r[4] = base.wrapping_add(cpu.r[0]); // r4 advanced by 0x11c

    // final: t = [base+0x3e4] / ([base+0x31c]*[base+0x358] / ([base+0x11c]*pi))
    //        [base+0x3e4] = t * 1000
    cpu.r[0] = 0x8c033764; // mova: pi
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.r[0] = 0x8c033768; // mova: 1000.0
    cpu.fr[3] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // * pi
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap();
    cpu.fr[3] = cpu.fr[3] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] / cpu.fr[1]; // fdiv fr1,fr3
    cpu.fr[0] = cpu.fr[0] / cpu.fr[3]; // fdiv fr3,fr0
    let _ = mem.write_f32(cpu.r[7], cpu.fr[0]);
    cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // 1000.0
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    let _ = mem.write_f32(cpu.r[7], cpu.fr[1]);

    // epilogue: mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 (delay slot)
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
