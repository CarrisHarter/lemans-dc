//! 0x8c23213c - code from 0x8c231e32 plus a table word, with an optional accumulate.
//!
//! The code is `min([r4+12], [r4+16])` mapped by 0x8c231e32. Bit 31 of
//! `[r4+24]` clear adds `table[code]` from 0x8c23d208. Bit set halves the
//! already-halved factors, accumulates, adds 10, then adds `table[code]` from
//! 0x8c23d224. Returns that sum.

use sh4_oracle::{Cpu, Memory};

fn mul(cpu: &mut Cpu) {
    let product = (cpu.r[14] as i32 as i64) * (cpu.r[13] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[11] = cpu.r[4];
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.set_t(cpu.r[13] & 1 != 0);
    cpu.r[13] >>= 1;
    cpu.r[14] = mem.read_u32(cpu.r[11].wrapping_add(16)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[14]);
    cpu.set_t(cpu.r[14] & 1 != 0);
    cpu.r[14] >>= 1;
    mul(cpu);
    cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[12] = cpu.macl;
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bsr delay
    cpu.pr = 0x8c232160;
    crate::registry::p_8c231e32::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c2321e4).unwrap_or(0x8000_0000);
    cpu.r[4] = cpu.r[0];
    cpu.r[2] = mem.read_u32(cpu.r[11].wrapping_add(24)).unwrap_or(0);
    cpu.r[5] = cpu.r[0];
    cpu.set_t((cpu.r[3] & cpu.r[2]) == 0); // tst r3,r2
    cpu.r[5] <<= 2; // bt.s delay: shll2 r5
    if !cpu.t() {
        cpu.set_t(cpu.r[13] & 1 != 0);
        cpu.r[13] >>= 1;
        cpu.set_t(cpu.r[14] & 1 != 0);
        cpu.r[14] >>= 1;
        cpu.set_t(cpu.r[13] >= cpu.r[14]); // cmp/hs r14,r13
        cpu.r[6] = 1; // bt.s delay
        if cpu.t() {
            cpu.r[4] = cpu.r[14];
        } else {
            cpu.r[4] = cpu.r[13]; // bra delay
        }
        cpu.set_t(cpu.r[4] > cpu.r[6]); // cmp/hi r6,r4
        if cpu.t() {
            loop {
                mul(cpu);
                cpu.set_t(cpu.r[13] & 1 != 0);
                cpu.r[13] >>= 1;
                cpu.set_t(cpu.r[14] & 1 != 0);
                cpu.r[14] >>= 1;
                cpu.set_t(cpu.r[4] & 1 != 0);
                cpu.r[4] >>= 1;
                cpu.r[3] = cpu.macl;
                cpu.set_t(cpu.r[4] > cpu.r[6]);
                cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[3]);
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.set_t(cpu.r[14] == cpu.r[13]);
        if cpu.t() {
            cpu.r[12] = cpu.r[12].wrapping_add(2); // bra delay
        } else {
            mul(cpu);
            cpu.r[13] = cpu.macl;
            cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[13]);
        }
        cpu.r[0] = mem.read_u32(0x8c2321e8).unwrap_or(0x8c23d224);
        cpu.r[12] = cpu.r[12].wrapping_add(10);
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[3]);
    } else {
        cpu.r[0] = mem.read_u32(0x8c2321ec).unwrap_or(0x8c23d208);
        cpu.r[2] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[2]);
    }
    cpu.r[0] = cpu.r[12];
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
