//! 0x8c232014 - scaled product of the halved fields, plus 0x800.
//!
//! Both factors are shifted once before the first multiply. Bit 31 of
//! `[r4+24]` selects the accumulate loop (equal factors add 2, then +10).
//! The clear path adds 16 when both original fields are 8. `rts` delay
//! copies `r7` into `r0`.

use sh4_oracle::{Cpu, Memory};

fn mul(cpu: &mut Cpu) {
    let product = (cpu.r[5] as i32 as i64) * (cpu.r[6] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
    cpu.set_t(cpu.r[6] & 1 != 0);
    cpu.r[6] >>= 1;
    cpu.r[3] = mem.read_u32(0x8c232060).unwrap_or(0x8000_0000);
    cpu.set_t(cpu.r[5] & 1 != 0);
    cpu.r[5] >>= 1;
    mul(cpu);
    cpu.set_t((cpu.r[3] & cpu.r[2]) == 0);
    cpu.r[7] = cpu.macl; // bt.s delay
    if !cpu.t() {
        cpu.set_t(cpu.r[6] & 1 != 0);
        cpu.r[6] >>= 1;
        cpu.set_t(cpu.r[5] & 1 != 0);
        cpu.r[5] >>= 1;
        cpu.set_t(cpu.r[6] >= cpu.r[5]); // cmp/hs r5,r6
        cpu.r[0] = 1; // bt.s delay
        if cpu.t() {
            cpu.r[4] = cpu.r[5];
        } else {
            cpu.r[4] = cpu.r[6]; // bra delay
        }
        cpu.set_t(cpu.r[4] > cpu.r[0]); // cmp/hi r0,r4
        if cpu.t() {
            loop {
                mul(cpu);
                cpu.set_t(cpu.r[6] & 1 != 0);
                cpu.r[6] >>= 1;
                cpu.set_t(cpu.r[5] & 1 != 0);
                cpu.r[5] >>= 1;
                cpu.set_t(cpu.r[4] & 1 != 0);
                cpu.r[4] >>= 1;
                cpu.r[3] = cpu.macl;
                cpu.set_t(cpu.r[4] > cpu.r[0]);
                cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]); // bt delay, always
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.set_t(cpu.r[5] == cpu.r[6]);
        if cpu.t() {
            cpu.r[7] = cpu.r[7].wrapping_add(2); // bra delay
        } else {
            mul(cpu);
            cpu.r[6] = cpu.macl;
            cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[6]);
        }
        cpu.r[7] = cpu.r[7].wrapping_add(10); // bra delay into the common tail
    } else {
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == 8);
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == 8);
            if cpu.t() {
                cpu.r[7] = cpu.r[7].wrapping_add(16);
            }
        }
    }
    cpu.r[3] = mem.read_u16(0x8c232100).unwrap_or(0x0800) as i16 as i32 as u32;
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    cpu.r[0] = cpu.r[7]; // rts delay
}
