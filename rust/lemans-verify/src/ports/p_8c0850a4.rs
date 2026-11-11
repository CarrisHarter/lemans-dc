//! 0x8c0850a4 — compute `[r4+4] = (B[2]*fr4 + B[1]) - (h/1000 + m + s*60 +
//! ms*3600)` and `[r4] = 1`, where B = *(float**)0x8c26c230 and the four
//! int16 args sit at @r15, @(2,r15), @(4,r15), @(6,r15).
//!
//! Original side effects the C does not show: r0 = 1, r1 = r4+4, r3 = ms,
//! r5 = s, r6 = m, fpul = m (last lds), fr1 = B[2]*fr4+B[1], fr2 = 3600.0,
//! fr3 = ms*3600, fr5 = 60.0, fr6 = s*60, fr7 = 1000.0, fr8 = the divisor
//! sum, fr0 = the stored result. Stores in order: f32 at r4+4 (fr1), f32 at
//! r4+4 (fr0), then u32 1 at r4 in the rts delay slot. No T bit is touched.

use sh4_oracle::{Cpu, Memory};

fn exts_w(v: u16) -> u32 {
    (v as i16 as i32) as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sp = cpu.r[15];
    cpu.r[0] = mem.read_u16(sp).unwrap_or(0) as u32; // mov.w @r15,r0
    let glob = mem.read_u32(0x8c085100).unwrap_or(0); // 0x8c26c230
    cpu.r[5] = exts_w(cpu.r[0] as u16); // exts.w r0,r5  (h)
    cpu.r[3] = mem.read_u32(glob).unwrap_or(0); // mov.l @r1,r3  (B)
    cpu.fr[7] = mem.read_f32(0x8c085104).unwrap_or(0.0); // 1000.0f
    let b2 = cpu.r[3].wrapping_add(8);
    cpu.r[0] = mem.read_u16(sp + 2).unwrap_or(0) as u32;
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.fr[1] = mem.read_f32(b2).unwrap_or(0.0); // fmov @r1,fr1
    cpu.r[6] = exts_w(cpu.r[0] as u16); // exts.w r0,r6  (m)
    cpu.r[0] = mem.read_u16(sp + 4).unwrap_or(0) as u32;
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.fpul = cpu.r[5]; // lds r5,fpul
    cpu.r[5] = exts_w(cpu.r[0] as u16); // exts.w r0,r5  (s)
    cpu.fr[5] = mem.read_f32(0x8c085108).unwrap_or(0.0); // 60.0f
    cpu.fr[8] = cpu.fpul as i32 as f32; // float fpul,fr8
    cpu.fpul = cpu.r[5]; // lds r5,fpul
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr0
    cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
    cpu.r[0] = mem.read_u16(sp + 6).unwrap_or(0) as u32;
    cpu.fr[6] = cpu.fpul as i32 as f32; // float fpul,fr6
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    cpu.r[3] = exts_w(cpu.r[0] as u16); // exts.w r0,r3  (ms)
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    cpu.fr[2] = mem.read_f32(0x8c08510c).unwrap_or(0.0); // 3600.0f
    cpu.fpul = cpu.r[3]; // lds r3,fpul
    cpu.fr[8] = cpu.fr[8] / cpu.fr[7]; // fdiv fr7,fr8
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    cpu.fr[3] = cpu.fpul as i32 as f32; // float fpul,fr3
    cpu.fpul = cpu.r[6]; // lds r6,fpul
    cpu.fr[6] = cpu.fr[6] * cpu.fr[5]; // fmul fr5,fr6
    cpu.fr[3] = cpu.fr[3] * cpu.fr[2]; // fmul fr2,fr3
    cpu.r[0] = 1; // mov #1,r0
    cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0
    cpu.fr[8] = cpu.fr[8] + cpu.fr[0]; // fadd fr0,fr8
    cpu.fr[8] = cpu.fr[8] + cpu.fr[6]; // fadd fr6,fr8
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
    cpu.fr[8] = cpu.fr[8] + cpu.fr[3]; // fadd fr3,fr8
    cpu.fr[0] = cpu.fr[0] - cpu.fr[8]; // fsub fr8,fr0
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // rts delay slot: mov.l r0,@r4
}
