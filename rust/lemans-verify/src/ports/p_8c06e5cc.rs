//! 0x8c06e5cc — signed coordinate check plus a two-level tile lookup.
//!
//! r4 = descriptor, r5 = x, r6 = y. Both axes must be signed-non-negative and
//! strictly below descriptor[0x3c] / descriptor[0x40]; any other case returns
//! r0 = 0. The axes are then scaled by `shad` with the negated
//! descriptor[0x44], and the map halfword at
//! descriptor[0x10] + 2*((x << sh) + (y << sh) * descriptor[8]) selects the
//! entry; a zero entry returns 0. A nonzero entry n reads
//! (n-1) * descriptor[0x4c] plus the masked axes scaled by descriptor[0x2c],
//! sign-extends 2*that index from descriptor[0x34], and adds the u32 at
//! descriptor[0x38] + 4*(n-1).
//!
//! Exit state is deliberate: each early return carries the T bit of the
//! comparison that chose it (`cmp/pz` false, `cmp/ge` true), the zero-entry
//! exit carries T=1 from `tst`, and the full path carries T from the final
//! `shll r3` (the last flag-setting instruction before `rts`). `mul.l` leaves
//! MACL/MACH from the last multiply reached on each path.

use sh4_oracle::{Cpu, Memory};

/// `shad Rm,Rn`: shift right arithmetic when the amount is negative, left
/// otherwise. The oracle takes the count from the low five bits in both
/// directions, including the `i32::MIN` wraparound case.
fn shad(value: u32, amount: u32) -> u32 {
    let amount = amount as i32;
    if amount >= 0 {
        value << ((amount as u32) & 31)
    } else {
        ((value as i32) >> (amount.wrapping_neg() as u32 & 31)) as u32
    }
}

/// `mul.l Rm,Rn`: signed 32x32 product; MACH gets the high word, MACL the low.
/// Returns MACL, as the following `sts macl,Rn` would.
fn mul_l(cpu: &mut Cpu, a: u32, b: u32) -> u32 {
    let product = (a as i32 as i64) * (b as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.macl
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r6 = cpu.r[6];

    // cmp/pz r5 ; bf -> return 0 with T=0.
    if (r5 as i32) < 0 {
        cpu.set_t(false);
        cpu.r[0] = 0;
        return;
    }
    // mov.l @(60,r4),r0 ; cmp/ge r0,r5 ; bf -> continue only while r5 < [0x3c].
    cpu.r[0] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
    if (r5 as i32) >= (cpu.r[0] as i32) {
        cpu.set_t(true);
        cpu.r[0] = 0;
        return;
    }
    // cmp/pz r6 ; bf -> return 0 with T=0.
    if (r6 as i32) < 0 {
        cpu.set_t(false);
        cpu.r[0] = 0;
        return;
    }
    // mov.l @(64,r4),r0 ; cmp/ge r0,r6 ; bf -> continue only while r6 < [0x40].
    cpu.r[0] = mem.read_u32(r4.wrapping_add(64)).unwrap_or(0);
    if (r6 as i32) >= (cpu.r[0] as i32) {
        cpu.set_t(true);
        cpu.r[0] = 0;
        return;
    }

    // neg [0x44] ; both `shad` uses take this same amount.
    cpu.r[0] = 0u32.wrapping_sub(mem.read_u32(r4.wrapping_add(68)).unwrap_or(0));
    cpu.r[3] = shad(r6, cpu.r[0]); // shad r0,r3
    cpu.r[1] = mem.read_u32(r4.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r1
    cpu.r[3] = mul_l(cpu, cpu.r[1], cpu.r[3]); // mul.l r1,r3 ; sts macl,r3
    cpu.r[1] = shad(r5, cpu.r[0]); // shad r0,r1
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // add r3,r1
    cpu.r[0] = mem.read_u32(r4.wrapping_add(16)).unwrap_or(0); // mov.l @(16,r4),r0
    cpu.r[1] <<= 1; // shll r1
    cpu.r[0] = mem
        .read_u16(cpu.r[0].wrapping_add(cpu.r[1]))
        .unwrap_or(0) as i16 as i32 as u32; // mov.w @(r0,r1),r0
    cpu.r[7] = cpu.r[0] & 0xffff; // extu.w r0,r7
    cpu.set_t(cpu.r[7] == 0); // tst r7,r7 ; bt -> r0 = 0
    if cpu.r[7] == 0 {
        cpu.r[0] = 0;
        return;
    }

    // mov.l @(76,r4),r0 ; add #-1,r7 ; mul.l r0,r3 ; sts macl,r3
    cpu.r[0] = mem.read_u32(r4.wrapping_add(76)).unwrap_or(0);
    cpu.r[7] = cpu.r[7].wrapping_sub(1);
    cpu.r[3] = mul_l(cpu, cpu.r[0], cpu.r[7]);
    cpu.r[1] = mem.read_u32(r4.wrapping_add(72)).unwrap_or(0); // mov.l @(72,r4),r1
    cpu.r[7] <<= 2; // shll2 r7
    cpu.r[0] = mem.read_u32(r4.wrapping_add(44)).unwrap_or(0); // mov.l @(44,r4),r0
    cpu.r[6] = r6 & cpu.r[1]; // and r1,r6
    cpu.r[5] = r5 & cpu.r[1]; // and r1,r5
    cpu.r[6] = mul_l(cpu, cpu.r[0], cpu.r[6]); // mul.l r0,r6 ; sts macl,r6
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]); // add r6,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]); // add r5,r3
    cpu.r[0] = mem.read_u32(r4.wrapping_add(52)).unwrap_or(0); // mov.l @(52,r4),r0
    cpu.set_t(cpu.r[3] >> 31 != 0); // shll r3
    cpu.r[3] <<= 1;
    cpu.r[1] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // mov.l @(56,r4),r1
    cpu.r[3] = mem
        .read_u16(cpu.r[0].wrapping_add(cpu.r[3]))
        .unwrap_or(0) as i16 as i32 as u32; // mov.w @(r0,r3),r3
    cpu.r[0] = cpu.r[7]; // mov r7,r0
    cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r1),r1
    cpu.r[0] = cpu.r[3] as u16 as i16 as i32 as u32; // exts.w r3,r0
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // rts delay slot: add r1,r0
}
