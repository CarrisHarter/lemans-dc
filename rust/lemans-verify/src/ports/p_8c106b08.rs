//! 0x8c106b08 — load `*(0x8c2ab3b0)` into r0; if it is zero return the
//! alternative pointer 0x8c2ab3c8, otherwise return the loaded value.
//!
//! `mov.l 0x8c106b1c,r0 ; mov.l @r0,r0 ; tst r0,r0 ; bt +5 ; rts ; nop`
//! then `mov.l 0x8c106b20,r0 ; rts ; nop`. Both rts delay slots are nops.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c106b1c).unwrap(); // 0x8c2ab3b0
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c106b20).unwrap(); // 0x8c2ab3c8
    }
}
