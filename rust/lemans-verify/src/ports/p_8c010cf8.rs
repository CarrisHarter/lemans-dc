//! 0x8c010cf8 — float conversion prologue; the fr4 == 0.0 branch stores 0 at @r4.
//!
//! Only the zero-input branch is pinnable by a spec: every non-zero branch ends
//! in a store to the global at 0x8c23d8b8, which lies outside the mapped image.
//! Original side effects on the zero path: r15 -4/+4 (stack window, excluded),
//! fr3 := 0, r5 := r15, r3 := 0x7f800000, r6 := fr4_bits & 0x7f800000,
//! r1 := 0, *r4 := 0, fr0 := fr4, T = 1 (fcmp/eq 0 == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.fr[3] = 0.0; // fldi0 fr3
    let zero = cpu.fr[4] == 0.0;
    cpu.set_t(zero); // fcmp/eq fr3,fr4
    cpu.r[5] = cpu.r[15]; // mov r15,r5
    cpu.r[3] = 0x7f800000; // mov.l literal
    let _ = mem.write_u32(cpu.r[5], cpu.fr[4].to_bits()); // fmov fr4,@r5 (stack)
    let bits = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[6] = bits & cpu.r[3]; // bf.s delay slot: and r3,r6
    if zero {
        cpu.r[1] = 0;
        let _ = mem.write_u32(cpu.r[4], 0); // mov.l r1,@r4
    } else {
        // Non-zero branches are not pinnable (store to 0x8c23d8b8); mirror the
        // register arithmetic up to the first unmapped access.
        cpu.r[3] = 0x7f800000;
        cpu.r[7] = 0x7fffff;
        cpu.set_t(cpu.r[6] == cpu.r[3]); // cmp/eq r3,r6
        if cpu.r[6] != cpu.r[3] {
            cpu.r[1] = bits;
            cpu.set_t(cpu.r[1] & cpu.r[7] == 0); // tst r1,r7
        }
    }
    cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot
}
