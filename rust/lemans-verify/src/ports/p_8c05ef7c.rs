//! 0x8c05ef7c — if byte r6 != 0: *(r4+8) |= r5, else *(r4+8) &= ~r5.
//!
//! Original side effects: r0 = extu.b(r6) then reused, T = (r6 byte == 0);
//! clear path leaves r1 = masked value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[6] & 0xff; // extu.b r6,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0); // mov.l @(8,r4),r0
        cpu.r[0] |= cpu.r[5];
        let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // rts delay slot
    } else {
        cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
        cpu.r[0] = !cpu.r[5]; // not r5,r0
        cpu.r[1] &= cpu.r[0];
        let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]); // rts delay slot
    }
}
