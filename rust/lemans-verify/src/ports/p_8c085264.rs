//! 0x8c085264 — register param as current: *(0x8c26c230) = param, param[0..3] = 0, return param.
//!
//! Side effects the C does not show: r1 := param+4, fr0 := 0.0 (fldi0),
//! r0 := param (the rts delay slot `mov r4,r0` is the return value, not an
//! earlier copy). T untouched (no cmp/tst).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c085280).unwrap_or(0); // mov.l literal -> 0x8c26c230
    cpu.r[1] = cpu.r[4].wrapping_add(4); // mov r4,r1; add #4,r1
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // mov.l r4,@r0
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
    cpu.r[0] = cpu.r[4].wrapping_add(8); // add #8,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // mov.l r0,@r4
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
