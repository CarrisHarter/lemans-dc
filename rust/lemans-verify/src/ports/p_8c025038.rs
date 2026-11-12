//! 0x8c025038 — fill a half-angle quad component.
//!
//! `r4` selects which component receives `sin(x/2)` (0 -> out[1], 1 -> out[2],
//! 2 -> out[3], anything else -> none); `fr4` is the angle `x` and the first
//! stack argument is the destination (four floats).
//!
//! The output starts as the global quad at 0x8c1d3ec8 (`out[i] = quad[i]`),
//! then `out[0]` is replaced by `0x8c0109e4(x/2)` and one further component by
//! `0x8c0111d0(x/2)`. So `out[0]` is the cos-like partner of the sine.
//!
//! Both callees can record a NaN/Inf flag in the global at 0x8c23d8b8; that
//! side effect belongs to them (and is reached here for non-finite `x/2`).
//! r13, r14, fr12-fr15 and pr are preserved.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Callee-saved pushes: fr15, fr14, fr13, fr12, r14, r13, pr.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);

    cpu.r[14] = cpu.r[4]; // selector
    cpu.r[13] = mem.read_u32(cpu.r[15].wrapping_add(36)).unwrap_or(0); // out pointer

    cpu.r[4] = mem.read_u32(0x8c0250d0).unwrap_or(0); // 0x8c1d3ec8
    cpu.fr[0] = mem.read_f32(0x8c0250d4).unwrap_or(0.0); // 0x3f000000 (0.5)
    cpu.fr[15] = cpu.fr[4];
    cpu.r[3] = cpu.r[4].wrapping_add(4);
    cpu.fr[15] *= cpu.fr[0];
    cpu.r[2] = cpu.r[15];
    cpu.r[1] = cpu.r[4];
    cpu.fr[14] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // quad[0]
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.r[0] = mem.read_u32(0x8c0250d8).unwrap_or(0); // 0x8c0111d0
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // quad[1]
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.fr[4] = cpu.fr[15]; // x/2
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // [r15+4] = quad[1]
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // quad[2]
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]); // [r15] = quad[2]
    cpu.fr[13] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // jsr delay: quad[3]
    cpu.pr = 0x8c025076;
    crate::registry::p_8c0111d0::run(cpu, mem); // sin

    cpu.r[0] = mem.read_u32(0x8c0250dc).unwrap_or(0); // 0x8c0109e4
    cpu.fr[4] = cpu.fr[15]; // x/2
    cpu.fr[12] = cpu.fr[0]; // jsr delay: sin result
    cpu.pr = 0x8c02507e;
    crate::registry::p_8c0109e4::run(cpu, mem); // cos-like partner

    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.t() {
        cpu.r[0] = 4;
        let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[12]);
        cpu.fr[14] = cpu.fr[0];
    } else {
        cpu.r[0] = cpu.r[14];
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.t() {
            let _ = mem.write_f32(cpu.r[15], cpu.fr[12]);
            cpu.fr[14] = cpu.fr[0];
        } else {
            cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
            if cpu.t() {
                cpu.fr[13] = cpu.fr[12];
                cpu.fr[14] = cpu.fr[0];
            }
        }
    }

    cpu.r[2] = cpu.r[15];
    let _ = mem.write_f32(cpu.r[13], cpu.fr[14]); // out[0]
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.r[1] = cpu.r[13];
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // [r15+4]
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // out[1]
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // [r15]
    cpu.r[13] = cpu.r[13].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // out[2]
    let _ = mem.write_f32(cpu.r[13], cpu.fr[13]); // out[3]
    cpu.r[15] = cpu.r[15].wrapping_add(8);

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[12] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
