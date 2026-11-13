//! 0x8c170dae — scale a count into two output words.
//!
//! `r4` below `0x9ca` is raised to that limit. A negative value is converted
//! with the `2^32` bias at `0x8c170e8c`. The quotient of two `0x8c010d8c`
//! calls is truncated toward zero, and a negative non-integral result is
//! decremented so it matches floor. `0x8c010fa0(2, that)` scales it back,
//! the result is nudged by the pool constants, and the truncated value is
//! stored through `r6`. The floored value is stored through `r5`. `r0` is 1.
//! `r14` and `fr12`–`fr15` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);

    cpu.r[5] = mem.read_u16(0x8c170e7e).unwrap_or(0) as i16 as i32 as u32; // 0x9ca
    cpu.set_t(cpu.r[4] >= cpu.r[5]); // cmp/hs
    let keep = cpu.t();
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[6]); // bt.s delay
    if !keep {
        cpu.r[4] = cpu.r[5];
    }

    cpu.fpul = cpu.r[4];
    cpu.r[0] = 0x8c170e88; // mova
    cpu.fr[13] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz
    let nonneg = cpu.t();
    cpu.fr[3] = cpu.fpul as i32 as f32; // bt.s delay: float fpul, fr3
    if !nonneg {
        cpu.r[3] = rd(mem, 0x8c170e8c); // 0x4f800000
        cpu.fpul = cpu.r[3];
        cpu.fr[2] = f32::from_bits(cpu.fpul);
        cpu.fr[3] += cpu.fr[2];
    }

    cpu.fr[15] = cpu.fr[3];
    cpu.fr[15] /= cpu.fr[13];
    cpu.fr[12] = 1.0;
    cpu.fr[12] += cpu.fr[12];
    cpu.r[3] = rd(mem, 0x8c170e90); // 0x8c010d8c
    cpu.fr[14] = cpu.fr[3];
    cpu.pr = 0x8c170dec;
    cpu.fr[4] = cpu.fr[15]; // jsr delay
    crate::registry::p_8c010d8c::run(cpu, mem);

    cpu.r[2] = rd(mem, 0x8c170e90);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]);
    cpu.pr = 0x8c170df4;
    cpu.fr[4] = cpu.fr[12]; // jsr delay
    crate::registry::p_8c010d8c::run(cpu, mem);
    cpu.fr[4] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    cpu.fr[3] = 0.0;
    cpu.fr[4] /= cpu.fr[0];
    cpu.set_t(cpu.fr[3] > cpu.fr[4]); // fcmp/gt fr4, fr3
    cpu.fpul = cpu.fr[4] as i32 as u32; // ftrc
    let negative = cpu.t();
    cpu.r[4] = cpu.fpul; // bf.s delay
    if negative {
        cpu.r[14] = cpu.r[4];
        cpu.fpul = cpu.r[14];
        cpu.fr[2] = cpu.fpul as i32 as f32;
        cpu.set_t(cpu.fr[2] > cpu.fr[4]); // fcmp/gt fr4, fr2
        if cpu.t() {
            cpu.r[14] = cpu.r[14].wrapping_sub(1); // bra delay
        }
    } else {
        cpu.r[14] = cpu.r[4];
    }

    cpu.fpul = cpu.r[14];
    cpu.r[3] = rd(mem, 0x8c170e94); // 0x8c010fa0
    cpu.fr[3] = cpu.fpul as i32 as f32;
    cpu.fr[5] = cpu.fr[3];
    cpu.pr = 0x8c170e1e;
    cpu.fr[4] = cpu.fr[12]; // jsr delay
    crate::registry::p_8c010fa0::run(cpu, mem);

    cpu.fr[4] = cpu.fr[0];
    cpu.fr[4] *= cpu.fr[13];
    cpu.r[0] = 0x8c170e98; // mova
    cpu.fr[1] = 0.0;
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[4];
    cpu.fr[4] = cpu.fr[14];
    cpu.fr[4] /= cpu.fr[3];
    cpu.fr[3] = 1.0;
    cpu.fr[3] = -cpu.fr[3];
    cpu.fr[4] += cpu.fr[3];
    cpu.fr[4] *= cpu.fr[2];
    cpu.set_t(cpu.fr[4] > cpu.fr[1]); // fcmp/gt fr1, fr4
    if cpu.t() {
        cpu.r[0] = 0x8c170e9c;
        cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[3] > cpu.fr[4]); // fcmp/gt fr4, fr3
        if cpu.t() {
            cpu.fr[3] = 1.0;
            cpu.fr[4] += cpu.fr[3];
        }
    }
    cpu.r[0] = 0x8c170ea0;
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[4] > cpu.fr[3]); // fcmp/gt fr3, fr4
    let over = cpu.t();
    cpu.fr[5] = cpu.fr[4]; // bf.s delay
    if over {
        cpu.r[0] = 0x8c170ea4;
        cpu.fr[1] = cpu.fr[5];
        cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[1] += cpu.fr[2];
        cpu.fr[2] = cpu.fr[1]; // bra delay
    } else {
        cpu.fr[2] = cpu.fr[5];
    }
    cpu.fpul = cpu.fr[2] as i32 as u32;
    cpu.r[2] = rd(mem, cpu.r[15].wrapping_add(4));
    cpu.r[0] = 1;
    cpu.r[4] = cpu.fpul;
    let _ = mem.write_u32(cpu.r[2], cpu.r[4]);
    cpu.r[3] = rd(mem, cpu.r[15]);
    let _ = mem.write_u32(cpu.r[3], cpu.r[14]);

    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[12] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
