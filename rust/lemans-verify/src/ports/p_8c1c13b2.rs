//! 0x8c1c13b2 — scan string at 0x8c212c90 counting r6 until byte==r4 or NUL; rts r0=r5 or 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = 0x8c212c90;
    cpu.r[3] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
    if cpu.r[3] == 0 { cpu.r[0] = 0; return; }
    cpu.r[6] = 0;
    loop {
        if cpu.r[4] == cpu.r[6] { cpu.r[0] = cpu.r[5]; return; }
        cpu.r[3] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; cpu.r[5] = cpu.r[5].wrapping_add(1);
        if cpu.r[3] != 0 { continue; }
        cpu.r[3] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
        cpu.r[6] = cpu.r[6].wrapping_add(1); // bf.s delay always executes
        if cpu.r[3] != 0 { continue; }
        cpu.r[0] = 0; cpu.sr |= 1; return;
    }
}
