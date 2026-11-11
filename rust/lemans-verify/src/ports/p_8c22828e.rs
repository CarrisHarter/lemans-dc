//! 0x8c22828e — flag bit search; rts r0 = bit index or -1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x8c2c6f8c;
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[6] = 0;
    if cpu.r[4] != 0 { // tst: T=1 if zero; bt.s taken when T=1 -> block runs when r4!=0
        cpu.r[5] = 1;
        cpu.r[1] = 0x8c2c6f8c;
        let _ = mem.write_u32(cpu.r[1], cpu.r[6]);
        cpu.r[2] = 0x8c2c6f98;
        let _ = mem.write_u32(cpu.r[2], cpu.r[5]);
        cpu.r[3] = 0x8c2c6f80;
        cpu.r[4] = !cpu.r[4];
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[0] &= cpu.r[4];
        let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
        cpu.r[3] = cpu.r[3].wrapping_add(24);
        let _ = mem.write_u32(cpu.r[3], cpu.r[6]);
    }
    cpu.r[1] = 0x8c2c7e24;
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    if cpu.r[2] != 0 { // tst: T=1 if zero; bt skips when T=1 -> block when r2!=0
        cpu.r[2] = 0x8c2c7e20;
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.r[5] = ((cpu.r[5] as i32) >> cpu.r[3]) as u32; // shad r3,r5 (r3 may be neg)
        cpu.r[3] = 0x8c2c6f80;
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        if cpu.r[0] & cpu.r[5] == 0 { // tst: T=1 if zero; bf taken when T=0 -> rts -1 when (r0&r5)!=0
            // fall through? bf 0x8c2282f2 when T=0; T=1 (zero) continues
        } else {
            cpu.r[0] = 0xffffffff; return; // rts -1 (delay nop)
        }
    }
    cpu.r[0] = 0x8c2c6f74;
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[2] = 0;
    if cpu.r[2] >= cpu.r[3] { cpu.r[0] = 0xffffffff; return; } // cmp/hs r3,r2: T = r2>=r3; bt.s -> -1 when T=1
    cpu.r[4] = cpu.r[6];
    loop {
        cpu.r[1] = 0x8c2c6f80;
        cpu.r[3] = ((cpu.r[5] as i32) >> cpu.r[4]) as u32; // shad r4,r3
        cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        if cpu.r[2] & cpu.r[3] == 0 { cpu.r[0] = cpu.r[4]; cpu.sr |= 1; return; } // bf taken when T=0; T=1 (zero) -> rts r0=r4
        cpu.r[2] = 0x8c2c6f74;
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        if cpu.r[4] >= cpu.r[3] { break; } // cmp/hs r3,r4: T = r4>=r3; bf loops while T=0
    }
    cpu.r[0] = 0xffffffff; // rts -1
}
