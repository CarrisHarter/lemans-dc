//! 0x8c1bbf74 — pick a float from a table indexed by `r5` and scale it.
//!
//! Original:
//! ```text
//! mov r5,r0 ; cmp/eq #1,r0 ; bt case1 ; cmp/eq #2,r0 ; bt case2 ; bra def
//! case1: r1 = *0x8c23dbf8 + 0x100 ; def: +0xf0 ; case2: +0x110
//!   fmov @r1,fr0 ; (bra 0x8c1bbfb6 ; fmov fr0,@r3)  with r3 = r4+16
//! tail: add #16,r4 ; mova 0x8c1bbfd4,r0 ; fmov @r4,fr1 ; fmov @r0,fr0
//!   fmul fr0,fr1 ; rts ; fmov fr1,@r4   (delay)
//! ```
//! So `*(r4+16)` is loaded from the table, then overwritten in the rts delay
//! slot with `loaded * pool_float(0x8c1bbfd4)`. `r4` ends r4+16, `r0` ends
//! 0x8c1bbfd4, `r3` ends r4+16. T ends 1 for r5==1/2, 0 otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let base = mem.read_u32(0x8c23dbf8).unwrap_or(0); // mov.l @0x8c23dbf8 indirection
    let off = if cpu.r[5] == 1 {
        cpu.set_t(true); // cmp/eq #1,r0
        0x100
    } else if cpu.r[5] == 2 {
        cpu.set_t(true); // cmp/eq #2,r0 (the last T write wins)
        0x110
    } else {
        cpu.set_t(false);
        0xf0
    };
    let v = mem.read_f32(base.wrapping_add(off)).unwrap_or(0.0);
    cpu.r[1] = base.wrapping_add(off); // r1 keeps table base + offset
    cpu.fr[0] = v;
    cpu.r[3] = r4.wrapping_add(16);
    // case paths store fr0 in the bra delay slot, default directly: same store
    mem.write_f32(r4 + 16, v).ok();
    cpu.r[4] = r4.wrapping_add(16);
    cpu.r[0] = 0x8c1bbfd4; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // fmov @r4,fr1
    cpu.fr[0] = mem.read_f32(0x8c1bbfd4).unwrap_or(0.0); // fmov @r0,fr0
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // fmul fr0,fr1
    // rts delay slot
    mem.write_f32(cpu.r[4], cpu.fr[1]).ok();
}
