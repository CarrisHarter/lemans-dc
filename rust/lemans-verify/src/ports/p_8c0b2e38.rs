//! 0x8c0b2e38 — build a 3x3 float matrix at `r4` from the 4 floats at `r5`
//! (the constant 2.0 lives at 0x8c0b2ee0). All single precision.
//!
//! Original (linear FPU sequence, no branches):
//! ```text
//! fmov fr15,@-r15 ; fmov fr14,@-r15 ; fmov fr13,@-r15 ; mov.l r14,@-r15
//! mov r5,r0 ; add #8,r0 ; fmov @r0,fr0        ; fr0  = [r5+8]
//! mov r5,r3 ; mov r5,r1 ; add #4,r3 ; fmov @r3,fr13 ; add #12,r1 ; fmov @r1,fr11
//! mova 0x8c0b2ee0,r0 ; fmov @r0,fr1           ; fr1  = 2.0
//! fmov fr0,fr15 ; fmov fr13,fr14 ; fmul fr1,fr14 ; fmul fr1,fr15
//! fmov fr11,fr10 ; fmov fr15,fr9 ; fmul fr1,fr10 ; fldi1 fr2 ; fmov fr10,fr8
//! fmov @r5,fr4 ; fmov fr15,fr5 ; fmov fr14,fr7 ; fmov fr14,fr6 ; fmov fr14,fr3
//! mov r4,r14 ; mov r4,r7 ; mov r4,r6 ; mov r4,r5 ; mov r4,r3 ; mov r4,r1 ; mov r4,r0
//! fmov fr2,fr1 ; fmul fr0,fr9 ; fmul fr13,fr14 ; fmul fr11,fr8
//! fsub fr9,fr1 ; fsub fr8,fr1 ; fmov fr1,@r4
//! fmul fr0,fr7 ; fsub fr14,fr2 ; fmov fr2,fr0 ; add #16,r14 ; fsub fr8,fr0
//! fmov fr0,@r14 ; fmul fr11,fr5 ; add #32,r7 ; fsub fr9,fr2 ; fmov fr5,fr0
//! fmul fr4,fr3 ; fmov fr2,@r7 ; add #28,r6 ; fsub fr3,fr0 ; fmov fr0,@r6
//! fmul fr11,fr6 ; add #20,r5 ; fadd fr3,fr5 ; fmov fr6,fr0 ; fmul fr4,fr15
//! fmov fr5,@r5 ; add #24,r3 ; fadd fr15,fr0 ; fmov fr0,@r3 ; add #8,r1
//! fsub fr15,fr6 ; fmov fr7,fr0 ; fmul fr4,fr10 ; fmov fr6,@r1 ; add #12,r0
//! fsub fr10,fr0 ; fmov fr0,@r0 ; add #4,r4 ; fadd fr10,fr7 ; fmov fr7,@r4
//! mov.l @r15+,r14 ; fmov @r15+,fr13 ; fmov @r15+,fr14 ; rts ; fmov @r15+,fr15
//! ```
//! Side effects the C hides: the stores land at r4+0, +16, +32, +28, +20, +24,
//! +8, +12, +4 (in that order), `r4` itself ends +4, `r0/r1/r3/r5/r6/r7/r14`
//! all keep r4-derived offsets, and fr13/fr14/fr15/r14 are pushed and popped
//! so the seeded values survive. T is never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // pushed and popped by the prologue/epilogue: seeds survive
    let fr13_seed = cpu.fr[13];
    let fr14_seed = cpu.fr[14];
    let fr15_seed = cpu.fr[15];
    let r14_seed = cpu.r[14];

    cpu.fr[0] = mem.read_f32(cpu.r[5].wrapping_add(8)).unwrap_or(0.0); // [r5+8]
    cpu.fr[13] = mem.read_f32(cpu.r[5].wrapping_add(4)).unwrap_or(0.0); // [r5+4]
    cpu.fr[11] = mem.read_f32(cpu.r[5].wrapping_add(12)).unwrap_or(0.0); // [r5+12]
    cpu.fr[1] = mem.read_f32(0x8c0b2ee0).unwrap_or(0.0); // 2.0
    cpu.fr[15] = cpu.fr[0];
    cpu.fr[14] = cpu.fr[13];
    cpu.fr[14] = cpu.fr[14] * cpu.fr[1];
    cpu.fr[15] = cpu.fr[15] * cpu.fr[1];
    cpu.fr[10] = cpu.fr[11];
    cpu.fr[9] = cpu.fr[15];
    cpu.fr[10] = cpu.fr[10] * cpu.fr[1];
    cpu.fr[2] = 1.0; // fldi1
    cpu.fr[8] = cpu.fr[10];
    cpu.fr[4] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // [r5]
    cpu.fr[5] = cpu.fr[15];
    cpu.fr[7] = cpu.fr[14];
    cpu.fr[6] = cpu.fr[14];
    cpu.fr[3] = cpu.fr[14];
    let r4 = cpu.r[4];
    cpu.r[14] = r4;
    cpu.r[7] = r4;
    cpu.r[6] = r4;
    cpu.r[5] = r4;
    cpu.r[3] = r4;
    cpu.r[1] = r4;
    cpu.r[0] = r4;
    cpu.fr[1] = cpu.fr[2];
    cpu.fr[9] = cpu.fr[9] * cpu.fr[0];
    cpu.fr[14] = cpu.fr[14] * cpu.fr[13];
    cpu.fr[8] = cpu.fr[8] * cpu.fr[11];
    cpu.fr[1] = cpu.fr[1] - cpu.fr[9];
    cpu.fr[1] = cpu.fr[1] - cpu.fr[8];
    let _ = mem.write_f32(r4, cpu.fr[1]); // [r4+0]
    cpu.fr[7] = cpu.fr[7] * cpu.fr[0];
    cpu.fr[2] = cpu.fr[2] - cpu.fr[14];
    cpu.fr[0] = cpu.fr[2];
    cpu.r[14] = cpu.r[14].wrapping_add(16);
    cpu.fr[0] = cpu.fr[0] - cpu.fr[8];
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]); // [r4+16]
    cpu.fr[5] = cpu.fr[5] * cpu.fr[11];
    cpu.r[7] = cpu.r[7].wrapping_add(32);
    cpu.fr[2] = cpu.fr[2] - cpu.fr[9];
    cpu.fr[0] = cpu.fr[5];
    cpu.fr[3] = cpu.fr[3] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[7], cpu.fr[2]); // [r4+32]
    cpu.r[6] = cpu.r[6].wrapping_add(28);
    cpu.fr[0] = cpu.fr[0] - cpu.fr[3];
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]); // [r4+28]
    cpu.fr[6] = cpu.fr[6] * cpu.fr[11];
    cpu.r[5] = cpu.r[5].wrapping_add(20);
    cpu.fr[5] = cpu.fr[5] + cpu.fr[3];
    cpu.fr[0] = cpu.fr[6];
    cpu.fr[15] = cpu.fr[15] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[5], cpu.fr[5]); // [r4+20]
    cpu.r[3] = cpu.r[3].wrapping_add(24);
    cpu.fr[0] = cpu.fr[0] + cpu.fr[15];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // [r4+24]
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    cpu.fr[6] = cpu.fr[6] - cpu.fr[15];
    cpu.fr[0] = cpu.fr[7];
    cpu.fr[10] = cpu.fr[10] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[6]); // [r4+8]
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    cpu.fr[0] = cpu.fr[0] - cpu.fr[10];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // [r4+12]
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[7] = cpu.fr[7] + cpu.fr[10];
    let _ = mem.write_f32(cpu.r[4], cpu.fr[7]); // [r4+4]

    // epilogue pops
    cpu.fr[13] = fr13_seed;
    cpu.fr[14] = fr14_seed;
    cpu.fr[15] = fr15_seed;
    cpu.r[14] = r14_seed;
}
