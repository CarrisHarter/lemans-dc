//! 0x8c13bf44 — lerp an object's two vectors into two global buffers.
//!
//! `fr15` is `min(1, [object+36])`. `fr14` is
//! `min(1, [object+36] - 2*[object+40])`. `0x8c0b4f5c` subtracts the vector
//! at `object+0` from the vector at `object+12`. Each component is scaled by
//! those factors and added back, then a `1.0` is stored as the fourth word.
//! The `fr15` result goes through `[*global+100]` and the `fr14` result
//! through `[*global+104]`. `[object+44]` is copied through `[*global+108]`.
//! Those three pointers then advance by 16, 16 and 4. `r13`/`r14` and
//! `fr14`/`fr15` are restored. `r0` is 108.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = word(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn push_f(cpu: &mut Cpu, mem: &mut Memory, value: f32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], value);
}

fn pop_f(cpu: &mut Cpu, mem: &Memory) -> f32 {
    let value = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push_f(cpu, mem, cpu.fr[15]);
    push_f(cpu, mem, cpu.fr[14]);
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    let frame = cpu.r[15];
    cpu.r[13] = cpu.r[4];

    cpu.r[0] = cpu.r[13].wrapping_add(36);
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[15] = 1.0;
    cpu.set_t(cpu.fr[15] > cpu.fr[2]); // fcmp/gt fr2, fr15
    if cpu.t() {
        cpu.fr[15] = cpu.fr[2];
    }

    cpu.r[1] = cpu.r[13].wrapping_add(40);
    cpu.r[0] = 0x8c13c038; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[14] = 1.0;
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 2.0
    cpu.fr[1] *= cpu.fr[0];
    cpu.fr[2] -= cpu.fr[1];
    cpu.set_t(cpu.fr[14] > cpu.fr[2]); // fcmp/gt fr2, fr14
    if cpu.t() {
        cpu.fr[14] = cpu.fr[2];
    }

    cpu.r[1] = word(mem, 0x8c13c03c); // 0x8c2ab7b0
    cpu.r[0] = frame;
    push(cpu, mem, cpu.r[0]);
    cpu.r[4] = cpu.r[13];
    cpu.r[0] = word(mem, 0x8c13c040); // 0x8c0b4f5c
    cpu.r[5] = cpu.r[13];
    cpu.pr = 0x8c13bf90;
    cpu.r[4] = cpu.r[4].wrapping_add(12); // jsr delay
    cpu.r[14] = word(mem, cpu.r[1]);
    crate::registry::p_8c0b4f5c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    cpu.r[0] = 0;
    cpu.fr[6] = mem.read_f32(cpu.r[0].wrapping_add(frame)).unwrap_or(0.0);
    cpu.r[1] = cpu.r[14];
    cpu.r[7] = cpu.r[13];
    cpu.r[0] = 4;
    cpu.fr[5] = mem.read_f32(cpu.r[0].wrapping_add(frame)).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[6];
    cpu.r[4] = cpu.r[13];
    cpu.r[0] = 8;
    cpu.fr[4] = mem.read_f32(cpu.r[0].wrapping_add(frame)).unwrap_or(0.0);
    cpu.r[1] = cpu.r[1].wrapping_add(104);
    cpu.fr[0] = mem.read_f32(cpu.r[13]).unwrap_or(0.0);
    cpu.fr[3] *= cpu.fr[14];
    cpu.r[0] = word(mem, cpu.r[1]);
    cpu.r[7] = cpu.r[7].wrapping_add(4);
    cpu.fr[3] += cpu.fr[0];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[3]);
    cpu.fr[1] = cpu.fr[5];
    cpu.r[0] = word(mem, cpu.r[1]);
    cpu.fr[1] *= cpu.fr[14];
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.fr[2] = cpu.fr[4];
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.fr[1] += cpu.fr[0];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
    cpu.fr[2] *= cpu.fr[14];
    cpu.r[0] = word(mem, cpu.r[1]);
    cpu.r[6] = cpu.r[14];
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[1] = 1.0;
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.fr[2] += cpu.fr[0];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[2]);
    cpu.r[6] = cpu.r[6].wrapping_add(100);
    cpu.r[0] = word(mem, cpu.r[1]);
    cpu.fr[6] *= cpu.fr[15];
    cpu.r[5] = cpu.r[14];
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
    cpu.fr[5] *= cpu.fr[15];
    cpu.fr[0] = mem.read_f32(cpu.r[13]).unwrap_or(0.0);
    cpu.fr[4] *= cpu.fr[15];
    cpu.r[3] = word(mem, cpu.r[6]);
    cpu.fr[6] += cpu.fr[0];
    cpu.r[5] = cpu.r[5].wrapping_add(108);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[6]);
    cpu.r[0] = 104;
    cpu.r[3] = word(mem, cpu.r[6]);
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.fr[5] += cpu.fr[0];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[5]);
    cpu.r[3] = word(mem, cpu.r[6]);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    cpu.fr[4] += cpu.fr[0];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[4]);
    cpu.r[3] = word(mem, cpu.r[6]);
    cpu.r[3] = cpu.r[3].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]);
    cpu.r[4] = word(mem, cpu.r[5]);
    cpu.r[3] = word(mem, cpu.r[13].wrapping_add(44));
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    cpu.r[1] = word(mem, cpu.r[1]);
    cpu.r[1] = cpu.r[1].wrapping_add(16);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]);
    cpu.r[1] = word(mem, cpu.r[6]);
    cpu.r[0] = 100;
    cpu.r[1] = cpu.r[1].wrapping_add(16);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]);
    cpu.r[1] = word(mem, cpu.r[5]);
    cpu.r[0] = 108;
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]);

    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
    cpu.fr[14] = pop_f(cpu, mem);
    cpu.fr[15] = pop_f(cpu, mem); // rts delay
}
