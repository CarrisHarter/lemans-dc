//! 0x8c23296c — r3=*r4 (re-read if nonzero); *(r5+4)=r3; rts delay *r4=r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.r[3] != 0 {
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0); // delay of bra
    } else {
        cpu.r[3] = 0; // bra delay slot
    }
    let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]); // rts delay
}
