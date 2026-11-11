//! 0x8c0853e8 — compare 4 halfwords; returns 1 if all equal else 0 (inverted
//! flag logic vs 0x8c08539c). r1/r6/r7 are match flags, r0 = r1 in rts delay.

use sh4_oracle::{Cpu, Memory};

fn exts_w(v: u16) -> u32 {
    (v as i16 as i32) as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = exts_w(mem.read_u16(cpu.r[4]).unwrap_or(0));
    cpu.r[1] = 0;
    cpu.r[0] = exts_w(mem.read_u16(cpu.r[5]).unwrap_or(0));
    cpu.r[6] = cpu.r[1];
    cpu.set_t(cpu.r[0] == cpu.r[3]); // cmp/eq r0,r3
    cpu.r[7] = cpu.r[1]; // bf.s delay slot
    if cpu.r[0] == cpu.r[3] {
        cpu.r[3] = exts_w(mem.read_u16(cpu.r[4] + 2).unwrap_or(0));
        cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + 2).unwrap_or(0));
        cpu.set_t(cpu.r[0] == cpu.r[3]);
        if cpu.r[0] == cpu.r[3] {
            cpu.r[7] = 1;
        }
    }
    cpu.r[0] = cpu.r[7] & 0xff; // extu.b r7,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[3] = exts_w(mem.read_u16(cpu.r[4] + 4).unwrap_or(0));
        cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + 4).unwrap_or(0));
        cpu.set_t(cpu.r[0] == cpu.r[3]);
        if cpu.r[0] == cpu.r[3] {
            cpu.r[6] = 1;
        }
        cpu.r[0] = cpu.r[6] & 0xff; // extu.b r6,r0
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] != 0 {
            cpu.r[3] = exts_w(mem.read_u16(cpu.r[4] + 6).unwrap_or(0));
            cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + 6).unwrap_or(0));
            cpu.set_t(cpu.r[0] == cpu.r[3]);
            if cpu.r[0] == cpu.r[3] {
                cpu.r[1] = 1;
            }
        }
    }
    cpu.r[0] = cpu.r[1]; // rts delay slot
}
