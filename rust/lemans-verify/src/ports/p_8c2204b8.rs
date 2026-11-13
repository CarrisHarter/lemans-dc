//! 0x8c2204b8 — clear one 52-byte record and a paired 32-byte slot.
//!
//! Raises the SR interrupt mask, then writes `*r4` at
//! `[*0x8c23d468 + 4 + 52**r4]` and zero at offsets 4, 8, 12, 16, 20, 28, 32,
//! 40, 44 and 48, with 3 at offset 24. A second zero lands at
//! `[**0x8c23d468 + 32**r4 + 20]`. The mask is restored. `r0` is 0.
//! `shll` leaves T from bit 27 of the index.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn hw(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn mul(cpu: &mut Cpu, rn: u32, rm: u32) {
    let product = (rn as i32 as i64) * (rm as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
}

/// `r2 = *r5 + 4 + 52 * *r4`.
fn addr_r2(cpu: &mut Cpu, mem: &Memory) {
    cpu.r[2] = word(mem, cpu.r[4]);
    cpu.r[3] = word(mem, cpu.r[5]);
    mul(cpu, cpu.r[2], cpu.r[6]); // mul.l r6, r2
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[2] = cpu.macl;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
}

/// `r3 = *r5 + 4 + 52 * *r4`.
fn addr_r3(cpu: &mut Cpu, mem: &Memory) {
    cpu.r[3] = word(mem, cpu.r[4]);
    cpu.r[2] = word(mem, cpu.r[5]);
    mul(cpu, cpu.r[3], cpu.r[6]); // mul.l r6, r3
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.r[3] = cpu.macl;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
}

fn store_at(mem: &mut Memory, addr: u32, value: u32) {
    let _ = mem.write_u32(addr, value);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[0] = cpu.sr;
    cpu.r[3] = hw(mem, 0x8c2205a8); // 0xffffff0f
    cpu.r[0] >>= 2;
    cpu.r[0] >>= 2;
    cpu.r[0] &= 15;
    store_at(mem, cpu.r[15], cpu.r[0]);
    cpu.r[0] = cpu.sr;
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0];

    cpu.r[7] = word(mem, cpu.r[4]);
    cpu.r[6] = 52;
    cpu.r[5] = word(mem, 0x8c2205ac); // 0x8c23d468
    mul(cpu, cpu.r[7], cpu.r[6]); // mul.l r6, r7
    cpu.r[2] = word(mem, cpu.r[5]);
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.r[3] = cpu.macl;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    store_at(mem, cpu.r[3], cpu.r[7]);
    cpu.r[7] = 0;

    addr_r2(cpu, mem);
    store_at(mem, cpu.r[2].wrapping_add(40), cpu.r[7]);
    addr_r3(cpu, mem);
    store_at(mem, cpu.r[3].wrapping_add(44), cpu.r[7]);
    addr_r2(cpu, mem);
    store_at(mem, cpu.r[2].wrapping_add(48), cpu.r[7]);
    addr_r3(cpu, mem);
    store_at(mem, cpu.r[3].wrapping_add(4), cpu.r[7]);
    addr_r2(cpu, mem);
    store_at(mem, cpu.r[2].wrapping_add(8), cpu.r[7]);
    addr_r3(cpu, mem);
    store_at(mem, cpu.r[3].wrapping_add(12), cpu.r[7]);
    addr_r2(cpu, mem);
    store_at(mem, cpu.r[2].wrapping_add(16), cpu.r[7]);
    addr_r3(cpu, mem);
    store_at(mem, cpu.r[3].wrapping_add(20), cpu.r[7]);
    cpu.r[1] = 3;
    addr_r2(cpu, mem);
    store_at(mem, cpu.r[2].wrapping_add(24), cpu.r[1]);
    addr_r2(cpu, mem);
    store_at(mem, cpu.r[2].wrapping_add(28), cpu.r[7]);

    cpu.r[3] = word(mem, cpu.r[4]);
    cpu.r[2] = word(mem, cpu.r[5]);
    mul(cpu, cpu.r[3], cpu.r[6]); // mul.l r6, r3
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.r[6] = cpu.macl;
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[2]);
    store_at(mem, cpu.r[6].wrapping_add(32), cpu.r[7]);

    cpu.r[1] = word(mem, cpu.r[4]);
    cpu.r[3] = word(mem, cpu.r[5]);
    cpu.r[1] <<= 2;
    cpu.r[3] = word(mem, cpu.r[3]);
    cpu.r[1] <<= 2;
    cpu.set_t(cpu.r[1] >> 31 != 0); // shll r1
    cpu.r[1] <<= 1;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    cpu.r[4] = cpu.r[1];
    store_at(mem, cpu.r[4].wrapping_add(20), cpu.r[7]);

    cpu.r[0] = word(mem, cpu.r[15]);
    cpu.r[2] = cpu.sr;
    cpu.r[3] = hw(mem, 0x8c2205a8);
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2;
    cpu.r[2] &= cpu.r[3];
    cpu.r[0] <<= 2;
    cpu.r[0] |= cpu.r[2];
    cpu.sr = cpu.r[0];
    cpu.r[0] = cpu.r[7];
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay
}
