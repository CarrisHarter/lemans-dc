//! 0x8c1b8248 — *r4 = 0x8c1f1b68 (overwrites 0x8c1f0318), zeros at +4..+28, rts delay [32]=0; r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8c1f0318;
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = 0x8c1f1b68;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    for off in [4u32, 8, 12, 16, 20, 24, 28] {
        let _ = mem.write_u32(cpu.r[4] + off, cpu.r[1]);
    }
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 32, cpu.r[1]); // rts delay slot
}
