//! 0x8c123d9c — byte r5 clamped to 10 → byte[r4+56]; *(r4+60)=0.0f; if byte at
//! r4+57 is 0 or > 10, copy byte r4+56 into r4+57.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = cpu.r[5] & 0xff; // extu.b r5,r1
    cpu.r[0] = 10;
    cpu.set_t(cpu.r[1] > cpu.r[0]); // cmp/hi r0,r1: T=(r1>r0)
    if cpu.r[1] > 10 {
        cpu.r[5] = cpu.r[0]; // clamp to 10
    }
    cpu.r[0] = 56;
    cpu.r[1] = cpu.r[4];
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[5] as u8);
    cpu.r[1] = cpu.r[4].wrapping_add(60);
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits());
    cpu.r[0] = cpu.r[4].wrapping_add(57);
    let b = mem.read_u8(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = b as u32;
    cpu.r[1] = cpu.r[0] & 0xff; // extu.b r0,r1
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1
    let mut copy = cpu.r[1] == 0;
    if !copy {
        cpu.r[0] = 10;
        cpu.set_t(cpu.r[1] > cpu.r[0]); // cmp/hi r0,r1
        copy = cpu.r[1] > 10;
    }
    if copy {
        cpu.r[0] = cpu.r[4].wrapping_add(56);
        let v = mem.read_u8(cpu.r[0]).unwrap_or(0);
        cpu.r[1] = v as u32; // mov.b sign-extends, then extu.b
        cpu.r[0] = 57;
        cpu.r[1] &= 0xff;
        let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u8);
    }
}
