//! 0x8c232096 - scaled product of `[r4+12]` and `[r4+16]`.
//!
//! Bit 31 of `[r4+24]` clear returns the product, unless `[r4+8]` is
//! 0x28000000: then a product of 1 stays 1 and any other product is halved.
//! Bit set halves both factors, accumulates, then adds 43 or 11 depending on
//! that same constant, and runs the same tail. `rts` delay restores r14.

use sh4_oracle::{Cpu, Memory};

fn mul(cpu: &mut Cpu) {
    let product = (cpu.r[7] as i32 as i64) * (cpu.r[6] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c232104).unwrap_or(0x8000_0000);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let saved14 = cpu.r[14];
    let _ = mem.write_u32(cpu.r[15], saved14);
    cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
    mul(cpu);
    cpu.set_t((cpu.r[3] & cpu.r[2]) == 0); // tst r3,r2
    cpu.r[5] = cpu.macl;
    cpu.r[14] = 1; // bt.s delay
    if !cpu.t() {
        cpu.set_t(cpu.r[6] & 1 != 0);
        cpu.r[6] >>= 1;
        cpu.set_t(cpu.r[7] & 1 != 0);
        cpu.r[7] >>= 1;
        cpu.set_t(cpu.r[6] >= cpu.r[7]); // cmp/hs r7,r6
        if cpu.t() {
            cpu.r[0] = cpu.r[7];
        } else {
            cpu.r[0] = cpu.r[6]; // bra delay
        }
        cpu.set_t(cpu.r[0] > cpu.r[14]); // cmp/hi r14,r0
        if cpu.t() {
            loop {
                mul(cpu);
                cpu.set_t(cpu.r[6] & 1 != 0);
                cpu.r[6] >>= 1;
                cpu.set_t(cpu.r[7] & 1 != 0);
                cpu.r[7] >>= 1;
                cpu.set_t(cpu.r[0] & 1 != 0);
                cpu.r[0] >>= 1;
                cpu.r[3] = cpu.macl;
                cpu.set_t(cpu.r[0] > cpu.r[14]);
                cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[3]); // bt delay, always
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.set_t(cpu.r[7] == cpu.r[6]);
        if cpu.t() {
            cpu.r[5] = cpu.r[5].wrapping_add(1); // bra delay
        } else {
            mul(cpu);
            cpu.r[6] = cpu.macl;
            cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
        }
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
        cpu.r[2] = mem.read_u32(0x8c232108).unwrap_or(0x2800_0000);
        cpu.set_t(cpu.r[2] == cpu.r[3]);
        if cpu.t() {
            cpu.r[5] = cpu.r[5].wrapping_add(43); // bra delay
        } else {
            cpu.r[5] = cpu.r[5].wrapping_add(11);
        }
    }
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.r[2] = mem.read_u32(0x8c2321e0).unwrap_or(0x2800_0000);
    cpu.set_t(cpu.r[2] == cpu.r[3]);
    if cpu.t() {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 1);
        if cpu.t() {
            cpu.r[5] = cpu.r[14]; // bra delay
        } else {
            cpu.set_t(cpu.r[5] & 1 != 0); // shlr r5
            cpu.r[5] >>= 1;
        }
    }
    cpu.r[0] = cpu.r[5];
    cpu.r[14] = saved14; // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
