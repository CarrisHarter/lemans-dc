//! 0x8c1ac8d4 — zero floats at r4+24..36; [r4+4]=0, byte[r4]=0, [r4+12]=[r4+16]=[r4+40]=0, r1=1→[r4+20] (rts delay), r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = 0.0; // fldi0 fr0
    for off in [24u32, 28, 32, 36] {
        let _ = mem.write_u32(cpu.r[4] + off, cpu.fr[0].to_bits());
    }
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[3]);
    cpu.r[1] = 1;
    let _ = mem.write_u32(cpu.r[4] + 40, cpu.r[3]);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u8(cpu.r[4], 0);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 20, cpu.r[1]); // rts delay slot
}
