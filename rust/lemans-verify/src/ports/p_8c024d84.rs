//! 0x8c024d84 — vector blend pipeline.
//!
//! Saves fr13-fr15/r14/r13, takes r6 as the output pointer and r5 (r13) plus
//! the entry fr4/fr5/fr6 as inputs. Calls 0x8c0b4f5c to diff `[r4]-[r4+24]`
//! into the frame, shuffles the three words, then chains 0x8c0b4e30
//! (scale by fr13*fr14), 0x8c0b4f34, 0x8c0b4ef0 (`1 - fr13*fr15`) with
//! intermediate fr14/fr13/fr15 reloads, and finally 0x8c0b4e30 and 0x8c0b4f34
//! again on the output pointer before restoring everything.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.fr[14] = cpu.fr[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[13] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(48);
    cpu.r[13] = cpu.r[5];
    cpu.fr[15] = cpu.fr[6];
    cpu.r[0] = cpu.r[15];
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c024e20).unwrap_or(0);
    crate::registry::p_8c0b4f5c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[2] = 28;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[0] = 24;
    cpu.fr[0] = mem.read_f32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap_or(0.0);
    cpu.r[2] = 36;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[0] = cpu.r[15];
    cpu.r[2] = 32;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    cpu.r[2] = 40;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);
    cpu.r[2] = 44;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.fr[4] = cpu.fr[13];
    cpu.r[4] = cpu.r[15];
    cpu.r[0] = mem.read_u32(0x8c024e24).unwrap_or(0);
    cpu.fr[4] = cpu.fr[4] * cpu.fr[14];
    cpu.r[4] = cpu.r[4].wrapping_add(40); // delay slot of `jsr`
    crate::registry::p_8c0b4e30::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[5] = cpu.r[15];
    cpu.r[0] = mem.read_u32(0x8c024e28).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.r[5] = cpu.r[5].wrapping_add(12); // delay slot of `jsr`
    crate::registry::p_8c0b4f34::run(cpu, mem);
    cpu.fr[0] = cpu.fr[13];
    cpu.r[0] = mem.read_u32(0x8c024e2c).unwrap_or(0);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[15];
    cpu.fr[4] = 1.0;
    cpu.r[4] = cpu.r[14];
    cpu.fr[4] = cpu.fr[4] - cpu.fr[0]; // delay slot of `jsr`
    crate::registry::p_8c0b4ef0::run(cpu, mem);
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c024e24).unwrap_or(0);
    cpu.fr[4] = cpu.fr[13];
    cpu.r[4] = cpu.r[14]; // delay slot of `jsr`
    crate::registry::p_8c0b4e30::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = mem.read_u32(0x8c024e28).unwrap_or(0);
    cpu.r[4] = cpu.r[13];
    cpu.r[5] = cpu.r[15]; // delay slot of `jsr`
    crate::registry::p_8c0b4f34::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
