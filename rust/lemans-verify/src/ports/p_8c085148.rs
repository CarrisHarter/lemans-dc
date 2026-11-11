//! 0x8c085148 — state machine step 1->2: param[2] = g[2]*fr4 + g[1].
//!
//! Side effects the C does not show: on the taken path r0 := 2, r1 := param+8,
//! r3 := g+4, fr0 := g[1], fr1 := result; T = (*param == 1). Skip path keeps
//! r0 = loaded state, T = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let state = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = state;
    cpu.set_t(state == 1); // cmp/eq #1,r0
    if state != 1 {
        return; // bf -> rts, delay slot nop
    }
    cpu.r[0] = mem.read_u32(0x8c085170).unwrap_or(0); // mov.l literal -> 0x8c26c230
    cpu.r[1] = cpu.r[4].wrapping_add(8); // mov r4,r1; add #8,r1
    let g = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r3
    cpu.r[3] = g;
    cpu.r[0] = g.wrapping_add(8); // mov r3,r0; add #8,r0
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1
    cpu.r[3] = g.wrapping_add(4); // add #4,r3
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr0
    cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
    cpu.r[0] = 2; // mov #2,r0
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // mov.l r0,@r4
}
