//! 0x8c0b4e80 — subtract_two_pairs.
//!
//! 'subs the two float pairs in place: `[r4] = [r4] sub [r5]` and
//! `[r4+4] = [r4+4] sub [r5+4]`; both pointers advance by 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
    let _ = mem.write_f32(cpu.r[4], cpu.fr[1]);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.fr[1] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
    let _ = mem.write_f32(cpu.r[4], cpu.fr[1]); // delay slot of `rts`
}
