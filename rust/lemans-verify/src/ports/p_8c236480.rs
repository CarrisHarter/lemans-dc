//! 0x8c236480 — if `[0x8c2c8b08+4]` is zero, set SR IMASK from
//! `[0x8c2c8b08+8] & 15` and return 0; else return 0xeeee (both via the
//! rts delay slot `mov r5,r0`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(0x8c2364c8).unwrap_or(0); // 0x8c2c8b08
    cpu.r[5] = mem.read_u32(0x8c2364c4).unwrap_or(0); // 0xeeee
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[3] & cpu.r[3] == 0); // tst r3,r3
    if cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
        cpu.r[5] = 0;
        cpu.r[3] = cpu.sr; // stc sr,r3
        cpu.r[2] = mem.read_u16(0x8c2364c0).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
        cpu.r[0] &= 15;
        cpu.r[0] <<= 2; // shll2
        cpu.r[3] &= cpu.r[2];
        cpu.r[0] <<= 2; // shll2
        cpu.r[0] |= cpu.r[3];
        cpu.sr = cpu.r[0]; // ldc r0,sr
    }
    cpu.r[0] = cpu.r[5]; // rts delay slot
}
