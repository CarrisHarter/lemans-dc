//! 0x8c0ac3c4 — store `fr4/fr5/fr6/fr7` into a struct at `r4` and, when the
//! flag byte at `r4+0x24` is nonzero, clamp `r4+0x10` into [-|fr3|/2, |fr3|/2]
//! (all single precision; the 0.5 constant lives at 0x8c0ac430).
//!
//! Original:
//! ```text
//! mov r4,r5 ; add #24,r5 ; mov r4,r0 ; fmov fr6,@r5 ; add #28,r0 ; mov r4,r3
//! fmov fr7,@r0 ; add #32,r3 ; mov r4,r1 ; mov r4,r0 ; fmov fr4,@r3
//! add #12,r1 ; fldi0 fr0 ; fmov fr0,@r1 ; add #20,r0 ; fsub fr7,fr5
//! fmov fr5,@r0 ; mov r4,r1 ; mov r4,r0 ; fmov @r3,fr1 ; add #36,r0
//! fmov @r5,fr0 ; add #16,r1 ; fsub fr0,fr1 ; fmov fr1,@r1 ; mov.b @r0,r0
//! extu.b r0,r0 ; extu.b r0,r0 ; tst r0,r0 ; bt 0x8c0ac42a
//! mov r4,r0 ; add #40,r0 ; fmov @r0,fr3 ; fmov @r1,fr2 ; mova 0x8c0ac430,r0
//! fmov @r0,fr0 ; fmov fr3,fr1 ; fneg fr1 ; fmul fr0,fr1 ; fcmp/gt fr2,fr1
//! bf skip1 ; fadd fr3,fr2
//! skip1: mova 0x8c0ac430,r0 ; fmov @r0,fr1 ; fmov fr3,fr0 ; fmul fr1,fr0
//! fcmp/gt fr2,fr0 ; bt skip2 ; fsub fr3,fr2
//! skip2: add #16,r4 ; fmov fr2,@r4
//! rts ; nop
//! ```
//! Side effects the C hides: `r4` itself is advanced by 16 on the flag path
//! (the final store lands on the original `r4+0x10`), `r0` keeps 0x8c0ac430
//! (flag path) or the flag byte (skip path), `r1` = r4+0x10, `r3` = r4+0x20,
//! `r5` = r4+0x18, `fr0`/`fr1` keep intermediate products, and T ends as the
//! last `fcmp/gt` (flag path) or 1 (skip path, `tst` of the zero flag byte).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    cpu.r[5] = r4.wrapping_add(24);
    let _ = mem.write_f32(cpu.r[5], cpu.fr[6]); // [r4+24] = fr6
    cpu.r[0] = r4.wrapping_add(28);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[7]); // [r4+28] = fr7
    cpu.r[3] = r4.wrapping_add(32);
    cpu.r[1] = r4;
    cpu.r[0] = r4;
    let _ = mem.write_f32(cpu.r[3], cpu.fr[4]); // [r4+32] = fr4
    cpu.r[1] = cpu.r[1].wrapping_add(12);
    cpu.fr[0] = 0.0; // fldi0
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // [r4+12] = 0.0
    cpu.r[0] = r4.wrapping_add(20);
    cpu.fr[5] = cpu.fr[5] - cpu.fr[7]; // fsub fr7,fr5
    let _ = mem.write_f32(cpu.r[0], cpu.fr[5]); // [r4+20] = fr5
    cpu.r[1] = r4;
    cpu.r[0] = r4;
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // [r4+32]
    cpu.r[0] = r4.wrapping_add(36);
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // [r4+24]
    cpu.r[1] = r4.wrapping_add(16);
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // [r4+16] = fr1
    let flag = mem.read_u8(cpu.r[0]).unwrap_or(0); // mov.b + extu.b x2
    cpu.r[0] = flag as u32;
    cpu.set_t(flag == 0);
    if flag != 0 {
        cpu.r[0] = r4.wrapping_add(40);
        cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // [r4+40]
        cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // [r4+16]
        cpu.r[0] = 0x8c0ac430; // mova
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0.5
        cpu.fr[1] = cpu.fr[3];
        cpu.fr[1] = -cpu.fr[1]; // fneg
        cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
        cpu.set_t(cpu.fr[1] > cpu.fr[2]); // fcmp/gt fr2,fr1
        if cpu.fr[1] > cpu.fr[2] {
            cpu.fr[2] = cpu.fr[2] + cpu.fr[3]; // fadd fr3,fr2
        }
        cpu.r[0] = 0x8c0ac430; // mova
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 0.5
        cpu.fr[0] = cpu.fr[3];
        cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // fmul fr1,fr0
        cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt fr2,fr0
        if !(cpu.fr[0] > cpu.fr[2]) {
            cpu.fr[2] = cpu.fr[2] - cpu.fr[3]; // fsub fr3,fr2
        }
        cpu.r[4] = cpu.r[4].wrapping_add(16); // add #16,r4
        let _ = mem.write_f32(cpu.r[4], cpu.fr[2]); // [r4_orig+16] = fr2
    }
}
