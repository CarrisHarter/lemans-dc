//! 0x8c01097c — call the atan kernel (0x8c014480) on fr4, then classify fr0:
//! NaN (exponent all-set, mantissa nonzero) writes 0x44d to 0x8c23d8b8,
//! Inf (mantissa zero) writes 0x44c, finite writes nothing. Returns fr0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = mem.read_u32(0x8c0109bc).unwrap_or(0); // 0x8c014480
    cpu.pr = 0x8c010986;
    crate::registry::p_8c014480::run(cpu, mem);
    cpu.r[3] = cpu.r[15];
    cpu.r[4] = mem.read_u32(0x8c0109c0).unwrap_or(0); // 0x7f800000
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]);
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] &= cpu.r[4];
    cpu.set_t(cpu.r[2] == cpu.r[4]); // cmp/eq r4,r2
    if cpu.t() {
        cpu.r[2] = cpu.r[15];
        cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c0109c4).unwrap_or(0); // 0x7fffff
        cpu.set_t(cpu.r[3] & cpu.r[1] == 0); // tst r3,r1
        if !cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c0109c8).unwrap_or(0); // 0x8c23d8b8
            cpu.r[0] = mem.read_u16(0x8c0109b6).unwrap_or(0) as i16 as i32 as u32; // 0x44d
            let _ = mem.write_u32(cpu.r[2], cpu.r[0]); // bra delay slot
        } else {
            cpu.r[1] = mem.read_u32(0x8c0109c8).unwrap_or(0);
            cpu.r[2] = mem.read_u16(0x8c0109b8).unwrap_or(0) as i16 as i32 as u32; // 0x44c
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
        }
    }
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
