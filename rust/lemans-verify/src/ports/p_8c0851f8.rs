//! 0x8c0851f8 — if param[2] == 0.0 store fr4 there, then param[1] = param[2]*(float)*param; *param += 1.
//!
//! Side effects the C does not show: r1 := old *param, fpul := old *param,
//! r3 := param+4, r0 := old *param + 1 (the add runs before the rts delay-slot
//! store), fr0 := re-read of param[2] after the conditional store, T =
//! (original param[2] == 0.0) from fcmp/eq. The `fmov fr4,@r0` follows an
//! UNdelayed `bf` (0x8B), so it executes only when param[2] == 0.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];
    cpu.r[0] = p.wrapping_add(8); // mov r4,r0; add #8,r0
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1
    cpu.fr[0] = 0.0; // fldi0 fr0
    let eq = cpu.fr[0] == cpu.fr[1];
    cpu.set_t(eq); // fcmp/eq fr0,fr1
    if eq {
        // bf (0x8B) is undelayed: this fmov runs only on the fall-through path
        let _ = mem.write_f32(cpu.r[0], cpu.fr[4]); // fmov fr4,@r0
    }
    cpu.r[1] = mem.read_u32(p).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = p.wrapping_add(8); // mov r4,r0; add #8,r0
    cpu.r[3] = p.wrapping_add(4); // mov r4,r3; add #4,r3
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
    cpu.fpul = cpu.r[1]; // lds r1,fpul
    cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
    cpu.fr[1] = cpu.fr[0] * cpu.fr[1]; // fmul fr0,fr1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]); // fmov fr1,@r3
    cpu.r[0] = mem.read_u32(p).unwrap_or(0).wrapping_add(1); // mov.l @r4,r0; add #1,r0
    let _ = mem.write_u32(p, cpu.r[0]); // rts delay slot: mov.l r0,@r4
}
