//! 0x8c16ec64 — scale two floats into 0..15 slots and store them when they differ.
//!
//! Builds a temporary at `r15` through `0x8c0ed79c`, then scales the incoming
//! `fr4` and `fr5` with `0x8c0ed6b8` (`r5 = 4`, `r6 = 0`) and truncates each
//! result. When bit 0 of `[object+8]` is clear, the fields are left alone.
//! Otherwise each truncated value is clamped with an unsigned compare: the
//! lower test against 0 never succeeds, and a value above 15 becomes 15. If
//! `[object+92]` does not already hold that pair and the incoming `r5` at
//! `+116`, the three words are stored and bit 31 of `[object+8]` is set.
//! `0x8c0ed764(temp, -1)` then runs and does not free the temporary.
//! `r12`–`r14` and `fr14`/`fr15` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn ftrc(value: f32) -> u32 {
    value as i32 as u32
}

/// Unsigned clamp used by both truncated results. `0 > src` is never true.
fn clamp_u15(cpu: &mut Cpu, src: u32) -> u32 {
    let mut out = 0u32;
    cpu.set_t(out > src); // cmp/hi src, #0
    if !cpu.t() {
        out = 15;
        cpu.set_t(src > out); // cmp/hi #15, src
        if !cpu.t() {
            out = src;
        }
    }
    out
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.fr[15] = cpu.fr[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[14] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);

    cpu.r[0] = rd(mem, 0x8c16ed2c); // 0x8c0ed79c
    cpu.pr = 0x8c16ec80;
    cpu.r[4] = cpu.r[15]; // jsr delay
    crate::registry::p_8c0ed79c::run(cpu, mem);

    cpu.r[0] = rd(mem, 0x8c16ed30); // 0x8c0ed6b8
    cpu.fr[4] = cpu.fr[14];
    cpu.r[4] = cpu.r[15];
    cpu.r[6] = 0;
    cpu.pr = 0x8c16ec8c;
    cpu.r[5] = 4; // jsr delay
    crate::registry::p_8c0ed6b8::run(cpu, mem);

    cpu.fr[4] = cpu.fr[15];
    cpu.fpul = ftrc(cpu.fr[0]);
    cpu.r[0] = rd(mem, 0x8c16ed30); // 0x8c0ed6b8
    cpu.r[4] = cpu.r[15];
    cpu.r[6] = 0;
    cpu.r[5] = 4;
    cpu.pr = 0x8c16ec9c;
    cpu.r[12] = cpu.fpul; // jsr delay: sts fpul, r12
    crate::registry::p_8c0ed6b8::run(cpu, mem);

    cpu.r[0] = rd(mem, cpu.r[13].wrapping_add(8));
    cpu.fpul = ftrc(cpu.fr[0]);
    cpu.r[0] &= 1;
    cpu.set_t(cpu.r[0] == 0);
    let skip = cpu.t();
    cpu.r[4] = cpu.fpul; // bt.s delay, always
    if !skip {
        cpu.r[1] = clamp_u15(cpu, cpu.r[12]);
        cpu.r[3] = clamp_u15(cpu, cpu.r[4]);
        cpu.r[0] = cpu.r[13].wrapping_add(92);
        cpu.r[4] = rd(mem, cpu.r[0]);
        cpu.r[0] = rd(mem, cpu.r[4].wrapping_add(120));
        cpu.set_t(cpu.r[0] == cpu.r[1]);
        let mut matched = cpu.t();
        if matched {
            cpu.r[0] = rd(mem, cpu.r[4].wrapping_add(124));
            cpu.set_t(cpu.r[0] == cpu.r[3]);
            matched = cpu.t();
        }
        if matched {
            cpu.r[0] = rd(mem, cpu.r[4].wrapping_add(116));
            cpu.set_t(cpu.r[0] == cpu.r[14]);
            matched = cpu.t();
        }
        if !matched {
            cpu.r[0] = 116;
            let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[14]);
            cpu.r[6] = cpu.r[13].wrapping_add(92);
            cpu.r[5] = rd(mem, cpu.r[6]);
            cpu.r[0] = 120;
            cpu.r[4] = rd(mem, 0x8c16ed34); // 0x80000000
            let _ = mem.write_u32(cpu.r[5].wrapping_add(cpu.r[0]), cpu.r[1]);
            cpu.r[1] = rd(mem, cpu.r[6]);
            cpu.r[0] = 124;
            let _ = mem.write_u32(cpu.r[1].wrapping_add(cpu.r[0]), cpu.r[3]);
            cpu.r[0] = rd(mem, cpu.r[13].wrapping_add(8));
            cpu.r[0] |= cpu.r[4];
            let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]);
        }
    }

    cpu.r[0] = rd(mem, 0x8c16ed38); // 0x8c0ed764
    cpu.r[4] = cpu.r[15];
    cpu.pr = 0x8c16ed1c;
    cpu.r[5] = (-1i32) as u32; // jsr delay
    crate::registry::p_8c0ed764::run(cpu, mem);

    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
