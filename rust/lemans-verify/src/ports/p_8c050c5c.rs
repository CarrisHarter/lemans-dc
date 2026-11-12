//! 0x8c050c5c — length of a 2-vector difference (hypot of table point minus r6 point).
//!
//! `r4` = object, `r5` = index (scaled by 4), `r6` = `float[2]` point.
//! Loads a `float[2]` from the row table: `k = [r4+4]<<2`,
//! `base = [r5*4 + [[r4+32]<<3 + [0x8c1f6be8+k] + 4]] * 0x50 + [0x8c1f6bd8+k]`,
//! `p = base+4`. If `p[0]==r6[0] && p[1]==r6[1]`, returns `fr0 = 0.0` directly
//! (T left clear by the second `fcmp/eq`). Otherwise pushes
//! `(p[0]-r6[0], p[1]-r6[1])` on the stack and tail-calls the hypot kernel
//! `0x8c0b2c00` with `r4` = stack pointer; its delay slot sets `r4` before the
//! call. Restores `r15`/`pr` and returns with the length in `fr0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[5] <<= 2;
    cpu.r[0] = mem.read_u32(0x8c050cc4).unwrap_or(0); // 0x8c1f6be8
    cpu.r[7] <<= 2;
    cpu.fr[3] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.r[0] <<= 2;
    let shll_out = cpu.r[0] >> 31 != 0;
    cpu.r[0] <<= 1;
    cpu.set_t(shll_out);
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.r[0] = cpu.r[5];
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c050cc8).unwrap_or(0); // 0x8c1f6bd8
    cpu.r[1] = cpu.r[3];
    cpu.r[1] <<= 2;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    cpu.r[1] <<= 2;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[7])).unwrap_or(0);
    cpu.r[1] <<= 2;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.set_t(cpu.fr[1] == cpu.fr[3]); // fcmp/eq fr3,fr1
    // bf.s to 0x8c050ca4; delay slot always runs:
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    if !cpu.t() {
        cpu.r[6] = cpu.r[6].wrapping_add(4);
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        cpu.fr[1] = cpu.fr[1] - cpu.fr[3];
        cpu.r[0] = 0;
        let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[1]);
        cpu.fr[2] = cpu.fr[2] - cpu.fr[0];
        cpu.r[0] = 4;
        let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[2]);
        cpu.r[0] = mem.read_u32(0x8c050ccc).unwrap_or(0); // 0x8c0b2c00
        cpu.pr = 0x8c050cba; // jsr return address
        cpu.r[4] = cpu.r[15]; // delay slot of `jsr`
        crate::registry::p_8c0b2c00::run(cpu, mem);
    } else {
        cpu.r[0] = cpu.r[6];
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[0] == cpu.fr[2]); // fcmp/eq fr0,fr2
        if !cpu.t() {
            cpu.r[6] = cpu.r[6].wrapping_add(4);
            cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
            cpu.fr[1] = cpu.fr[1] - cpu.fr[3];
            cpu.r[0] = 0;
            let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[1]);
            cpu.fr[2] = cpu.fr[2] - cpu.fr[0];
            cpu.r[0] = 4;
            let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[2]);
            cpu.r[0] = mem.read_u32(0x8c050ccc).unwrap_or(0); // 0x8c0b2c00
            cpu.pr = 0x8c050cba; // jsr return address
            cpu.r[4] = cpu.r[15]; // delay slot of `jsr`
            crate::registry::p_8c0b2c00::run(cpu, mem);
        } else {
            cpu.fr[0] = 0.0; // fldi0, delay slot of `bra`
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
