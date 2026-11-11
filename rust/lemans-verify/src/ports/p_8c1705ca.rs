//! 0x8c1705ca — bit-field clear via cache-hint-style masked word stores.
//!
//! Base r5 = pool(0x8c2b5260); reads v = [r5+124] and decides on v == 2.
//! Masks: r12 = ~(1 shad r4), r14 = ~(1 shad (r4-16)) (the bf.s delay slot
//! `not r14,r14` always runs). Depending on v and r4 (unsigned) it clears
//! one 16-bit field: r4<16 -> [r5+0x90] &= r12; 16<=r4<32 -> [r5+0x92] &= r14;
//! 32<=r4<48 -> [r5+0x94] &= ~(1 shad (r4-32)) and r4 -= 32;
//! 48<=r4<63 -> [r5+0x96] &= ~(1 shad (r4-48)) and r4 -= 48 (here r1 ends
//! as r5+0x96 and r0 as the masked value); r4>=63 -> no store.
//! r12/r13/r14 are pushed/popped; leaked: r0, r1, r2, r3, r4, r5, r6, r7, T.

use sh4_oracle::{Cpu, Memory};

fn shad(amount: u32, v: u32) -> u32 {
    let a = amount as i32;
    if a >= 0 {
        v << (a as u32 & 31)
    } else {
        ((v as i32) >> ((-a) as u32 & 31)) as u32
    }
}

fn sext16(v: u16) -> u32 {
    v as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4in = cpu.r[4];
    let r5 = mem.read_u32(0x8c170664).unwrap(); // 0x8c2b5260
    let w90 = sext16(mem.read_u16(0x8c170660).unwrap()); // 0x90
    let r13 = w90.wrapping_add(r5).wrapping_add(2); // 0x8c2b52f2
    let r7 = w90.wrapping_add(r5); // 0x8c2b52f0
    let mut r0 = mem.read_u32(r5 + 124).unwrap();
    let mut r3 = r4in.wrapping_sub(16);
    let r12 = !shad(r4in, 1);
    let r14 = !shad(r3, 1);
    let mut r4 = r4in;
    let mut r6 = 1u32;
    let mut r2 = 0u32;
    let mut r1 = 0u32;

    cpu.set_t(r0 == 2); // cmp/eq #2,r0
    if cpu.t() {
        // bf.s not taken: fall through 0x8c1705f6
        r2 = 16;
        cpu.set_t(r4 >= 16); // cmp/hs r2,r4
        if cpu.t() {
            r0 = 31;
            cpu.set_t(r4 >= 31); // cmp/hs r0,r4
            if !cpu.t() {
                r1 = sext16(mem.read_u16(r13).unwrap()) & r14;
                mem.write_u16(r13, r1 as u16).unwrap(); // bra delay slot
            }
        } else {
            r1 = sext16(mem.read_u16(r7).unwrap()) & r12;
            mem.write_u16(r7, r1 as u16).unwrap(); // bra delay slot
        }
    } else {
        // bf.s taken to 0x8c17060a
        r2 = 16;
        cpu.set_t(r4 >= 16); // cmp/hs r2,r4
        if !cpu.t() {
            r1 = sext16(mem.read_u16(r7).unwrap()) & r12;
            mem.write_u16(r7, r1 as u16).unwrap(); // bra delay slot 0x8c170616
        } else {
            r0 = 32;
            cpu.set_t(r4 >= 32); // cmp/hs r0,r4
            if !cpu.t() {
                r1 = sext16(mem.read_u16(r13).unwrap()) & r14;
                mem.write_u16(r13, r1 as u16).unwrap(); // bra delay slot 0x8c170624
            } else {
                r3 = 48;
                cpu.set_t(r4 >= 48); // cmp/hs r3,r4
                if !cpu.t() {
                    let off = mem.read_u16(0x8c17065c).unwrap() as u32; // 0x94
                    r0 = off;
                    r4 = r4.wrapping_sub(32);
                    r6 = !shad(r4, 1);
                    r1 = sext16(mem.read_u16(r5 + off).unwrap()) & r6;
                    mem.write_u16(r5 + off, r1 as u16).unwrap(); // bra delay slot 0x8c17063a
                } else {
                    r2 = 63;
                    cpu.set_t(r4 >= 63); // cmp/hs r2,r4
                    if !cpu.t() {
                        let off = mem.read_u16(0x8c17065e).unwrap() as u32; // 0x96
                        r4 = r4.wrapping_sub(48);
                        r6 = !shad(r4, 1);
                        r0 = sext16(mem.read_u16(r5 + off).unwrap()) & r6;
                        r1 = off.wrapping_add(r5);
                        mem.write_u16(r5 + off, r0 as u16).unwrap(); // plain 0x8c170652
                    }
                }
            }
        }
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[2] = r2;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
