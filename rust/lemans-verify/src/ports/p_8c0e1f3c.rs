//! 0x8c0e1f3c — byte[r4+97] := (byte r5 != 0 && byte at r4+98 != 0) ? 1 : 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5] & 0xff; // extu.b r5,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[1] = 0; // bt.s delay slot
    if cpu.r[0] != 0 {
        cpu.r[0] = cpu.r[4].wrapping_add(98);
        let b = mem.read_u8(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = b as u32; // extu.b x2
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            cpu.r[1] = 1;
        }
    }
    cpu.r[0] = 97;
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8); // rts delay
}
