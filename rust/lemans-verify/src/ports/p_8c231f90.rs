//! 0x8c231f90 - scaled product of `[r4+12]` and `[r4+16]`.
//!
//! Bit 31 of `[r4+24]` clear: the product is shifted by 1 or 2 depending on
//! whether `[r4+4] == 1`. Bit set: both factors are halved, then a short
//! multiply-accumulate runs while the smaller factor stays above 1. `rts`
//! delay copies `r7` into `r0`.

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
    mul(cpu);
    cpu.r[3] = mem.read_u32(0x8c231fc8).unwrap_or(0x8000_0000);
    cpu.set_t((cpu.r[3] & cpu.r[2]) == 0); // tst r3,r2
    cpu.r[7] = cpu.macl; // bt.s delay
    if !cpu.t() {
        cpu.set_t(cpu.r[6] & 1 != 0); // shlr r6
        cpu.r[6] >>= 1;
        cpu.set_t(cpu.r[5] & 1 != 0); // shlr r5
        cpu.r[5] >>= 1;
        cpu.set_t(cpu.r[6] >= cpu.r[5]); // cmp/hs r5,r6
        cpu.r[0] = 1; // bt.s delay
        if cpu.t() {
            cpu.r[1] = cpu.r[5];
        } else {
            cpu.r[1] = cpu.r[6]; // bra delay
        }
        cpu.set_t(cpu.r[1] > cpu.r[0]); // cmp/hi r0,r1
        if cpu.t() {
            loop {
                mul(cpu);
                cpu.set_t(cpu.r[6] & 1 != 0);
                cpu.r[6] >>= 1;
                cpu.set_t(cpu.r[5] & 1 != 0);
                cpu.r[5] >>= 1;
                cpu.set_t(cpu.r[1] & 1 != 0);
                cpu.r[1] >>= 1;
                cpu.r[3] = cpu.macl;
                cpu.set_t(cpu.r[1] > cpu.r[0]); // cmp/hi r0,r1
                cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]); // bt delay, always
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.set_t(cpu.r[5] == cpu.r[6]); // cmp/eq r5,r6
        if cpu.t() {
            cpu.r[7] = cpu.r[7].wrapping_add(4); // bra delay
        } else {
            mul(cpu);
            cpu.r[6] = cpu.macl;
            cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[6]);
        }
    }
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.t() {
        cpu.set_t(cpu.r[7] >> 31 != 0); // bra delay: shll r7
        cpu.r[7] <<= 1;
    } else {
        cpu.r[7] <<= 2; // shll2, T unchanged
    }
    cpu.r[0] = cpu.r[7]; // rts delay
}
