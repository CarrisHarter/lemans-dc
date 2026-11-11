//! 0x8c0824c0 — set_handler_default.
//!
//! `tst r5,r5 ; bf skip ; r5 = [0x8c0824cc] ; skip: rts ; [r4+16]=r5` — a null
//! handler is replaced by the default from the pool. T is the `tst` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let null = cpu.r[5] == 0;
    cpu.set_t(null); // tst r5,r5
    if null {
        cpu.r[5] = mem.read_u32(0x8C08_24CC).unwrap_or(0);
    }
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[5]); // delay slot of `rts`
}
