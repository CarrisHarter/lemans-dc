//! 0x8c0c0328 — 3D plane/ray style FPU test; returns 1 or 0 in r0.
//!
//! Reads two floats each from r4..r7 (advancing each pointer by 4 — the
//! `add #4,rN` instructions are observable side effects), computes a
//! determinant in fr5, and:
//! ```text
//! fcmp/eq fr4,fr5 ; bf 0x8c0c0364 ; rts ; mov #0,r0 (delay)  ; det == 0 → 0
//! … fdiv/fmul chain, stores fr1 to *r1 and fr11 to *r0 …
//! fcmp/gt fr4,fr0 ; bf 0x8c0c039e   (→ rts ; mov #0,r0)
//! fcmp/gt fr0,fr3 ; bf 0x8c0c039e
//! fmov @r0,fr0 ; fcmp/gt fr4,fr0 ; bf 0x8c0c039e
//! fcmp/gt fr0,fr3 ; bf 0x8c0c039e
//! rts ; mov #1,r0 (delay)
//! ```
//! r1 = *r15 and r0 = *(r15+4) (stack-passed out pointers). Hidden side
//! effects: r4..r7 each end +4, r0 ends as the 0/1 result, fr0..fr11 all
//! hold intermediate values, fr4 = 0.0 and fr3 = 1.0, T follows the last
//! fcmp. Note the early-exit path can produce -0.0 values that still
//! compare equal to +0.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap();
    cpu.fr[10] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.fr[4] = 0.0; // fldi0 fr4
    cpu.fr[9] = mem.read_f32(cpu.r[7]).unwrap();
    cpu.r[6] = cpu.r[6].wrapping_add(4);
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.r[7] = cpu.r[7].wrapping_add(4);
    cpu.fr[11] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[9] = cpu.fr[9] - cpu.fr[10]; // fsub fr10,fr9
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[11] = cpu.fr[11] - cpu.fr[2]; // fsub fr2,fr11
    cpu.fr[9] = -cpu.fr[9]; // fneg fr9
    cpu.fr[7] = mem.read_f32(cpu.r[6]).unwrap();
    cpu.fr[5] = cpu.fr[11]; // fmov fr11,fr5
    cpu.fr[6] = mem.read_f32(cpu.r[7]).unwrap();
    cpu.fr[0] = cpu.fr[9]; // fmov fr9,fr0
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[6] = cpu.fr[6] - cpu.fr[7]; // fsub fr7,fr6
    cpu.fr[8] = mem.read_f32(cpu.r[5]).unwrap();
    cpu.fr[6] = -cpu.fr[6]; // fneg fr6
    cpu.fr[5] = cpu.fr[5] * cpu.fr[6]; // fmul fr6,fr5
    cpu.fr[8] = cpu.fr[8] - cpu.fr[1]; // fsub fr1,fr8
    cpu.fr[0] = cpu.fr[0] * cpu.fr[8]; // fmul fr8,fr0
    cpu.fr[5] = cpu.fr[5] - cpu.fr[0]; // fsub fr0,fr5
    // fcmp/eq fr4,fr5 → T = fr5 == fr4 ; bf 0x8c0c0364
    if cpu.fr[5] == cpu.fr[4] {
        cpu.set_t(true);
        cpu.r[0] = 0; // rts delay: mov #0,r0
        return;
    }
    cpu.set_t(false);
    cpu.fr[7] = cpu.fr[7] - cpu.fr[1]; // fsub fr1,fr7
    cpu.fr[10] = cpu.fr[10] - cpu.fr[2]; // fsub fr2,fr10
    cpu.fr[1] = cpu.fr[10]; // fmov fr10,fr1
    cpu.fr[0] = cpu.fr[7]; // fmov fr7,fr0
    cpu.fr[3] = 1.0; // fldi1 fr3
    cpu.fr[2] = cpu.fr[3]; // fmov fr3,fr2
    cpu.fr[1] = cpu.fr[1] * cpu.fr[6]; // fmul fr6,fr1
    cpu.fr[0] = cpu.fr[0] * cpu.fr[9]; // fmul fr9,fr0
    cpu.fr[2] = cpu.fr[2] / cpu.fr[5]; // fdiv fr5,fr2
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[2]; // fmul fr2,fr1
    cpu.fr[11] = cpu.fr[11] * cpu.fr[7]; // fmul fr7,fr11
    cpu.fr[8] = cpu.fr[8] * cpu.fr[10]; // fmul fr10,fr8
    cpu.fr[11] = cpu.fr[11] - cpu.fr[8]; // fsub fr8,fr11
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    cpu.fr[11] = cpu.fr[11] * cpu.fr[2]; // fmul fr2,fr11
    let _ = mem.write_f32(cpu.r[0], cpu.fr[11]); // fmov fr11,@r0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
    let t1 = cpu.fr[0] > cpu.fr[4]; // fcmp/gt fr4,fr0
    cpu.set_t(t1);
    if !t1 {
        cpu.r[0] = 0; // bf 0x8c0c039e ; rts ; mov #0,r0
        return;
    }
    let t2 = cpu.fr[3] > cpu.fr[0]; // fcmp/gt fr0,fr3
    cpu.set_t(t2);
    if !t2 {
        cpu.r[0] = 0;
        return;
    }
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
    let t3 = cpu.fr[0] > cpu.fr[4]; // fcmp/gt fr4,fr0
    cpu.set_t(t3);
    if !t3 {
        cpu.r[0] = 0;
        return;
    }
    let t4 = cpu.fr[3] > cpu.fr[0]; // fcmp/gt fr0,fr3
    cpu.set_t(t4);
    if !t4 {
        cpu.r[0] = 0;
        return;
    }
    cpu.r[0] = 1; // rts ; mov #1,r0 (delay)
}
