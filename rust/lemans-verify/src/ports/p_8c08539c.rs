//! 0x8c08539c — compare 4 halfwords; returns 0 if all equal else 1.
//!
//! Original side effects: r1 = 1 then 0 on full match, r6/r7 = match flags,
//! r3/r0 = last compared halfwords (exts.w), r0 = r1 in rts delay slot,
//! T = last tst/cmp.

use sh4_oracle::{Cpu, Memory};

fn exts_w(v: u16) -> u32 {
    (v as i16 as i32) as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = exts_w(mem.read_u16(cpu.r[4]).unwrap_or(0));
    cpu.r[1] = 1;
    cpu.r[0] = exts_w(mem.read_u16(cpu.r[5]).unwrap_or(0));
    cpu.r[6] = cpu.r[1];
    cpu.set_t(cpu.r[0] == cpu.r[3]); // cmp/eq r0,r3
    cpu.r[7] = cpu.r[1]; // bf.s delay slot
    if cpu.r[0] == cpu.r[3] {
        cpu.r[3] = exts_w(mem.read_u16(cpu.r[4] + 2).unwrap_or(0));
        cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + 2).unwrap_or(0));
        cpu.set_t(cpu.r[0] == cpu.r[3]);
        if cpu.r[0] == cpu.r[3] {
            cpu.r[7] = 0;
        }
    }
    cpu.r[0] = cpu.r[7] & 0xff; // extu.b r7,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.r[3] = exts_w(mem.read_u16(cpu.r[4] + 4).unwrap_or(0));
        cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + 4).unwrap_or(0));
        cpu.set_t(cpu.r[0] == cpu.r[3]);
        if cpu.r[0] == cpu.r[3] {
            cpu.r[6] = 0;
        }
        cpu.r[0] = cpu.r[6] & 0xff; // extu.b r6,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            cpu.r[3] = exts_w(mem.read_u16(cpu.r[4] + 6).unwrap_or(0));
            cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + 6).unwrap_or(0));
            cpu.set_t(cpu.r[0] == cpu.r[3]);
            if cpu.r[0] == cpu.r[3] {
                cpu.r[1] = 0;
            }
        }
    }
    cpu.r[0] = cpu.r[1]; // rts delay slot
}
