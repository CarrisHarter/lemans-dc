//! 0x8c1739f4 — raise SR IMASK to 15, then lower it to `r4 & 255` unless the
//! old IMASK was already >= that value (in which case SR keeps the raised mask).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[2] = (-16i32) as u32; // mov #-16,r2
    cpu.r[2] &= 0xff; // extu.b r2,r2
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[2] |= cpu.r[0];
    cpu.sr = cpu.r[2]; // ldc r2,sr
    cpu.r[1] = cpu.r[0];
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    cpu.r[4] &= 0xff; // extu.b r4,r4
    cpu.set_t((cpu.r[0] as i32) >= (cpu.r[4] as i32)); // cmp/ge r4,r0
    if !cpu.t() {
        cpu.r[4] <<= 2; // shll2
        cpu.r[4] <<= 2; // shll2
        cpu.r[2] = (-16i32) as u32;
        cpu.r[2] &= 0xff; // extu.b r2,r2
        cpu.r[2] = !cpu.r[2]; // not r2,r2
        cpu.r[1] &= cpu.r[2];
        cpu.r[1] |= cpu.r[4];
    }
    cpu.sr = cpu.r[1]; // ldc r1,sr
}
