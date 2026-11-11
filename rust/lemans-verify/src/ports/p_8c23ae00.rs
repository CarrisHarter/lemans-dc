//! 0x8c23ae00 — r5=0x8c2c9e88; r4 = (r4==r5); rts delay r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[5] = 0x8c2c9e88;
    let eq = cpu.r[4] == cpu.r[5];
    cpu.set_t(eq); // cmp/eq leaves T set
    cpu.r[4] = cpu.t() as u32; // movt r4
    cpu.r[0] = cpu.r[4]; // rts delay
}
