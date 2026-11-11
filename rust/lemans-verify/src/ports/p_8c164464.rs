//! 0x8c164464 — struct init at r4: [0]=0,[4]=4,[8]=3,[12]=1,[16..28]=0,[32..44]=20, [48]=0; r0=r4, rts delay [44]=20.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    cpu.r[0] = 4;
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[1] = 20;
    cpu.r[0] = 3;
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]);
    cpu.r[0] = 1;
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[0]);
    for off in [16u32, 20, 24, 28] {
        let _ = mem.write_u32(cpu.r[4] + off, cpu.r[3]);
    }
    let _ = mem.write_u32(cpu.r[4] + 48, cpu.r[3]);
    cpu.r[0] = cpu.r[4];
    for off in [32u32, 36, 40] {
        let _ = mem.write_u32(cpu.r[4] + off, cpu.r[1]);
    }
    let _ = mem.write_u32(cpu.r[4] + 44, cpu.r[1]); // rts delay slot
}
