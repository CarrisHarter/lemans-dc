//! 0x8c216e18 — strstr-style count: r7 counts outer advances; rts r0=r7 on match.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = 0;
    loop {
        cpu.r[3] = (mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32) as u32;
        if cpu.r[3] == 0 { break; }
        cpu.r[6] = cpu.r[5];
        loop {
            cpu.r[2] = (mem.read_u8(cpu.r[6]).unwrap_or(0) as i8 as i32) as u32;
            cpu.r[3] = (mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32) as u32;
            if cpu.r[2] == cpu.r[3] {
                cpu.set_t(true); // cmp/eq r2,r3 taken by bt
                cpu.r[0] = cpu.r[7];
                return;
            }
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[3] = (mem.read_u8(cpu.r[6]).unwrap_or(0) as i8 as i32) as u32;
            if cpu.r[3] != 0 { continue; }
            break;
        }
        cpu.r[7] = cpu.r[7].wrapping_add(1);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
    }
    cpu.sr |= 1;
    cpu.r[0] = cpu.r[7]; // rts delay
}
