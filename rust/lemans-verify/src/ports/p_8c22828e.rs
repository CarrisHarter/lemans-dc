//! 0x8c22828e — flag bit search; rts r0 = bit index or -1.
//!
//! `bt.s` runs its delay slot (`mov #1,r5`) on both paths, and `shad` treats
//! the shift count as signed: negative counts shift right.
//!
//! Side effects: clears 0x8c2c6f8c/0x8c2c6f98 and masks a bit out of
//! 0x8c2c6f80 when 0x8c2c6f8c starts non-zero; T mirrors the last compare.

use sh4_oracle::{Cpu, Memory};

fn shad(amount: u32, value: u32) -> u32 {
    let a = amount as i32;
    if a >= 0 {
        value << ((a as u32) & 31)
    } else {
        ((value as i32) >> (a.wrapping_neg() as u32 & 31)) as u32
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x8c2c6f8c;
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    let clear = cpu.r[4] == 0; // tst r4,r4
    cpu.set_t(clear);
    cpu.r[6] = 0;
    cpu.r[5] = 1; // bt.s delay slot: runs whether or not the branch is taken
    if !clear {
        cpu.r[1] = 0x8c2c6f8c;
        let _ = mem.write_u32(cpu.r[1], cpu.r[6]);
        cpu.r[2] = 0x8c2c6f98;
        let _ = mem.write_u32(cpu.r[2], cpu.r[5]);
        cpu.r[3] = 0x8c2c6f80;
        cpu.r[4] = !cpu.r[4];
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[0] &= cpu.r[4];
        let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
        cpu.r[3] = cpu.r[3].wrapping_add(24);
        let _ = mem.write_u32(cpu.r[3], cpu.r[6]);
    }
    cpu.r[1] = 0x8c2c7e24;
    cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let zero = cpu.r[2] == 0; // tst r2,r2
    cpu.set_t(zero);
    if !zero {
        cpu.r[2] = 0x8c2c7e20;
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.r[5] = shad(cpu.r[3], cpu.r[5]); // shad r3,r5
        cpu.r[3] = 0x8c2c6f80;
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        let none = (cpu.r[0] & cpu.r[5]) == 0; // tst r0,r5
        cpu.set_t(none);
        if !none {
            cpu.r[0] = 0xffffffff; // bf 0x8c2282f2
            return;
        }
        cpu.r[1] = 0x8c2c7e20;
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0); // rts delay slot
        return;
    }
    cpu.r[0] = 0x8c2c6f74;
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[2] = 0;
    let empty = cpu.r[2] >= cpu.r[3]; // cmp/hs r3,r2
    cpu.set_t(empty);
    cpu.r[4] = cpu.r[6]; // bt.s delay slot
    if empty {
        cpu.r[0] = 0xffffffff;
        return;
    }
    loop {
        cpu.r[1] = 0x8c2c6f80;
        cpu.r[3] = shad(cpu.r[4], cpu.r[5]); // shad r4,r3
        cpu.r[2] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        let found = (cpu.r[2] & cpu.r[3]) == 0; // tst r2,r3
        cpu.set_t(found);
        if found {
            cpu.r[0] = cpu.r[4]; // rts delay slot
            return;
        }
        cpu.r[2] = 0x8c2c6f74;
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        let done = cpu.r[4] >= cpu.r[3]; // cmp/hs r3,r4
        cpu.set_t(done);
        if done {
            cpu.r[0] = 0xffffffff;
            return;
        }
    }
}
