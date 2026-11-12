//! 0x8c0148ec — double-precision log2-ish kernel: normalizes (fr5,fr4) via
//! 0x8c014936, scales the exponent word by 1.6505 (float(fpu word)), rounds
//! via 0x8c014ab4, then truncates the fraction word to an int return in r0
//! (rts delay slot `sts fpul,r0`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.r[4] = cpu.r[15];
    cpu.r[4] = cpu.r[4].wrapping_add(8); // bsr delay slot
    cpu.pr = 0x8c0148f6;
    crate::registry::p_8c014936::run(cpu, mem);
    cpu.r[3] = cpu.fpscr; // sts fpscr,r3
    cpu.r[1] = mem.read_u32(0x8c014a24).unwrap_or(0); // 0x80000
    cpu.r[0] = 0x8c014a28; // mova
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.r[3] |= cpu.r[1];
    cpu.fpscr = cpu.r[3]; // lds r3,fpscr
    cpu.fpul = cpu.r[2]; // lds r2,fpul
    cpu.r[3] = mem.read_u32(0x8c014a30).unwrap_or(0); // 0xffe7ffff
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = 0x8c014a2c; // mova
    cpu.fr[2] = cpu.fpul as i32 as f32; // float fpul,fr2
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[2] = cpu.fpscr; // sts fpscr,r2
    cpu.r[2] &= cpu.r[3];
    cpu.fr[5] = cpu.fr[3];
    cpu.fr[4] = cpu.fr[2];
    cpu.fr[4] = cpu.fr[4] * cpu.fr[0]; // fmul fr0,fr4
    cpu.fpscr = cpu.r[2]; // lds r2,fpscr
    cpu.r[4] = cpu.r[15]; // bsr delay slot
    cpu.pr = 0x8c01491e;
    crate::registry::p_8c014ab4::run(cpu, mem);
    cpu.r[0] = 4;
    cpu.r[3] = mem.read_u32(0x8c014a24).unwrap_or(0); // 0x80000
    cpu.fr[2] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[0] = cpu.fpscr; // sts fpscr,r0
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[0] |= cpu.r[3];
    cpu.fpscr = cpu.r[0]; // lds r0,fpscr
    cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc fr2,fpul
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.fpul; // rts delay slot: sts fpul,r0
}
