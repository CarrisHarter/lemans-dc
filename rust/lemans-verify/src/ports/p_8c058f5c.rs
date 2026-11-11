//! 0x8c058f5c — return 1 iff the four floats at [r4..r4+12] equal [r5..r5+12].
//!
//! ```text
//! mov r4,r1 ; mov r5,r0 ; add #4,r1 ; add #4,r0
//! fmov @r1,fr1 ; fmov @r0,fr0 ; fcmp/eq fr0,fr1 ; bt next
//! rts ; mov #0,r0          ; early exit: r0 = 0
//! … repeated at +8 and +12 …
//! fmov @r4,fr1 ; fmov @r5,fr0 ; fcmp/eq fr0,fr1
//! rts ; movt r0            ; r0 = T
//! ```
//!
//! Hidden side effects the C does not show: r1 and r0 are clobbered every
//! block (r1 = r4+12, r0 = r5+12 on the fall-through path), fr0/fr1 keep the
//! last compared pair, T is the last fcmp/eq result, and each early exit
//! leaves r0 = 0 with the r1/r0 of *that* block (r4+4 / r4+8 / r4+12), not
//! the final ones.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    for off in [4u32, 8, 12] {
        cpu.r[1] = r4.wrapping_add(off);
        cpu.r[0] = r5.wrapping_add(off);
        cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
        cpu.set_t(cpu.fr[1] == cpu.fr[0]); // fcmp/eq fr0,fr1: T = fr1 == fr0
        if !cpu.t() {
            cpu.r[0] = 0; // delay slot of rts: mov #0,r0
            return;
        }
    }
    cpu.fr[1] = mem.read_f32(r4).unwrap();
    cpu.fr[0] = mem.read_f32(r5).unwrap();
    cpu.set_t(cpu.fr[1] == cpu.fr[0]); // fcmp/eq fr0,fr1
    cpu.r[0] = cpu.t() as u32; // delay slot of rts: movt r0
}
