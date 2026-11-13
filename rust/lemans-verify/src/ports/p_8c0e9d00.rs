//! 0x8c0e9d00 — reduce record bounds, scale them, and shift a copy.
//!
//! `r4` is the destination, `r5` the source, `r6` the count. The count and
//! source are passed to `0x8c0e9f6c`, which writes four floats into a stack
//! slot. Those are scaled by the literal at `0x8c0e9de8`, truncated, negated
//! and scaled again by `0x8c0e9df0`. A zero byte at `0x8c27ad58` clears the
//! first offset; a zero byte at `0x8c27ad60` clears the second. A positive
//! count then copies each 24-byte record and adds the two offsets at `+16`
//! and `+20`. `r9`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn f32_at(mem: &Memory, addr: u32) -> f32 {
    mem.read_f32(addr).unwrap_or(0.0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = rd(mem, 0x8c0e9de4); // 0x8c0e9f6c
    cpu.r[5] = cpu.r[14];
    cpu.pr = 0x8c0e9d22;
    cpu.r[4] = cpu.r[13]; // jsr delay
    crate::registry::p_8c0e9f6c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    cpu.r[2] = 0;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[3] = f32_at(mem, cpu.r[2]);
    cpu.fr[2] = f32_at(mem, 0x8c0e9de8);
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = f32_at(mem, cpu.r[2]);
    cpu.r[0] = rd(mem, 0x8c0e9dec); // 0x8c27ad58
    cpu.r[2] = 4;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[1] = f32_at(mem, cpu.r[2]);
    cpu.fr[3] += cpu.fr[0];
    cpu.r[1] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[2] = 12;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = f32_at(mem, cpu.r[2]);
    cpu.fr[3] *= cpu.fr[2];
    cpu.r[1] &= 0xff;
    cpu.fr[1] += cpu.fr[0];
    cpu.fr[1] *= cpu.fr[2];
    cpu.fr[3] *= cpu.fr[2];
    cpu.fr[1] *= cpu.fr[2];
    cpu.r[1] &= 0xff;
    cpu.fpul = cpu.fr[3] as i32 as u32; // ftrc fr3,fpul
    cpu.r[4] = cpu.fpul;
    cpu.fpul = cpu.fr[1] as i32 as u32; // ftrc fr1,fpul
    cpu.r[3] = cpu.fpul;
    cpu.fpul = cpu.r[4];
    cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
    cpu.fpul = cpu.r[3];
    cpu.fr[0] = f32_at(mem, 0x8c0e9df0);
    cpu.fr[1] = -cpu.fr[1];
    cpu.set_t(cpu.r[1] == 0);
    cpu.fr[2] = cpu.fpul as i32 as f32; // float fpul,fr2
    cpu.fr[2] = -cpu.fr[2];
    cpu.fr[1] *= cpu.fr[0];
    let clear_first = cpu.t();
    cpu.fr[2] *= cpu.fr[0]; // bf.s delay, always
    if clear_first {
        cpu.fr[1] = 0.0; // fldi0
    }
    cpu.r[0] = rd(mem, 0x8c0e9df4); // 0x8c27ad60
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] &= 0xff;
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.fr[2] = 0.0;
    }
    cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl
    let skip_loop = !cpu.t();
    cpu.r[3] = 0; // bf.s delay, always
    if !skip_loop {
        cpu.r[4] = cpu.r[3];
        cpu.r[11] = cpu.r[12].wrapping_add(16);
        cpu.r[6] = cpu.r[12].wrapping_add(20);
        loop {
            cpu.r[0] = cpu.r[13].wrapping_add(cpu.r[4]);
            cpu.r[5] = rd(mem, cpu.r[0]);
            cpu.r[1] = cpu.r[12].wrapping_add(cpu.r[4]);
            cpu.r[9] = rd(mem, cpu.r[0].wrapping_add(4));
            let _ = mem.write_u32(cpu.r[1], cpu.r[5]);
            cpu.r[3] = cpu.r[3].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[1].wrapping_add(4), cpu.r[9]);
            cpu.r[5] = cpu.r[6].wrapping_add(cpu.r[4]);
            cpu.r[9] = rd(mem, cpu.r[0].wrapping_add(8));
            cpu.r[7] = cpu.r[11].wrapping_add(cpu.r[4]);
            cpu.r[10] = rd(mem, cpu.r[0].wrapping_add(12));
            let cont = (cpu.r[14] as i32) > (cpu.r[3] as i32); // cmp/gt r3, r14
            cpu.set_t(cont);
            let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[9]);
            cpu.r[4] = cpu.r[4].wrapping_add(24);
            let _ = mem.write_u32(cpu.r[1].wrapping_add(12), cpu.r[10]);
            cpu.r[10] = rd(mem, cpu.r[0].wrapping_add(16));
            cpu.r[0] = rd(mem, cpu.r[0].wrapping_add(20));
            let _ = mem.write_u32(cpu.r[1].wrapping_add(16), cpu.r[10]);
            let _ = mem.write_u32(cpu.r[1].wrapping_add(20), cpu.r[0]);
            cpu.fr[0] = f32_at(mem, cpu.r[7]);
            cpu.fr[0] += cpu.fr[1];
            let _ = mem.write_f32(cpu.r[7], cpu.fr[0]);
            cpu.fr[0] = f32_at(mem, cpu.r[5]);
            cpu.fr[0] += cpu.fr[2];
            if !cont {
                let _ = mem.write_f32(cpu.r[5], cpu.fr[0]); // bt.s delay still runs
                break;
            }
            let _ = mem.write_f32(cpu.r[5], cpu.fr[0]);
        }
    }

    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
