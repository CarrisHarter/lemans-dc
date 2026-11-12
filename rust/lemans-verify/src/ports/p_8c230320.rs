//! 0x8c230320 - copy three float triples and pack four scaled channels.
//!
//! Same frame as 0x8c230282. Each source also contributes the floats at
//! +16/+20/+24/+28, multiplied by 255 and packed into the word at dest+20.
//! A product at or above 2^31 is reduced by 2^32 before `ftrc`.

use sh4_oracle::{Cpu, Memory};

fn ftrc(v: f32) -> u32 {
    v as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(116);
    cpu.r[14] = cpu.r[15].wrapping_add(4);
    cpu.r[3] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[15], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[3], cpu.r[5]);
    cpu.r[0] = 0x8c230424; // mova
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[6]);
    cpu.r[5] = 0;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[7]);
    cpu.r[14] = 84;
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(16), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(20), cpu.r[2]);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(24), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[4] = mem.read_u16(0x8c230420).unwrap_or(0xff) as i16 as i32 as u32;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(28), cpu.r[2]);
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    loop {
        cpu.r[2] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        cpu.r[0] = 4;
        cpu.r[3] = cpu.r[15].wrapping_add(32).wrapping_add(cpu.r[5]);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[2])).unwrap_or(0.0);
        cpu.r[0] = 8;
        cpu.r[1] = mem.read_u32(0x8c230428).unwrap_or(0);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[3]);
        cpu.r[2] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        cpu.r[3] = cpu.r[15].wrapping_add(32).wrapping_add(cpu.r[5]);
        cpu.fpul = cpu.r[1];
        cpu.r[0] = 8;
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[2])).unwrap_or(0.0);
        cpu.r[0] = 4;
        let _ = mem.write_f32(cpu.r[3].wrapping_add(4), cpu.fr[3]);
        cpu.r[0] = 12;
        cpu.r[2] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        cpu.r[3] = cpu.r[15].wrapping_add(32).wrapping_add(cpu.r[5]);
        cpu.fr[2] = f32::from_bits(cpu.fpul);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[2])).unwrap_or(0.0);
        cpu.r[0] = 8;
        let _ = mem.write_f32(cpu.r[3].wrapping_add(8), cpu.fr[3]);
        cpu.r[0] = 16;
        cpu.r[7] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        cpu.r[6] = cpu.r[6].wrapping_add(4);
        cpu.r[3] = cpu.r[15].wrapping_add(32);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0.0);
        cpu.fr[3] *= cpu.fr[4];
        cpu.set_t(cpu.fr[2] > cpu.fr[3]); // fcmp/gt fr3,fr2
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]); // bt.s delay
        if !cpu.t() {
            cpu.fr[2] += cpu.fr[2];
            cpu.fr[3] -= cpu.fr[2];
        }
        cpu.r[0] = 20;
        cpu.fpul = ftrc(cpu.fr[3]);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0.0);
        cpu.r[0] = 0x8c230428; // mova
        cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[3] *= cpu.fr[4];
        cpu.r[2] = cpu.fpul;
        cpu.set_t(cpu.fr[2] > cpu.fr[3]);
        cpu.r[2] &= cpu.r[4];
        cpu.r[2] = cpu.r[2].wrapping_shl(16);
        cpu.r[2] = cpu.r[2].wrapping_shl(8); // bt.s delay
        if !cpu.t() {
            cpu.fr[2] += cpu.fr[2];
            cpu.fr[3] -= cpu.fr[2];
        }
        cpu.r[0] = 24;
        cpu.fpul = ftrc(cpu.fr[3]);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0.0);
        cpu.r[0] = 0x8c230428;
        cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[3] *= cpu.fr[4];
        cpu.r[1] = cpu.fpul;
        cpu.set_t(cpu.fr[2] > cpu.fr[3]);
        cpu.r[1] &= cpu.r[4];
        cpu.r[1] = cpu.r[1].wrapping_shl(16);
        cpu.r[2] |= cpu.r[1]; // bt.s delay
        if !cpu.t() {
            cpu.fr[2] += cpu.fr[2];
            cpu.fr[3] -= cpu.fr[2];
        }
        cpu.r[0] = 28;
        cpu.fpul = ftrc(cpu.fr[3]);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0.0);
        cpu.r[0] = 0x8c230428;
        cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[3] *= cpu.fr[4];
        cpu.r[1] = cpu.fpul;
        cpu.set_t(cpu.fr[2] > cpu.fr[3]);
        cpu.r[1] &= cpu.r[4];
        cpu.r[1] = cpu.r[1].wrapping_shl(8);
        cpu.r[2] |= cpu.r[1]; // bt.s delay
        if !cpu.t() {
            cpu.fr[2] += cpu.fr[2];
            cpu.fr[3] -= cpu.fr[2];
        }
        cpu.fpul = ftrc(cpu.fr[3]);
        cpu.r[5] = cpu.r[5].wrapping_add(28);
        cpu.set_t(cpu.r[5] >= cpu.r[14]); // cmp/hs r14,r5
        cpu.r[1] = cpu.fpul;
        cpu.r[1] &= cpu.r[4];
        cpu.r[2] |= cpu.r[1];
        let _ = mem.write_u32(cpu.r[3].wrapping_add(20), cpu.r[2]);
        if cpu.t() {
            break;
        }
    }
    cpu.r[2] = mem.read_u32(0x8c23042c).unwrap_or(0x8c2395c0);
    cpu.r[4] = cpu.r[15].wrapping_add(16); // jsr delay
    cpu.pr = 0x8c230406;
    crate::registry::p_8c2395c0::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(116);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
