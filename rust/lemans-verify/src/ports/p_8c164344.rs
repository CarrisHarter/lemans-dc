//! 0x8c164344 — init struct at r4: [0]=2,[4]=1,[8]=1,[12]=1,[16]=2,[20..32]=0,[36]=1,[40..44]=0,[48]=2, rts delay [52]=0; r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = 2;
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
    cpu.r[3] = 1;
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[3]);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[5]);
    for off in [20u32, 24, 28, 32] {
        let _ = mem.write_u32(cpu.r[4] + off, cpu.r[1]);
    }
    let _ = mem.write_u32(cpu.r[4] + 36, cpu.r[3]);
    for off in [40u32, 44] {
        let _ = mem.write_u32(cpu.r[4] + off, cpu.r[1]);
    }
    let _ = mem.write_u32(cpu.r[4] + 48, cpu.r[5]);
    let _ = mem.write_u32(cpu.r[4] + 52, cpu.r[1]); // rts delay slot
}
