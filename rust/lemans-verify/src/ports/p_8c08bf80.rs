//! 0x8c08bf80 — copy 16 bytes (4 floats) from r5 to r4, return r4.
//!
//! Side effects the C does not show: r1 ends as r4+4, r5 ends as r5+12
//! (`add #12,r5` in the middle of the copy), fr0 ends as the float at
//! src+12, and r0 = r4 comes from the `rts` delay slot, not the last
//! `add #12,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    let dst = cpu.r[4];
    cpu.fr[0] = mem.read_f32(src).unwrap_or(0.0); // fmov @r5,fr0
    cpu.r[1] = src; // mov r5,r1
    cpu.r[0] = src; // mov r5,r0
    let _ = mem.write_f32(dst, cpu.fr[0]); // fmov fr0,@r4
    cpu.r[1] = src.wrapping_add(4); // add #4,r1
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
    cpu.r[5] = src.wrapping_add(12); // add #12,r5
    cpu.r[0] = src.wrapping_add(8); // add #8,r0
    cpu.r[1] = dst; // mov r4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
    cpu.r[0] = dst; // mov r4,r0
    cpu.r[0] = cpu.r[0].wrapping_add(8); // add #8,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr0
    cpu.r[0] = dst; // mov r4,r0
    cpu.r[0] = cpu.r[0].wrapping_add(12); // add #12,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.r[0] = dst; // rts delay slot: mov r4,r0
}
