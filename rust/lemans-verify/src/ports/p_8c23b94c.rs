//! 0x8c23b94c — search array at 0x8c23d798: for i in 0..*(base+8): if *(base+32)+i*24 == r4 { clear low word of entry }; bare rts.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = 0x8c23d798;
    cpu.r[6] = 0;
    cpu.r[2] = cpu.r[6]; // mov r6,r2
    cpu.r[7] = 0xfffe;
    loop {
        cpu.r[3] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
        if cpu.r[6] >= cpu.r[3] { break; } // cmp/hs r3,r6 at entry: T = r6>=r3; bt exits when T=1
        cpu.r[0] = ((cpu.r[6] << 1).wrapping_add(cpu.r[6]) << 2) << 1;
        cpu.r[2] = mem.read_u32(cpu.r[5] + 32).unwrap_or(0);
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]);
        if cpu.r[4] == cpu.r[0] {
            cpu.r[3] = ((cpu.r[6] << 1).wrapping_add(cpu.r[6]) << 2) << 1;
            cpu.r[2] = mem.read_u32(cpu.r[5] + 32).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
            cpu.r[1] = (mem.read_u16(cpu.r[3]).unwrap_or(0) as u32) & cpu.r[7];
            let _ = mem.write_u16(cpu.r[3], cpu.r[1] as u16);
        }
        cpu.r[6] = cpu.r[6].wrapping_add(1);
    }
    cpu.sr |= 1;
}
