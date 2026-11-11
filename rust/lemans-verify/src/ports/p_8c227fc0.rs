//! 0x8c227fc0 — scan table at 0x8c2c6fa0 stride 0x1c4 up to 0xe20; rts delay r0 = r5 (or r0+r5 if hit 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = 0xe20;
    cpu.r[4] = 0;
    cpu.r[6] = 0x1c4;
    cpu.r[5] = cpu.r[4];
    cpu.r[0] = 0x8c2c6fa0;
    loop {
        cpu.r[3] = mem.read_u32(cpu.r[0] + cpu.r[4]).unwrap_or(0);
        if cpu.r[3] == 0 {
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]); // rts delay
            cpu.sr |= 1;
            return;
        }
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[6]);
        if cpu.r[4] >= cpu.r[7] { break; } // cmp/hs r7,r4: T = r4>=r7
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]); // bf.s delay
    }
    cpu.r[0] = 0;
    cpu.sr |= 1;
}
