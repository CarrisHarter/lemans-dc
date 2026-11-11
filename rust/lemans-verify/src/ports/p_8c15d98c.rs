//! 0x8c15d98c — fpul = *(r4+12); rts delay fr0 = float(fpul).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.fpul = cpu.r[0];
    cpu.fr[0] = cpu.fpul as i32 as f32; // rts delay
}
