//! 0x8c0ba4d4 - store a slot pointer, then maybe clear it.
//!
//! Returns immediately when byte `[r4]` is 0, `r5` is negative, or `r5 > 3`.
//! Otherwise `0x8c21ccf6(r6 + table[r5])` is stored at
//! `0x8c27a038 + 24*r5 + 4*r6`. If `0x8c21ccec(table[r5])` is nonzero, that
//! slot is cleared.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[11] = cpu.r[5];
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[0] &= 0xff;
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    let mut skip = cpu.t();
    if !skip {
        cpu.set_t((cpu.r[11] as i32) >= 0); // cmp/pz r11
        if !cpu.t() {
            skip = true;
        }
    }
    if !skip {
        cpu.r[0] = 3;
        cpu.set_t((cpu.r[11] as i32) > (cpu.r[0] as i32)); // cmp/gt r0,r11
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c0ba544).unwrap_or(0x8c1fd440);
            cpu.r[13] = cpu.r[11];
            cpu.r[13] <<= 2;
            cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
            cpu.r[4] = cpu.r[12];
            cpu.r[0] = mem.read_u32(0x8c0ba548).unwrap_or(0x8c21ccf6);
            cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // jsr delay
            cpu.pr = 0x8c0ba50a;
            crate::registry::p_8c21ccf6::run(cpu, mem);
            cpu.r[14] = cpu.r[11];
            cpu.set_t(cpu.r[14] >> 31 != 0); // shll r14
            cpu.r[14] <<= 1;
            cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[11]);
            cpu.r[14] <<= 2;
            cpu.set_t(cpu.r[14] >> 31 != 0); // shll r14
            cpu.r[14] <<= 1;
            cpu.r[3] = mem.read_u32(0x8c0ba54c).unwrap_or(0x8c27a038);
            cpu.r[1] = cpu.r[14];
            cpu.r[4] = cpu.r[0];
            cpu.r[12] <<= 2;
            cpu.r[0] = cpu.r[12];
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[1].wrapping_add(cpu.r[0]), cpu.r[4]);
            cpu.r[0] = mem.read_u32(0x8c0ba544).unwrap_or(0x8c1fd440);
            cpu.r[1] = mem.read_u32(0x8c0ba550).unwrap_or(0x8c21ccec);
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0); // jsr delay
            cpu.pr = 0x8c0ba52a;
            crate::registry::p_8c21ccec::run(cpu, mem);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if !cpu.t() {
                cpu.r[1] = mem.read_u32(0x8c0ba54c).unwrap_or(0x8c27a038);
                cpu.r[0] = cpu.r[12];
                cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[1]);
                cpu.r[1] = 0;
                let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
            }
        }
    }
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
