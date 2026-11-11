//! 0x8c08bfac — float clamp step on a 4-float vector at r4.
//!
//! `fcmp/gt fr2,fr0` sets T = (fr0 > fr2) where fr2 is the float at 0x8c08c010
//! (0.001) and fr0 is p[3]. If p[3] > 0.001: p[0] += p[1], p[3] -= *(0x8c1cfa18),
//! and if the new p[3] is still > 0.001 the function returns immediately;
//! otherwise p[1] = p[2] and p[2] = 0 (the latter in the `rts` delay slot,
//! with r4 already advanced by 8). If p[3] <= 0.001: p[0] += p[1] + 2*p[2] and
//! p[1] += 2*p[2]. Side effects the C mangles: r3 = r4+12, r1 = r4+4,
//! r0 = 0x8c1cfa18 (then-branch) or r4+8 (else), fr0/fr1/fr2 all clobbered,
//! and T holds the last `fcmp/gt` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];
    cpu.r[3] = p.wrapping_add(12); // mov r4,r3 ; add #12,r3
    cpu.r[0] = 0x8c08c010; // mova 0x8c08c010,r0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr0
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr2
    cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt fr2,fr0: T = fr0 > fr2
    if !cpu.t() {
        // else branch at 0x8c08bfe2
        cpu.r[1] = p.wrapping_add(4); // mov r4,r1 ; add #4,r1
        cpu.fr[1] = mem.read_f32(p).unwrap_or(0.0); // fmov @r4,fr1
        cpu.r[0] = p; // mov r4,r0
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
        cpu.r[0] = cpu.r[0].wrapping_add(8); // add #8,r0
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        let _ = mem.write_f32(p, cpu.fr[1]); // fmov fr1,@r4
        cpu.fr[1] = mem.read_f32(p).unwrap_or(0.0); // fmov @r4,fr1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        let _ = mem.write_f32(p, cpu.fr[1]); // fmov fr1,@r4
        cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
        cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
        cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
        let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
        return; // rts ; nop
    }
    // then branch at 0x8c08bfba
    cpu.r[1] = p.wrapping_add(4); // mov r4,r1 ; add #4,r1
    cpu.fr[1] = mem.read_f32(p).unwrap_or(0.0); // fmov @r4,fr1
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr0
    cpu.r[0] = mem.read_u32(0x8c08c014).unwrap_or(0); // mov.l literal (0x8c1cfa18)
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    let _ = mem.write_f32(p, cpu.fr[1]); // fmov fr1,@r4
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr1
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[1]); // fmov fr1,@r3
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr0
    cpu.set_t(cpu.fr[0] > cpu.fr[2]); // fcmp/gt fr2,fr0
    if cpu.t() {
        return; // bt 0x8c08c00a ; rts ; nop
    }
    cpu.r[4] = p.wrapping_add(8); // add #8,r4
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // fmov @r4,fr1
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // rts delay slot: fmov fr0,@r4
}
