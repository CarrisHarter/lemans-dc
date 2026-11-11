//! 0x8c0e8d70 — straight-line FPU transform writing five floats derived from
//! fr4..fr8 to r4+0, +20, +40, +48, +52 and a copy of fr8 at r4+60 (rts delay
//! slot). Reads the ROM constant 0.5f at 0x8c0e8dc0 into fr3 and two floats
//! through the globals 0x8c27ad38 (fr1) and 0x8c27ad40 (fr0). Single path.
//!
//! Hidden side effects the C does not show: r0/r1/r3/r5 are left as the store
//! addresses r4+40/+52/+20/+48, r4 ends at param+60, r6/r7 = 0x8c27ad40/0x8c27ad38,
//! and fr0..fr9 are all clobbered (fr9 = (fr4+fr6)*0.5*fr8 + fr1, etc.).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u32(0x8C0E8DC4).unwrap(); // 0x8c27ad38
    cpu.r[6] = mem.read_u32(0x8C0E8DC8).unwrap(); // 0x8c27ad40
    cpu.fr[9] = cpu.fr[4]; // fmov fr4,fr9
    cpu.fr[2] = cpu.fr[5]; // fmov fr5,fr2
    cpu.fr[3] = mem.read_f32(0x8C0E8DC0).unwrap(); // 0.5f
    cpu.fr[9] = cpu.fr[9] + cpu.fr[6]; // fadd fr6,fr9
    cpu.fr[2] = cpu.fr[2] + cpu.fr[7]; // fadd fr7,fr2
    cpu.fr[6] = cpu.fr[6] - cpu.fr[4]; // fsub fr4,fr6
    cpu.fr[6] = cpu.fr[6] * cpu.fr[3]; // fmul fr3,fr6
    cpu.r[5] = cpu.r[4]; // mov r4,r5
    cpu.fr[7] = cpu.fr[7] - cpu.fr[5]; // fsub fr5,fr7
    cpu.fr[7] = cpu.fr[7] * cpu.fr[3]; // fmul fr3,fr7
    cpu.r[3] = cpu.r[4]; // mov r4,r3
    cpu.fr[7] = -cpu.fr[7]; // fneg fr7
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    cpu.fr[1] = mem.read_f32(cpu.r[7]).unwrap(); // fmov @r7,fr1
    cpu.fr[9] = cpu.fr[9] * cpu.fr[3]; // fmul fr3,fr9
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0
    cpu.fr[6] = cpu.fr[6] * cpu.fr[8]; // fmul fr8,fr6
    cpu.fr[9] = cpu.fr[9] + cpu.fr[1]; // fadd fr1,fr9
    let _ = mem.write_f32(cpu.r[4], cpu.fr[6]); // fmov fr6,@r4
    cpu.fr[2] = cpu.fr[2] * cpu.fr[3]; // fmul fr3,fr2
    cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2
    cpu.r[5] = cpu.r[5].wrapping_add(48); // add #48,r5
    cpu.fr[9] = cpu.fr[9] * cpu.fr[8]; // fmul fr8,fr9
    let _ = mem.write_f32(cpu.r[5], cpu.fr[9]); // fmov fr9,@r5
    cpu.r[3] = cpu.r[3].wrapping_add(20); // add #20,r3
    cpu.fr[7] = cpu.fr[7] * cpu.fr[8]; // fmul fr8,fr7
    let _ = mem.write_f32(cpu.r[3], cpu.fr[7]); // fmov fr7,@r3
    cpu.r[1] = cpu.r[1].wrapping_add(52); // add #52,r1
    cpu.fr[2] = cpu.fr[2] * cpu.fr[8]; // fmul fr8,fr2
    let _ = mem.write_f32(cpu.r[1], cpu.fr[2]); // fmov fr2,@r1
    cpu.r[0] = cpu.r[0].wrapping_add(40); // add #40,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[8]); // fmov fr8,@r0
    cpu.r[4] = cpu.r[4].wrapping_add(60); // add #60,r4
    let _ = mem.write_f32(cpu.r[4], cpu.fr[8]); // rts delay slot: fmov fr8,@r4
}
