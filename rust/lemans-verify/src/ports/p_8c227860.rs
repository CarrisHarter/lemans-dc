//! 0x8c227860 — pack `fr4` into a bitfield at `*r4`.
//!
//! `0x8c010cf8` writes the biased exponent byte at the frame and returns the
//! sign-and-mantissa forced into `[0.5, 1)`. That magnitude is doubled. A
//! result `>= 1` subtracts 1 and sets bit 15. Seven fraction thresholds then
//! set bits 14..8, and the decremented exponent byte fills bits 7..0.
//! `r0` is 0. `r14` is restored.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn hw(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = word(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

/// One fraction stage: if `fr4 >= -neg`, set `bit` and subtract `pos`.
fn peel(cpu: &mut Cpu, mem: &mut Memory, neg: u32, bit: u32, pos: u32) {
    cpu.r[0] = neg; // mova
    cpu.fr[2] = cpu.fr[4];
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[1] = 0.0;
    cpu.fr[2] += cpu.fr[3];
    cpu.set_t(cpu.fr[1] > cpu.fr[2]); // fcmp/gt fr2, fr1
    if !cpu.t() {
        cpu.r[0] = pos;
        cpu.r[3] = hw(mem, bit);
        cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[14] |= cpu.r[3];
        cpu.fr[4] -= cpu.fr[2];
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let frame = cpu.r[15];
    cpu.r[2] = word(mem, 0x8c227970); // 0x8c010cf8
    cpu.r[14] = 0;
    let _ = mem.write_u32(frame.wrapping_add(4), cpu.r[4]);
    cpu.pr = 0x8c227870;
    cpu.r[4] = frame; // jsr delay
    crate::registry::p_8c010cf8::run(cpu, mem);

    cpu.fr[3] = 1.0;
    cpu.fr[3] += cpu.fr[3]; // 2.0
    cpu.fr[4] = cpu.fr[0];
    cpu.fr[2] = 1.0;
    cpu.fr[2] = -cpu.fr[2];
    cpu.fr[4] *= cpu.fr[3];
    cpu.fr[0] = 0.0;
    cpu.r[3] = mem.read_u8(frame).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[3] = cpu.r[3].wrapping_add(0xffff_ffff);
    cpu.fr[1] = cpu.fr[4];
    cpu.fr[1] += cpu.fr[2];
    cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1, fr0
    let _ = mem.write_u8(frame, cpu.r[3] as u8); // bt.s delay
    if !cpu.t() {
        cpu.fr[3] = 1.0;
        cpu.fr[4] -= cpu.fr[3];
        cpu.r[14] = word(mem, 0x8c227974); // 0x8000
    }

    peel(cpu, mem, 0x8c227978, 0x8c227960, 0x8c22797c);
    peel(cpu, mem, 0x8c227980, 0x8c227962, 0x8c227984);
    peel(cpu, mem, 0x8c227988, 0x8c227964, 0x8c22798c);
    peel(cpu, mem, 0x8c227990, 0x8c227966, 0x8c227994);
    peel(cpu, mem, 0x8c227998, 0x8c227968, 0x8c22799c);
    peel(cpu, mem, 0x8c2279a0, 0x8c22796a, 0x8c2279a4);

    cpu.r[0] = 0x8c2279a8; // mova
    cpu.fr[2] = 0.0;
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[4] += cpu.fr[3];
    cpu.set_t(cpu.fr[2] > cpu.fr[4]); // fcmp/gt fr4, fr2
    if !cpu.t() {
        cpu.r[2] = hw(mem, 0x8c22796c); // 0x100
        cpu.r[14] |= cpu.r[2];
    }

    cpu.r[3] = mem.read_u8(frame).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[0] = 0;
    cpu.r[3] &= 0xff; // extu.b
    cpu.r[14] |= cpu.r[3];
    cpu.r[3] = word(mem, frame.wrapping_add(4));
    let _ = mem.write_u32(cpu.r[3], cpu.r[14]);

    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
