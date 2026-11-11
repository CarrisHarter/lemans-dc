//! 0x8c221f1c — r2=0x8c23d480; r3=0; rts delay *r2=r3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x8c23d480;
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[2], cpu.r[3]); // rts delay
}
