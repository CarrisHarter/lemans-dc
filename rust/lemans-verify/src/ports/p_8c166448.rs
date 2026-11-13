//! 0x8c166448 - scale a pair of floats and write them plus 1.0.
//!
//! A stack pointer (entry sp+32) and r7 supply the inputs. The byte at
//! 0x8c20b1f0 picks the path: nonzero divides 1 by the accumulated fr4 and
//! adds 0.5; zero calls atan2 (0x8c01092c) and scales fr0 by ~0.159. Both
//! paths store the two results at r5 and 1.0 at r6. The bt.s delay add
//! into fr15 runs on both paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);

    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(56)).unwrap_or(0);
    cpu.r[13] = cpu.r[5];
    cpu.r[1] = cpu.r[7].wrapping_add(8);
    cpu.fr[3] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[0];
    cpu.fr[15] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(124);
    cpu.fr[5] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[3] = cpu.r[0];
    cpu.r[1] = mem.read_u32(0x8c1664f8).unwrap_or(0x8c20b1f0);
    cpu.fr[2] = cpu.fr[3];
    cpu.fr[0] = cpu.fr[5];
    cpu.r[4] = cpu.r[0];
    cpu.r[5] = cpu.r[0];
    cpu.r[6] = mem.read_u32(0x8c1664fc).unwrap_or(0x80);
    cpu.r[3] = cpu.r[3].wrapping_add(112);
    // mov.b sign-extends; the two extu.b that follow keep the low byte.
    cpu.r[1] = mem.read_u8(cpu.r[1]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
    cpu.fr[6] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.fr[4] = cpu.fr[15];
    cpu.fr[0] = -cpu.fr[0];
    cpu.r[1] &= 0xff;
    cpu.r[1] &= 0xff;
    cpu.fr[15] *= cpu.fr[6];
    cpu.fr[3] = cpu.fr[5] * cpu.fr[3];
    cpu.r[7] = cpu.r[7].wrapping_add(4);
    cpu.r[4] = cpu.r[4].wrapping_add(116);
    cpu.fr[15] += cpu.fr[3];
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[0].wrapping_add(120);
    cpu.fr[4] *= cpu.fr[0];
    cpu.fr[2] *= cpu.fr[6];
    cpu.fr[5] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    cpu.fr[15] *= cpu.fr[1];
    cpu.fr[3] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.set_t(cpu.r[1] == 0);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[5] *= cpu.fr[3];
    cpu.fr[4] += cpu.fr[2];
    cpu.fr[15] += cpu.fr[0]; // bt.s delay, both paths
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c16650c).unwrap_or(0x8c01092c);
        cpu.pr = 0x8c1664c8;
        crate::registry::p_8c01092c::run(cpu, mem);
        cpu.fr[2] = mem.read_f32(0x8c166510).unwrap_or(0.0);
        cpu.fr[1] = mem.read_f32(0x8c166508).unwrap_or(0.5);
        cpu.fr[0] *= cpu.fr[2];
        cpu.fr[0] += cpu.fr[1];
    } else {
        cpu.r[0] = mem.read_u32(0x8c166500).unwrap_or(0x8c20b200);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[0] = mem.read_u32(0x8c166504).unwrap_or(0x8c20b1f8);
        cpu.fr[4] += cpu.fr[0];
        cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[0] = 1.0;
        cpu.fr[0] /= cpu.fr[4];
        cpu.fr[1] = mem.read_f32(0x8c166508).unwrap_or(0.5);
        cpu.fr[2] = cpu.fr[5] * cpu.fr[2];
        cpu.fr[0] *= cpu.fr[2];
        cpu.fr[0] += cpu.fr[1];
    }
    cpu.r[0] = 0;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[15]);
    cpu.r[2] = cpu.r[15].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[0] = cpu.r[15];
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.fr[0] = 1.0;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[0]);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);

    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
