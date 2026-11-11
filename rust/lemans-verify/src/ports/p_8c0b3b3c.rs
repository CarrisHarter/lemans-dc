//! 0x8c0b3b3c — build a face normal from the 3x3 matrix at r4 and write four
//! floats to the pointer passed on the stack (@(44,r15), loaded into r12).
//!
//! CFG: `fcmp/gt fr0,fr2` on sum = f[r4]+f[r4+16]+f[r4+32] splits into
//!   positive (sum > 0, 0x8c0b3b7a): fr3 = 0.5*sqrt(sum+1), fr1 = 0.5/sqrt,
//!     fr4 = ([r4+20]-[r4+28])*fr1, fr2 = ([r4+24]-[r4+8])*fr1,
//!     fr5 = ([r4+4]-[r4+12])*fr1 (bra delay slot), then the common tail;
//!   non-positive (0x8c0b3bb2): k = (f[r4+16] > f[r4]) ? 1 : 0, upgraded to 2
//!     when f[r4+32] > f[r4+16k]; permutation table at 0x8c1fcaf0 = [1,2,0]
//!     gives i = table[k], j = table[i]; s = sqrt(f[r4+32]-f[r4]-f[r4+16]+1)
//!     (division skipped when s == 0 via bt.s), three differences scaled by
//!     0.5/s are staged through stack slots [r15+12+4k], [r15+24],
//!     [r15+12+4i], [r15+12+4j] and read back as fr3=[r15+24], fr4=[r15+12],
//!     fr2=[r15+16], fr5=[r15+20].
//! Common tail (0x8c0b3c90) stores fr3,fr4,fr2,fr5 to out+0,+4,+8,+12.
//!
//! Hidden side effects the C does not show: the output pointer comes from the
//! stack, not r5 (r5 is clobbered with the table pointer 0x8c1fcaf0); the
//! stack slots are written before being read back (matters when an index is
//! 0, so [r15+12] feeds fr4); r12/r11/r13/r14 are restored by the epilogue
//! pops, so r12 ends as the *seeded* r12, not the output pointer; pointer
//! arithmetic leaves r2 = r15+24, r3 = r15+12+4i, r4 = r15, r5 = 4k and
//! advanced input pointers in r6/r7 (negative path) or r3..r7 (positive);
//! T ends as the `fcmp/eq fr0,fr4` (sqrt == 0) on the negative path and as
//! the `fcmp/gt` (sum > 0) on the positive one.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[15];
    cpu.r[15] = s.wrapping_sub(44); // 4 pushes + add #-28,r15
    let sp = cpu.r[15];
    let r4o = cpu.r[4];
    let seed_r12 = cpu.r[12];
    let out = mem.read_u32(sp.wrapping_add(44)).unwrap(); // mov.l @(44,r15),r12
    let table = mem.read_u32(0x8c0b3cb0).unwrap(); // 0x8c1fcaf0
    cpu.r[5] = table;
    let r7 = r4o.wrapping_add(12);
    let r3 = r4o.wrapping_add(24);
    mem.write_u32(sp, mem.read_u32(table).unwrap()).unwrap();
    mem.write_u32(sp.wrapping_add(4), mem.read_u32(table.wrapping_add(4)).unwrap()).unwrap();
    mem.write_u32(sp.wrapping_add(8), mem.read_u32(table.wrapping_add(8)).unwrap()).unwrap();
    cpu.r[7] = r7;
    cpu.r[3] = r3;
    cpu.fr[0] = 0.0; // fldi0 fr0
    let r1 = r7.wrapping_add(4); // r4+16
    let r0 = r3.wrapping_add(8); // r4+32
    cpu.fr[4] = mem.read_f32(r4o).unwrap();
    cpu.fr[3] = mem.read_f32(r1).unwrap();
    cpu.fr[2] = cpu.fr[4]; // fmov fr4,fr2
    cpu.fr[1] = mem.read_f32(r0).unwrap();
    cpu.fr[2] = cpu.fr[2] + cpu.fr[3]; // fadd fr3,fr2
    cpu.fr[2] = cpu.fr[2] + cpu.fr[1]; // fadd fr1,fr2
    cpu.set_t(cpu.fr[2] > cpu.fr[0]); // fcmp/gt fr0,fr2

    if cpu.t() {
        // ---- positive path (sum > 0), 0x8c0b3b7a..0x8c0b3bb0 ----
        cpu.r[6] = r7; // mov r7,r6
        cpu.r[5] = r3; // mov r3,r5
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.r[0] = 0x8c0b3cb4; // mova 0x8c0b3cb4,r0
        cpu.r[1] = r4o; // mov r4,r1
        cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2
        cpu.r[6] = cpu.r[6].wrapping_add(8); // r4+20
        cpu.r[5] = cpu.r[5].wrapping_add(4); // r4+28
        cpu.fr[2] = cpu.fr[2].sqrt(); // fsqrt fr2
        cpu.r[4] = r4o.wrapping_add(4);
        cpu.fr[1] = mem.read_f32(0x8c0b3cb4).unwrap(); // 0.5
        cpu.fr[3] = cpu.fr[2]; // fmov fr2,fr3
        cpu.fr[4] = mem.read_f32(cpu.r[6]).unwrap(); // f[r4+20]
        cpu.fr[3] = cpu.fr[3] * cpu.fr[1]; // fmul fr1,fr3
        cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap(); // f[r4+28]
        cpu.fr[1] = cpu.fr[1] / cpu.fr[2]; // fdiv fr2,fr1
        cpu.r[1] = cpu.r[1].wrapping_add(8); // r4+8
        cpu.fr[4] = cpu.fr[4] - cpu.fr[0]; // fsub fr0,fr4
        cpu.fr[2] = mem.read_f32(r3).unwrap(); // f[r4+24]
        cpu.fr[4] = cpu.fr[4] * cpu.fr[1]; // fmul fr1,fr4
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // f[r4+8]
        cpu.fr[5] = mem.read_f32(cpu.r[4]).unwrap(); // f[r4+4]
        cpu.fr[2] = cpu.fr[2] - cpu.fr[0]; // fsub fr0,fr2
        cpu.fr[0] = mem.read_f32(r7).unwrap(); // f[r4+12]
        cpu.fr[2] = cpu.fr[2] * cpu.fr[1]; // fmul fr1,fr2
        cpu.fr[5] = cpu.fr[5] - cpu.fr[0]; // fsub fr0,fr5
        cpu.fr[5] = cpu.fr[5] * cpu.fr[1]; // bra delay slot: fmul fr1,fr5
    } else {
        // ---- non-positive path, 0x8c0b3bb2..0x8c0b3c8e ----
        cpu.set_t(cpu.fr[3] > cpu.fr[4]); // fcmp/gt fr4,fr3
        let mut k = if cpu.t() { 1u32 } else { 0u32 }; // bf.s; delay mov #0,r1
        let r5v = r4o.wrapping_add(16u32.wrapping_mul(k)); // r4+16k
        let r3v = r4o.wrapping_add(32); // add #8,r3
        cpu.fr[1] = mem.read_f32(r5v).unwrap();
        cpu.fr[0] = mem.read_f32(r3v).unwrap();
        cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
        if cpu.t() {
            k = 2; // bf falls through into mov #2,r1
        }
        let r5b = 4u32.wrapping_mul(k); // r5 = 4k
        let i = mem.read_u32(sp.wrapping_add(r5b)).unwrap(); // local[k]
        let j = mem.read_u32(sp.wrapping_add(4u32.wrapping_mul(i))).unwrap(); // local[i]
        let r13v = r4o.wrapping_add(12u32.wrapping_mul(k));
        let r7v = r4o.wrapping_add(12u32.wrapping_mul(i));
        let r6v = r4o.wrapping_add(12u32.wrapping_mul(j));
        let r1v = r7v.wrapping_add(4u32.wrapping_mul(i)); // r4+16i
        let r11v = r13v.wrapping_add(r5b); // r4+16k
        cpu.fr[3] = mem.read_f32(r1v).unwrap();
        let r4v = r6v.wrapping_add(4u32.wrapping_mul(j)); // r4+16j
        cpu.fr[1] = mem.read_f32(r4v).unwrap();
        cpu.fr[0] = 1.0; // fldi1 fr0
        cpu.fr[4] = mem.read_f32(r11v).unwrap();
        cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // fadd fr1,fr3
        cpu.r[4] = sp; // mov r15,r4
        cpu.fr[4] = cpu.fr[4] - cpu.fr[3]; // fsub fr3,fr4
        cpu.fr[4] = cpu.fr[4] + cpu.fr[0]; // fadd fr0,fr4
        cpu.fr[4] = cpu.fr[4].sqrt(); // fsqrt fr4
        cpu.fr[0] = 0.0; // fldi0 fr0
        cpu.fr[2] = mem.read_f32(0x8c0b3cb4).unwrap(); // 0.5
        cpu.fr[1] = cpu.fr[4]; // fmov fr4,fr1
        let r1c = sp.wrapping_add(12).wrapping_add(r5b); // r15+12+4k
        cpu.fr[1] = cpu.fr[1] * cpu.fr[2]; // fmul fr2,fr1
        cpu.set_t(cpu.fr[4] == cpu.fr[0]); // fcmp/eq fr0,fr4
        mem.write_f32(r1c, cpu.fr[1]).unwrap(); // bt.s delay: fmov fr1,@r1
        if !cpu.t() {
            cpu.fr[2] = cpu.fr[2] / cpu.fr[4]; // fdiv fr4,fr2
            cpu.fr[4] = cpu.fr[2]; // fmov fr2,fr4
        }
        // 8c0b3c3c..8c0b3c8e, exact order (stores before the stack reads that
        // depend on them):
        let r0a = r7v.wrapping_add(4u32.wrapping_mul(j));
        let r1a = r6v.wrapping_add(4u32.wrapping_mul(i));
        cpu.fr[1] = mem.read_f32(r0a).unwrap();
        cpu.fr[0] = mem.read_f32(r1a).unwrap();
        cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
        cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
        mem.write_f32(sp.wrapping_add(24), cpu.fr[1]).unwrap(); // fmov fr1,@r2
        let r0b = r13v.wrapping_add(4u32.wrapping_mul(i));
        let r7b = r7v.wrapping_add(r5b);
        let r13b = r13v.wrapping_add(4u32.wrapping_mul(j));
        cpu.fr[1] = mem.read_f32(r0b).unwrap();
        let r6b = r6v.wrapping_add(r5b);
        cpu.fr[0] = mem.read_f32(r7b).unwrap();
        let r3c = sp.wrapping_add(12).wrapping_add(4u32.wrapping_mul(i));
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
        mem.write_f32(r3c, cpu.fr[1]).unwrap(); // fmov fr1,@r3 -> [r15+12+4i]
        let r14c = sp.wrapping_add(12).wrapping_add(4u32.wrapping_mul(j));
        cpu.fr[1] = mem.read_f32(r13b).unwrap();
        cpu.fr[0] = mem.read_f32(r6b).unwrap();
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
        mem.write_f32(r14c, cpu.fr[1]).unwrap(); // fmov fr1,@r14 -> [r15+12+4j]
        cpu.fr[4] = mem.read_f32(sp.wrapping_add(12)).unwrap();
        cpu.fr[2] = mem.read_f32(sp.wrapping_add(16)).unwrap();
        cpu.fr[5] = mem.read_f32(sp.wrapping_add(20)).unwrap();
        cpu.fr[3] = mem.read_f32(sp.wrapping_add(24)).unwrap();
        // Final GPR state of the negative path (before the tail reuses r0/r1):
        cpu.r[2] = sp.wrapping_add(24); // mov #24,r2; add r15,r2
        cpu.r[3] = r3c; // add r1,r3 with r1 = r15+12
        cpu.r[4] = sp.wrapping_add(12); // mov r15,r4 then add #12,r4
        cpu.r[5] = r5b;
        cpu.r[6] = r6b;
        cpu.r[7] = r7b;
    }

    // ---- common tail, 0x8c0b3c90 ----
    cpu.r[1] = out; // mov r12,r1
    mem.write_f32(out, cpu.fr[3]).unwrap(); // fmov fr3,@r12
    cpu.r[0] = out; // mov r12,r0
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    mem.write_f32(cpu.r[1], cpu.fr[4]).unwrap(); // fmov fr4,@r1
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    mem.write_f32(cpu.r[0], cpu.fr[2]).unwrap(); // fmov fr2,@r0
    cpu.r[12] = out.wrapping_add(12); // add #12,r12 (overwritten by the pop)
    mem.write_f32(cpu.r[12], cpu.fr[5]).unwrap(); // fmov fr5,@r12
    cpu.r[12] = seed_r12; // epilogue pops restore r11..r14
    cpu.r[15] = s; // add #28,r15 + four pops
}
