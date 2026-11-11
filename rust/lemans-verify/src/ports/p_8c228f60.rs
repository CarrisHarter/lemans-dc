//! 0x8c228f60 — gate on `[r4+20]==[r4+24]` (else return 1), then three
//! optional bitmask updates from `[r4+28/32/36]` (each -1 = skip, or skip
//! when the index is past the limit word): nonzero flag ORs `1<<idx` into
//! the set word, zero flag ANDs the inverse into the clear word. Finally
//! zeroes `[r4]`, unlinks the node via `[r4+12]` (prev) / `[r4+16]` (next),
//! returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
    cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r3,r2
    if !cpu.t() {
        cpu.r[0] = 1;
        return;
    }
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(28)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == (-1i32) as u32); // cmp/eq #-1,r0
    cpu.r[5] = 1; // bt.s delay slot: always runs
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c229040).unwrap_or(0); // 0x8c2c6f70
        cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(28)).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.set_t(cpu.r[6] >= cpu.r[2]); // cmp/hs r2,r6
        if !cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c229044).unwrap_or(0); // 0x8c2c6f94
            cpu.r[7] = cpu.r[5];
            cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            cpu.set_t(cpu.r[2] & cpu.r[2] == 0); // tst r2,r2
            cpu.r[7] = cpu.r[7].wrapping_shl(cpu.r[6]); // bt.s delay slot: shad r6,r7 (r6 >= 0 here)
            if !cpu.t() {
                cpu.r[2] = mem.read_u32(0x8c229048).unwrap_or(0); // 0x8c2c6f88
                cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
                cpu.r[0] |= cpu.r[7];
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
            } else {
                cpu.r[2] = mem.read_u32(0x8c22904c).unwrap_or(0); // 0x8c2c6f7c
                cpu.r[7] = !cpu.r[7];
                cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
                cpu.r[0] &= cpu.r[7];
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
            }
        }
    }
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == (-1i32) as u32); // cmp/eq #-1,r0
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c229050).unwrap_or(0); // 0x8c2c6f74
        cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.set_t(cpu.r[7] >= cpu.r[2]); // cmp/hs r2,r7
        if !cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c229054).unwrap_or(0); // 0x8c2c6f98
            cpu.r[6] = cpu.r[5];
            cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            cpu.set_t(cpu.r[2] & cpu.r[2] == 0); // tst r2,r2
            cpu.r[6] = cpu.r[6].wrapping_shl(cpu.r[7]); // bt.s delay slot: shad r7,r6
            if !cpu.t() {
                cpu.r[2] = mem.read_u32(0x8c229058).unwrap_or(0); // 0x8c2c6f8c
                cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
                cpu.r[0] |= cpu.r[6];
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
            } else {
                cpu.r[2] = mem.read_u32(0x8c22905c).unwrap_or(0); // 0x8c2c6f80
                cpu.r[6] = !cpu.r[6];
                cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
                cpu.r[0] &= cpu.r[6];
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
            }
        }
    }
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == (-1i32) as u32); // cmp/eq #-1,r0
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c229060).unwrap_or(0); // 0x8c2c6f78
        cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.set_t(cpu.r[6] >= cpu.r[2]); // cmp/hs r2,r6
        if !cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c229064).unwrap_or(0); // 0x8c2c6f9c
            cpu.r[7] = cpu.r[5];
            cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
            cpu.set_t(cpu.r[2] & cpu.r[2] == 0); // tst r2,r2
            cpu.r[7] = cpu.r[7].wrapping_shl(cpu.r[6]); // bt.s delay slot: shad r6,r7
            if !cpu.t() {
                cpu.r[2] = mem.read_u32(0x8c229068).unwrap_or(0); // 0x8c2c6f90
                cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
                cpu.r[0] |= cpu.r[7];
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
            } else {
                cpu.r[2] = mem.read_u32(0x8c22906c).unwrap_or(0); // 0x8c2c6f84
                cpu.r[7] = !cpu.r[7];
                cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
                cpu.r[0] &= cpu.r[7];
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
            }
        }
    }
    cpu.r[6] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[6]);
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[5] & cpu.r[5] == 0); // tst r5,r5
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0); // bt.s delay slot: always runs
    if !cpu.t() {
        cpu.set_t(cpu.r[4] & cpu.r[4] == 0); // tst r4,r4
        if !cpu.t() {
            let _ = mem.write_u32(cpu.r[5].wrapping_add(16), cpu.r[4]);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[5]); // bra delay slot
        } else {
            let _ = mem.write_u32(cpu.r[5].wrapping_add(16), cpu.r[6]); // bra delay slot
        }
    } else {
        cpu.set_t(cpu.r[4] & cpu.r[4] == 0); // tst r4,r4
        if !cpu.t() {
            let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[6]);
        }
    }
    cpu.r[0] = 0; // rts delay slot
}
