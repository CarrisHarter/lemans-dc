//! 0x8c014de0 — wrap a byte accessor call.
//!
//! Masks the byte at `r5` to 7 bits, calls 0x8c010e84(`r15+13`, 3) and stores
//! its two out-words, then folds them: `avg = (r4 + (0 - (r4<0))) >> 1`,
//! `parity = avg & 1` normalised to 0 or -1, shifted left 7 and ORed into the
//! byte the callee left at `*slot`, then calls 0x8c014da4(`r15`) and returns
//! its `fr0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u16(cpu.r[15] + 4, cpu.r[0] as u16);
    cpu.r[14] = cpu.r[15];
    cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
    cpu.r[14] = cpu.r[14].wrapping_add(12);
    cpu.r[3] = mem.read_u32(0x8c014e44).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.r[0] = cpu.r[0] & 127;
    cpu.r[4] = cpu.r[4].wrapping_add(1);
    let _ = mem.write_u8(cpu.r[5], cpu.r[0] as u8);
    cpu.r[6] = 3;
    let _ = mem.write_u32(cpu.r[15] + 8, cpu.r[4]); // delay slot of `jsr`
    crate::registry::p_8c010e84::run(cpu, mem);
    cpu.r[0] = mem.read_u16(cpu.r[15] + 4).unwrap_or(0) as i16 as u32;
    cpu.r[2] = 0;
    cpu.r[4] = cpu.r[0];
    cpu.r[3] = cpu.r[4];
    cpu.set_t((cpu.r[2] as i32) > (cpu.r[3] as i32));
    cpu.r[0] = cpu.r[4];
    let t = cpu.t();
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]).wrapping_add(t as u32); // addc
    let lsb = cpu.r[3] & 1;
    cpu.r[3] = ((cpu.r[3] as i32) >> 1) as u32; // shar: T = shifted-out LSB
    cpu.set_t(lsb != 0);
    let pz = (cpu.r[0] as i32) >= 0;
    cpu.set_t(pz);
    let _ = mem.write_u8(cpu.r[14], cpu.r[3] as u8);
    cpu.r[3] = mem.read_u32(cpu.r[15] + 8).unwrap_or(0);
    if !pz {
        cpu.r[0] = !cpu.r[0];
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.r[0] = cpu.r[0] & 1;
        cpu.r[0] = !cpu.r[0];
        cpu.r[0] = cpu.r[0].wrapping_add(1);
    } else {
        cpu.r[0] = cpu.r[0] & 1; // delay slot of `bra`
    }
    cpu.r[1] = mem.read_u8(cpu.r[3]).unwrap_or(0) as i8 as u32;
    cpu.r[2] = 7;
    cpu.r[0] = if (cpu.r[2] as i32) >= 0 {
        cpu.r[0] << cpu.r[2]
    } else {
        ((cpu.r[0] as i32) >> cpu.r[2].wrapping_neg()) as u32
    }; // shad
    cpu.r[5] = cpu.r[14];
    cpu.r[1] = cpu.r[1] | cpu.r[0];
    let _ = mem.write_u8(cpu.r[3], cpu.r[1] as u8);
    cpu.r[3] = mem.read_u32(0x8c014e48).unwrap_or(0);
    cpu.r[4] = cpu.r[15]; // delay slot of `jsr`
    crate::registry::p_8c014da4::run(cpu, mem);
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
