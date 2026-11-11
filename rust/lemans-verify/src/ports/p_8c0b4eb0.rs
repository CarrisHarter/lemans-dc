//! 0x8c0b4eb0 — subtract: with r3=`[r15]`, stores [r4]-[r5] to [r3] and
//! [r4+4]-[r5+4] to [r3+4]; the second store is the rts delay slot after
//! `add #4,r3`.
//!
//! Leftover registers: r0=r5+4, r1=r4+4, r3=r3+4; fr0=[r5+4], fr1=first
//! difference, fr2=second difference.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let a = cpu.r[4];
    let b = cpu.r[5];
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);

    let d0 = mem.read_f32(a).unwrap_or(0.0) - mem.read_f32(b).unwrap_or(0.0);
    let b4 = mem.read_f32(b + 4).unwrap_or(0.0);
    let d1 = mem.read_f32(a + 4).unwrap_or(0.0) - b4;

    let _ = mem.write_f32(dst, d0);
    let _ = mem.write_f32(dst + 4, d1); // delay slot of `rts`

    cpu.fr[0] = b4;
    cpu.fr[1] = d0;
    cpu.fr[2] = d1;
    cpu.r[0] = b + 4;
    cpu.r[1] = a + 4;
    cpu.r[3] = dst + 4;
}
