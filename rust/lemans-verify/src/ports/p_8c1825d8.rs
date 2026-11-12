//! 0x8c1825d8 — call-free float pipeline over the r4 object.
//!
//! Original (0x8c1825d8..0x8c18262c, pools at 0x8c182630..0x8c182646):
//!   [r4] = 0 (byte); [r4+4] = 0x8c1f0380; [r4+12] = fr4; [r4+16] = fr5
//!   [r6] = (0xc45 * [r6] + 0x362b) & 0xffff, where r6 = [[0x8c26c238]]
//!   fr1 = float([r6]); the 2^32 fixup (mova/fadd) is dead: [r6] is
//!   zero-extended so fr1 >= 0.0 and the bf at 0x8c182616 always skips
//!   [r4+8] = (fr5 - fr4) * fr1 * [0x8c1d1a10] + fr4
//! Exit: r0 = r4 (rts delay slot), r1 = r4+8, r5 = macl + 0x362b,
//! r6 = [P], r7 = r4+12, fpul = [r6], T = (0.0 > fr1), always clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov #0,r0; mov.b r0,@r4
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4], 0);
    // mov r4,r7; mov.l pool,r0; mov r4,r5; mov r4,r1
    cpu.r[7] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c182630).unwrap_or(0x8c1f0380);
    cpu.r[5] = cpu.r[4];
    cpu.r[1] = cpu.r[4];
    // mov.l r0,@(4,r4)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
    // add #12,r7; fmov fr4,@r7
    cpu.r[7] = cpu.r[7].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[7], cpu.fr[4]);
    // add #16,r5; fmov fr5,@r5
    cpu.r[5] = cpu.r[5].wrapping_add(16);
    let _ = mem.write_f32(cpu.r[5], cpu.fr[5]);
    // fldi0 fr0; mov.l pool,r0; add #8,r1
    cpu.fr[0] = 0.0;
    cpu.r[0] = mem.read_u32(0x8c182634).unwrap_or(0x8c26c238);
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    // fmov @r5,fr2; mov.l @r0,r0; mov.l pool,r3
    cpu.fr[2] = mem.read_f32(cpu.r[5]).unwrap_or(cpu.fr[5]);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c182638).unwrap_or(0xc45);
    // mov.l @r0,r6; fmov @r7,fr3; mov.l @r6,r5; mov.l pool,r0
    cpu.r[6] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.fr[3] = mem.read_f32(cpu.r[7]).unwrap_or(cpu.fr[4]);
    cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c18263c).unwrap_or(0x362b);
    // mul.l r3,r5; sts macl,r5; add r0,r5; extu.w r5,r0; mov.l r0,@r6
    let p = (cpu.r[5] as i32 as i64) * (cpu.r[3] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    cpu.r[5] = cpu.macl;
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]);
    cpu.r[0] = cpu.r[5] & 0xffff;
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
    // fsub fr3,fr2
    cpu.fr[2] = cpu.fr[2] - cpu.fr[3];
    // mov.l @r6,r0; lds r0,fpul; float fpul,fr1
    cpu.r[0] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.fpul = cpu.r[0];
    cpu.fr[1] = cpu.fpul as i32 as f32;
    // fcmp/gt fr1,fr0 (T = fr0 > fr1); bf skips the 2^32 fixup unless fr1 < 0
    let take_fixup = cpu.fr[0] > cpu.fr[1];
    cpu.set_t(take_fixup);
    if take_fixup {
        // mova pool,r0; fmov @r0,fr0 (= 2^32); fadd fr0,fr1
        cpu.r[0] = 0x8c182640;
        cpu.fr[0] = mem.read_f32(0x8c182640).unwrap_or(4294967296.0);
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0];
    }
    // mov.l pool,r0; fmul fr1,fr2; fmov @r0,fr0
    cpu.r[0] = mem.read_u32(0x8c182644).unwrap_or(0x8c1d1a10);
    cpu.fr[2] = cpu.fr[2] * cpu.fr[1];
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // fmul fr0,fr2; fadd fr3,fr2; fmov fr2,@r1
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0];
    cpu.fr[2] = cpu.fr[2] + cpu.fr[3];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[2]);
    // rts; delay slot: mov r4,r0
    cpu.r[0] = cpu.r[4];
}
