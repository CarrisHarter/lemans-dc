//! 0x8c0749f0 — scale a table entry by `1/fr4` unless it is the sentinel 5.
//!
//! ```text
//! r3 = 0xe8 ; r5 <<= 2 ; r1 = r4 ; r0 = r5 ; r1 += r3
//! r0 = [r1+r0] ; cmp/eq #5,r0 ; bt end
//! r4 += 16 ; r0 <<= 2 ; fr0 = 1.0 ; r0 = [r4+r0]
//! fr0 = fr0 / fr4 ; [r0] = fr0
//! end: rts ; nop
//! ```
//!
//! Hidden side effects: `r5` is shifted in place, `r1` keeps `r4+0xe8`, `r3`
//! keeps 0xe8, `r4` is advanced by 16 only on the non-sentinel path, `r0` ends
//! as the loaded sentinel or as the destination pointer, and T is the
//! `cmp/eq` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r3 = mem.read_u32(0x8C07_4A10).unwrap_or(0xe8);
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[1] = cpu.r[4];
    cpu.r[0] = cpu.r[5];
    cpu.r[1] = cpu.r[1].wrapping_add(r3);
    cpu.r[3] = r3;

    let entry = mem
        .read_u32(cpu.r[1].wrapping_add(cpu.r[0]))
        .unwrap_or(0);
    cpu.r[0] = entry;
    let sentinel = entry == 5;
    cpu.set_t(sentinel); // cmp/eq #5,r0
    if sentinel {
        return;
    }

    cpu.r[4] = cpu.r[4].wrapping_add(16);
    cpu.r[0] = cpu.r[0] << 2;
    cpu.fr[0] = 1.0; // fldi1 fr0
    cpu.r[0] = mem
        .read_u32(cpu.r[4].wrapping_add(cpu.r[0]))
        .unwrap_or(0);
    cpu.fr[0] = cpu.fr[0] / cpu.fr[4];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
}
