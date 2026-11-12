//! 0x8c237932 - zero the descriptor at 0x8c23d798.
//!
//! `[desc+8]` halfwords at `[desc+32] + 24*i` are cleared, then words +36..+64.
//! `[desc+12]` halfwords at `[desc+72] + 76*i` are cleared, then the word at
//! +68. Returns 0. `mul.l` writes both MAC halves.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(0x8c2379a4).unwrap_or(0x8c23d798);
    cpu.r[5] = 0;
    cpu.r[2] = cpu.r[5];
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[2] >= cpu.r[3]); // cmp/hs r3,r2
    cpu.r[6] = cpu.r[5]; // bt.s delay
    if !cpu.t() {
        loop {
            cpu.r[0] = cpu.r[6];
            cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
            cpu.r[0] <<= 1;
            cpu.r[3] = cpu.r[6];
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
            cpu.r[0] <<= 2; // shll2
            cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
            cpu.r[0] <<= 1;
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[2]);
            let _ = mem.write_u16(cpu.r[0], cpu.r[5] as u16);
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] >= cpu.r[3]);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 64;
    cpu.r[6] = cpu.r[5];
    cpu.r[2] = 0;
    for off in [36, 40, 44, 48, 52, 56, 60] {
        let _ = mem.write_u32(cpu.r[4].wrapping_add(off), cpu.r[5]);
    }
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[2] >= cpu.r[3]);
    cpu.r[7] = 76; // bt.s delay
    if !cpu.t() {
        loop {
            let product = (cpu.r[7] as i32 as i64) * (cpu.r[6] as i32 as i64);
            cpu.mach = (product >> 32) as u32;
            cpu.macl = product as u32;
            cpu.r[3] = cpu.r[4].wrapping_add(72);
            cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0);
            cpu.r[6] = cpu.r[6].wrapping_add(1);
            cpu.r[0] = cpu.macl;
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
            let _ = mem.write_u16(cpu.r[0], cpu.r[5] as u16);
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
            cpu.set_t(cpu.r[6] >= cpu.r[2]);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 68;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]);
    cpu.r[0] = 0; // rts delay
}
