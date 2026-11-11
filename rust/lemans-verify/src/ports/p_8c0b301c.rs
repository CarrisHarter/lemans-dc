//! 0x8c0b301c — normalize a 3x3 rotation matrix (r5) and write the normalized
//! matrix to r4. Straight-line FPU code: nine `fmov` loads from r5+0..r5+0x20,
//! a cross product whose z component (fr5) is inverted into fr0, then nine
//! scaled differences stored to r4 in the fixed order
//! +0,+12,+24,+4,+16,+28,+8,+20,+32.
//!
//! Hidden side effects the C does not show: the epilogue pops restore r8..r14
//! to their entry values, so only r0..r7 keep working values afterwards
//! (r0 = r4+20, r1 = r4+8, r3 = r4+28, r4 = r4+32, r6 = r5+12, r7 = r5+24);
//! the FPU registers fr1..fr8 keep their last loaded/computed values
//! (fr0 = 1/cross, fr5 = the cross product itself).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let f = |mem: &Memory, a: u32| mem.read_f32(a).unwrap();
    // Prologue: seven pushes; the stack window is excluded from comparison.
    cpu.r[15] = cpu.r[15].wrapping_sub(28);
    let seed = [
        cpu.r[8], cpu.r[9], cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14],
    ];
    let r5 = cpu.r[5];
    cpu.r[12] = r5.wrapping_add(4);
    cpu.r[9] = r5.wrapping_add(16);
    cpu.r[11] = r5.wrapping_add(20);
    cpu.r[13] = r5.wrapping_add(28);
    cpu.r[14] = r5.wrapping_add(32);
    cpu.r[10] = r5.wrapping_add(8);
    cpu.r[6] = r5.wrapping_add(12);
    cpu.r[7] = r5.wrapping_add(24);

    cpu.fr[1] = f(mem, cpu.r[12]);
    cpu.fr[8] = f(mem, cpu.r[9]);
    cpu.fr[3] = f(mem, cpu.r[10]);
    cpu.fr[4] = cpu.fr[3];
    cpu.fr[2] = f(mem, cpu.r[11]);
    cpu.fr[5] = cpu.fr[1];
    cpu.fr[7] = f(mem, cpu.r[13]);
    cpu.fr[5] = cpu.fr[5] * cpu.fr[2]; // fmul fr2,fr5
    cpu.fr[4] = cpu.fr[4] * cpu.fr[8]; // fmul fr8,fr4
    cpu.fr[2] = cpu.fr[2] * cpu.fr[7]; // fmul fr7,fr2
    cpu.fr[0] = f(mem, cpu.r[14]);
    cpu.fr[6] = cpu.fr[8];
    cpu.fr[6] = cpu.fr[6] * cpu.fr[0]; // fmul fr0,fr6
    cpu.fr[6] = cpu.fr[6] - cpu.fr[2]; // fsub fr2,fr6
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // fmul fr1,fr0
    cpu.fr[7] = cpu.fr[7] * cpu.fr[3]; // fmul fr3,fr7
    cpu.fr[7] = cpu.fr[7] - cpu.fr[0]; // fsub fr0,fr7
    cpu.fr[2] = f(mem, cpu.r[6]);
    cpu.r[3] = cpu.r[4]; // mov r4,r3
    cpu.fr[1] = f(mem, r5);
    cpu.fr[0] = cpu.fr[6]; // fmov fr6,fr0
    cpu.fr[3] = f(mem, cpu.r[7]);
    cpu.fr[5] = cpu.fr[5] - cpu.fr[4]; // fsub fr4,fr5
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.fr[7] = cpu.fr[2] * cpu.fr[7]; // fmul fr2,fr7
    cpu.fr[0] = cpu.fr[1] * cpu.fr[0]; // fmul fr1,fr0
    cpu.fr[7] = cpu.fr[7] + cpu.fr[0]; // fadd fr0,fr7
    cpu.fr[5] = cpu.fr[3] * cpu.fr[5]; // fmul fr3,fr5
    cpu.fr[5] = cpu.fr[5] + cpu.fr[7]; // fadd fr7,fr5
    cpu.fr[0] = 1.0; // fldi1 fr0
    cpu.fr[0] = cpu.fr[0] / cpu.fr[5]; // fdiv fr5,fr0
    cpu.fr[6] = cpu.fr[0] * cpu.fr[6]; // fmul fr0,fr6
    mem.write_f32(cpu.r[4], cpu.fr[6]).unwrap(); // fmov fr6,@r4   -> [r4+0]

    cpu.r[3] = cpu.r[3].wrapping_add(12);
    cpu.fr[3] = f(mem, cpu.r[11]);
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    cpu.fr[1] = f(mem, cpu.r[7]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.fr[2] = f(mem, cpu.r[14]);
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[6]);
    cpu.r[8] = cpu.r[4]; // mov r4,r8
    cpu.r[1] = cpu.r[1].wrapping_add(24);
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[3], cpu.fr[3]).unwrap(); // -> [r4+12]

    cpu.r[8] = cpu.r[8].wrapping_add(16);
    cpu.fr[3] = f(mem, cpu.r[6]);
    cpu.fr[1] = f(mem, cpu.r[13]);
    cpu.fr[2] = f(mem, cpu.r[7]);
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[9]);
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[1], cpu.fr[3]).unwrap(); // -> [r4+24]

    cpu.fr[3] = f(mem, cpu.r[13]);
    cpu.fr[1] = f(mem, cpu.r[10]);
    cpu.fr[2] = f(mem, cpu.r[12]);
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[14]);
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[0], cpu.fr[3]).unwrap(); // -> [r4+4]

    cpu.fr[3] = f(mem, cpu.r[14]);
    cpu.fr[1] = f(mem, r5);
    cpu.fr[2] = f(mem, cpu.r[10]);
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[7]);
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[8], cpu.fr[3]).unwrap(); // -> [r4+16]

    cpu.r[3] = cpu.r[4]; // mov r4,r3
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    cpu.r[3] = cpu.r[3].wrapping_add(28);
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    cpu.r[0] = cpu.r[0].wrapping_add(20);
    cpu.r[4] = cpu.r[4].wrapping_add(32);

    cpu.fr[3] = f(mem, cpu.r[7]); // f[r5+24]
    cpu.fr[1] = f(mem, cpu.r[12]); // f[r5+4]
    cpu.fr[2] = f(mem, r5); // f[r5+0]
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[13]); // f[r5+28]
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[3], cpu.fr[3]).unwrap(); // -> [r4+28]

    cpu.fr[3] = f(mem, cpu.r[12]); // f[r5+4]
    cpu.fr[1] = f(mem, cpu.r[11]); // f[r5+20]
    cpu.fr[2] = f(mem, cpu.r[9]); // f[r5+16]
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[10]); // f[r5+8]
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[1], cpu.fr[3]).unwrap(); // -> [r4+8]

    cpu.fr[3] = f(mem, cpu.r[10]);
    cpu.fr[1] = f(mem, cpu.r[6]);
    cpu.fr[2] = f(mem, cpu.r[11]);
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, r5);
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[0], cpu.fr[3]).unwrap(); // -> [r4+20]

    cpu.fr[3] = f(mem, r5);
    cpu.fr[1] = f(mem, cpu.r[9]);
    cpu.fr[2] = f(mem, cpu.r[6]);
    cpu.fr[3] = cpu.fr[1] * cpu.fr[3];
    cpu.fr[1] = f(mem, cpu.r[12]);
    cpu.fr[2] = cpu.fr[1] * cpu.fr[2];
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    cpu.fr[3] = cpu.fr[0] * cpu.fr[3];
    mem.write_f32(cpu.r[4], cpu.fr[3]).unwrap(); // -> [r4+32]

    // Epilogue: mov.l @r15+,r8..r14 (rts delay slot) restores the saved regs.
    cpu.r[8] = seed[0];
    cpu.r[9] = seed[1];
    cpu.r[10] = seed[2];
    cpu.r[11] = seed[3];
    cpu.r[12] = seed[4];
    cpu.r[13] = seed[5];
    cpu.r[14] = seed[6];
    cpu.r[15] = cpu.r[15].wrapping_add(28);
}
