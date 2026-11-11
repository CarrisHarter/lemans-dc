//! 0x8c085110 — state machine step 2->1: param[1] = g[2]*fr4 + g[1] - param[2] + param[1].
//!
//! Side effects the C does not show: on the taken path r0 := 1, r1 := param+4,
//! r5 := g+4, r3 := g+8, fr0 := old param[1], fr1 := param[2], fr2 := result;
//! T = (*param == 2) from cmp/eq #2. On the skip path r0 keeps the loaded state.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let state = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = state;
    cpu.set_t(state == 2); // cmp/eq #2,r0
    if state != 2 {
        return; // bf -> rts, delay slot nop
    }
    cpu.r[0] = mem.read_u32(0x8c085144).unwrap_or(0); // mov.l literal -> 0x8c26c230
    cpu.r[1] = cpu.r[4].wrapping_add(4); // mov r4,r1; add #4,r1
    let g = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r5
    cpu.r[5] = g;
    cpu.r[3] = g.wrapping_add(8); // add #8,r3
    cpu.r[5] = g.wrapping_add(4); // add #4,r5
    cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr2
    cpu.r[0] = cpu.r[4].wrapping_add(8); // mov r4,r0; add #8,r0
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1
    cpu.fr[2] = cpu.fr[2] * cpu.fr[4]; // fmul fr4,fr2
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr0
    cpu.r[0] = 1; // mov #1,r0
    cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
    cpu.fr[2] = cpu.fr[2] - cpu.fr[1]; // fsub fr1,fr2
    cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2
    let _ = mem.write_f32(cpu.r[1], cpu.fr[2]); // fmov fr2,@r1
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]); // mov.l r0,@r4
}
