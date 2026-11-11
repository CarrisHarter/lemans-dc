//! 0x8c216d44 — lower-to-upper: if 'a'<=exts.b(r4)<='z' { r4 -= 32 }.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    let c = cpu.r[4] as i8;
    if (97..=122).contains(&c) {
        cpu.r[4] = cpu.r[4].wrapping_sub(32);
    }
}
