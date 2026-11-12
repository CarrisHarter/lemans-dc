//! 0x8c225392 — strlen capped at 12; rts delay r0=r5 (count).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = 0;
    cpu.r[7] = 12;
    cpu.r[5] = cpu.r[6];
    while cpu.r[6] < 12 {
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
        if cpu.r[3] == 0 { break; }
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay
    }
    cpu.sr = (cpu.sr & !1) | 1; // cmp/ge left T=1
    cpu.r[0] = cpu.r[5]; // rts delay
}
