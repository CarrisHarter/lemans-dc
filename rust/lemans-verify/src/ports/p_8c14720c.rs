//! 0x8c14720c - wrap a float difference into [0, 1] and scale a field.
//!
//! `fr15` becomes `[r5+0x94] - [r4+0x94]`, reduced into [0, 1] by adding or
//! subtracting 1. That offset is folded into a product of the fields at
//! +112, +76 and +68. If the scaled field is larger, it is stored and the
//! function returns 1. Otherwise the reduced difference is written at
//! `r4+0x94` and the function returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8c1472c0).unwrap_or(0x94);
    cpu.r[1] = cpu.r[14].wrapping_add(cpu.r[3]);
    cpu.r[0] = cpu.r[13].wrapping_add(cpu.r[3]);
    cpu.fr[15] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[1] = 0.0;
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[15] -= cpu.fr[0];
    cpu.set_t(cpu.fr[1] > cpu.fr[15]); // fcmp/gt fr15,fr1
    if !cpu.t() {
        // difference already >= 0
    } else {
        loop {
            cpu.fr[0] = 1.0;
            cpu.fr[15] += cpu.fr[0];
            cpu.set_t(cpu.fr[1] > cpu.fr[15]);
            if !cpu.t() {
                break;
            }
        }
    }
    cpu.fr[0] = 1.0;
    cpu.set_t(cpu.fr[15] > cpu.fr[0]); // fcmp/gt fr0,fr15
    if !cpu.t() {
    } else {
        loop {
            cpu.fr[15] -= cpu.fr[0];
            cpu.set_t(cpu.fr[15] > cpu.fr[0]);
            if !cpu.t() {
                break;
            }
        }
    }
    cpu.r[1] = cpu.r[13].wrapping_add(112);
    cpu.r[4] = cpu.r[13].wrapping_add(76);
    cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[3] = cpu.r[13].wrapping_add(68);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[0] = 0x8c1472c4; // mova
    cpu.fr[4] *= cpu.fr[0];
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[4] *= cpu.fr[1];
    cpu.r[1] = mem.read_u32(0x8c1472c8).unwrap_or(0x8c010b38);
    cpu.fr[4] += cpu.fr[0]; // jsr delay
    cpu.pr = 0x8c147262;
    crate::registry::p_8c010b38::run(cpu, mem);
    cpu.r[4] = cpu.r[13].wrapping_add(112);
    cpu.r[3] = cpu.r[13].wrapping_add(76);
    cpu.r[1] = cpu.r[13].wrapping_add(68);
    cpu.fr[3] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[0] = 0x8c1472cc; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.fr[0] += cpu.fr[15];
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[3] *= cpu.fr[1];
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] /= cpu.fr[3];
    cpu.fr[2] *= cpu.fr[1];
    cpu.set_t(cpu.fr[2] > cpu.fr[0]); // fcmp/gt fr0,fr2: T = fr2 > fr0
    if cpu.t() {
        let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
        cpu.r[0] = 1;
    } else {
        let _ = mem.write_f32(cpu.r[1], cpu.fr[2]);
        cpu.r[0] = mem.read_u32(0x8c1472c0).unwrap_or(0x94);
        cpu.fr[2] = 1.0;
        cpu.fr[3] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]);
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        cpu.fr[3] *= cpu.fr[0];
        cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
        cpu.fr[3] *= cpu.fr[1];
        cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]);
        cpu.fr[0] -= cpu.fr[3];
        let _ = mem.write_f32(cpu.r[13], cpu.fr[0]);
        cpu.fr[0] = 0.0; // bra delay
        loop {
            cpu.fr[1] = mem.read_f32(cpu.r[13]).unwrap_or(0.0);
            cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
            if !cpu.t() {
                break;
            }
            cpu.fr[1] += cpu.fr[2];
            let _ = mem.write_f32(cpu.r[13], cpu.fr[1]);
        }
        cpu.r[0] = 0;
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
