//! 0x8c217e3e — compare 5 bytes at r4+1 vs 0x8c23c2f0; r0=1 if all equal, else r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = cpu.r[4];
    cpu.r[7] = 0x8c23c2f0;
    cpu.r[5] = 5;
    cpu.r[4] = 0;
    cpu.r[6] = cpu.r[6].wrapping_add(1);
    loop {
        cpu.r[2] = (mem.read_u8(cpu.r[6]).unwrap_or(0) as i8 as i32) as u32; cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.r[3] = (mem.read_u8(cpu.r[7]).unwrap_or(0) as i8 as i32) as u32; cpu.r[7] = cpu.r[7].wrapping_add(1);
        let eq = cpu.r[2] == cpu.r[3];
        cpu.set_t(eq); // cmp/eq r2,r3
        if !eq {
            cpu.r[0] = 0; // rts delay
            return;
        }
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        if cpu.r[4] >= cpu.r[5] { break; } // cmp/ge r5,r4: T = r4>=r5
    }
    cpu.sr |= 1;
    cpu.r[0] = 1;
}
