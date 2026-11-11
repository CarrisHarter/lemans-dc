//! 0x8c03376c — the adjacent twin of 0x8c033618, same field layout but the
//! reciprocals/multiplies are inverted: where 618 multiplies by [base+0x278]
//! and divides [base+0x364] by C3, this one divides by [base+0x278], multiplies
//! [base+0x364] by C3, and ends with ([base+0x31c]*[base+0x358]/([base+0x11c]*pi))
//! * [base+0x3e4] / 1000 instead of / that / * 1000.
//!
//! Straight-line, no branches. Same hidden side effects as 0x8c033618:
//! r4 ends advanced by 0x11c, r13/r14 go through the stack, all constants are
//! read from the image pool.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];

    // prologue: mov.l r14,@-r15 ; mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);

    // [base+60] = 1 / [base+60]
    cpu.r[3] = base.wrapping_add(60);
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.fr[2] = 1.0; // fldi1 fr2
    cpu.fr[0] = cpu.fr[2];
    cpu.r[0] = mem.read_u32(0x8c03386c).unwrap(); // 0x270
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1];
    cpu.r[1] = base;
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);

    // [base+0x270] = 1 / [base+0x270]
    cpu.r[1] = base.wrapping_add(cpu.r[0]);
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.fr[0] = cpu.fr[2];
    cpu.r[0] = base;
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);

    // [base+112] = 1 / [base+112]
    cpu.r[0] = base.wrapping_add(112);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[3] = base;
    cpu.r[3] = base.wrapping_add(116);
    cpu.fr[2] = cpu.fr[2] / cpu.fr[0]; // fdiv fr0,fr2
    let _ = mem.write_f32(cpu.r[0], cpu.fr[2]);

    // C1 = [0x8c033870]; [base+116] *= C1; later [base+0x17c] *= C1
    cpu.r[1] = base;
    cpu.r[0] = 0x8c033870; // mova
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[6] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[5] = base;
    cpu.r[0] = mem.read_u32(0x8c033874).unwrap(); // 0x388
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2];
    cpu.r[1] = base.wrapping_add(cpu.r[0]); // base+0x388
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);

    // C2 = [0x8c033878]; [base+0x388],[base+0x38c],[base+0x390],[base+0x394],[base+0xec] *= C2
    cpu.r[0] = 0x8c033878; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[3] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // [base+0x388]
    cpu.r[0] = mem.read_u32(0x8c03387c).unwrap(); // 0x38c
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[3] = base.wrapping_add(cpu.r[0]); // base+0x38c
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[1] = base;
    cpu.r[0] = mem.read_u32(0x8c033880).unwrap(); // 0x390
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = base.wrapping_add(cpu.r[0]); // base+0x390
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[3] = base;
    cpu.r[0] = mem.read_u32(0x8c033884).unwrap(); // 0x394
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[3] = base.wrapping_add(cpu.r[0]); // base+0x394
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[1] = base;
    cpu.r[0] = mem.read_u32(0x8c033888).unwrap(); // 0xec
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = base.wrapping_add(cpu.r[0]); // base+0xec
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[0] = mem.read_u32(0x8c03388c).unwrap(); // 0x17c
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // fmul fr2,fr0 (C1', per 0x8c0337e6)
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[6] = base.wrapping_add(cpu.r[0]); // base+0x17c
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.r[0] = base;
    cpu.r[1] = mem.read_u32(0x8c033890).unwrap(); // 0x370
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2]; // [base+0x17c] *= C1
    cpu.r[3] = mem.read_u32(0x8c033894).unwrap(); // 0x278
    cpu.r[0] = base.wrapping_add(cpu.r[1]); // base+0x370
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]);

    // [base+0x370] /= [base+0x278]
    cpu.r[5] = base.wrapping_add(cpu.r[3]); // base+0x278
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[0] = cpu.fr[0] / cpu.fr[1]; // fdiv fr1,fr0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);

    // [base+0x374] /= [base+0x278]; [base+0x378] /= [base+0x278]
    cpu.r[0] = mem.read_u32(0x8c033898).unwrap(); // 0x374
    cpu.r[3] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[1] = base;
    cpu.r[3] = base.wrapping_add(cpu.r[0]); // base+0x374
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[13] = base;
    cpu.r[0] = mem.read_u32(0x8c03389c).unwrap(); // 0x378
    cpu.r[7] = base;
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[1] = base.wrapping_add(cpu.r[0]); // base+0x378
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[5] = base;
    cpu.r[0] = mem.read_u32(0x8c0338a0).unwrap(); // 0x364
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);

    // [base+0x364] *= C3, C3 = [0x8c0338a4]
    cpu.r[13] = base.wrapping_add(cpu.r[0]); // base+0x364
    cpu.r[0] = 0x8c0338a4; // mova
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[1] = base;
    cpu.fr[1] = mem.read_f32(cpu.r[13]).unwrap();
    cpu.r[14] = mem.read_u32(0x8c0338a8).unwrap(); // 0x3e4
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
    cpu.r[6] = mem.read_u32(0x8c0338ac).unwrap(); // 0x31c
    cpu.r[7] = base.wrapping_add(cpu.r[14]); // base+0x3e4
    cpu.r[3] = mem.read_u32(0x8c0338b0).unwrap(); // 0x358
    cpu.r[5] = base.wrapping_add(cpu.r[6]); // base+0x31c
    cpu.r[0] = mem.read_u32(0x8c0338b4).unwrap(); // 0x11c
    cpu.r[1] = base.wrapping_add(cpu.r[3]); // base+0x358
    let _ = mem.write_f32(cpu.r[13], cpu.fr[1]);
    cpu.r[4] = base.wrapping_add(cpu.r[0]); // r4 advanced by 0x11c

    // [base+0x3e4] = ([base+0x31c]*[base+0x358] / ([base+0x11c]*pi)) * [base+0x3e4] / 1000
    cpu.r[0] = 0x8c0338b8; // mova: pi
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap(); // [base+0x11c]
    cpu.r[0] = 0x8c0338bc; // mova: 1000.0
    cpu.fr[3] = mem.read_f32(cpu.r[5]).unwrap(); // [base+0x31c]
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // * pi
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap(); // [base+0x358]
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap(); // [base+0x3e4]
    cpu.fr[3] = cpu.fr[3] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] / cpu.fr[1]; // fdiv fr1,fr3
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // fmul fr0,fr3
    let _ = mem.write_f32(cpu.r[7], cpu.fr[3]);
    cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap();
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // 1000.0
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    let _ = mem.write_f32(cpu.r[7], cpu.fr[1]);

    // epilogue: mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 (delay slot)
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
