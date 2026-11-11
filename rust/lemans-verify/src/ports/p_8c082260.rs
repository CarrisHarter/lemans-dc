//! 0x8c082260 — init FPU struct at r4 (single-precision path):
//! [16]=1.0f, [20]=0, [24]=0x41400000, [28]=[32]=0, [0..12] copied from 0x8c1d0f28.
//!
//! Original side effects: r1 = 0 then last copied word, r3/r5 walk the source,
//! r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = cpu.r[4].wrapping_add(20);
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_u32(cpu.r[1], 0);
    cpu.r[0] = cpu.r[4].wrapping_add(16);
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[0].to_bits());
    cpu.r[3] = cpu.r[4].wrapping_add(24);
    cpu.fr[0] = f32::from_bits(mem.read_u32(0x8c082298).unwrap_or(0)); // mova+fmov
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[3], cpu.fr[0].to_bits());
    cpu.r[0] = cpu.r[4];
    cpu.r[5] = mem.read_u32(0x8c08229c).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 28, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 32, cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5] + 12).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]); // rts delay slot
}
