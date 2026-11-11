//! 0x8c10eae0 — byte-compare loop r4 vs r5 (same shape as 0x8c05e038); r0=diff in rts delay.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[3] & 0xff;
    loop {
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] == 0 {
            return;
        }
        let d = mem.read_u8(cpu.r[4]).unwrap_or(0);
        let s = mem.read_u8(cpu.r[5]).unwrap_or(0);
        cpu.r[3] = d as u32;
        cpu.r[0] = cpu.r[3];
        cpu.r[1] = s as u32;
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[0] == cpu.r[1]);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        if cpu.r[0] == cpu.r[1] {
            continue;
        }
        cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]);
        return;
    }
}
