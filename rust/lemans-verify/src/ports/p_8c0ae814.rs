//! 0x8c0ae814 — init_record_with_arg.
//!
//! `[r4]=r5 ; r1=0 ; [r4+4]=0 ; r0=r4 ; [r4+8]=0 ; [r4+12]=0 ; rts ; [r4+16]=0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let _ = mem.write_u32(base, cpu.r[5]);
    cpu.r[1] = 0;
    let _ = mem.write_u32(base + 4, 0);
    cpu.r[0] = base;
    let _ = mem.write_u32(base + 8, 0);
    let _ = mem.write_u32(base + 12, 0);
    let _ = mem.write_u32(base + 16, 0); // delay slot of `rts`
}
