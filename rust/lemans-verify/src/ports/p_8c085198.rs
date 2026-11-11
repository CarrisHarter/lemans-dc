//! 0x8c085198 — unconditional: param[1] = g[2]*fr4 + g[1], then *param = 1.
//!
//! Side effects the C does not show: the `mov.l r0,@r4` store is the rts
//! DELAY SLOT, so *param := 1 always; r0 := 1, r1 := param+4, r3 := g+4,
//! fr0 := g[1], fr1 := result. No cmp/tst: T is left untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0851b8).unwrap_or(0); // mov.l literal -> 0x8c26c230
    cpu.r[1] = cpu.r[4].wrapping_add(4); // mov r4,r1; add #4,r1
    let g = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r3
    cpu.r[3] = g;
    cpu.r[0] = g.wrapping_add(8); // mov r3,r0; add #8,r0
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1
    cpu.r[3] = g.wrapping_add(4); // add #4,r3
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr0
    cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
    cpu.r[0] = 1; // mov #1,r0
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    // rts delay slot: mov.l r0,@r4 — always runs
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
}
