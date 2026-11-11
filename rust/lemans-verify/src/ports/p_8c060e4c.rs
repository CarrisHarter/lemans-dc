//! 0x8c060e4c — r4 += 4; store fr4 at the new r4 (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // rts delay: fmov fr4,@r4
}
