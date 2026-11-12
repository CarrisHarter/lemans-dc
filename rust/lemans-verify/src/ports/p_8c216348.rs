//! 0x8c216348 - copy 60 bytes into a stride-64 slot, then check a tag.
//!
//! A nonzero halfword at `r4+2` returns -6. Otherwise the destination is
//! `([r4+12] + (([r4+40]+1) << 6))` and the source is `[r4+48]`. A tag of
//! 0xffff, or a tag matching 0x8c215f6c(62, dest), returns 0; else -4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u16(cpu.r[13].wrapping_add(2)).unwrap_or(0) as i16 as i32 as u32;
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[14] = mem.read_u32(cpu.r[13].wrapping_add(40)).unwrap_or(0);
        cpu.r[6] = 60;
        cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.r[5] = mem.read_u32(cpu.r[13].wrapping_add(48)).unwrap_or(0);
        cpu.r[14] <<= 2;
        cpu.r[14] <<= 2;
        cpu.r[14] <<= 2;
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[3]);
        cpu.r[4] = cpu.r[14];
        cpu.r[4] = cpu.r[4].wrapping_add(2); // bsr delay
        cpu.pr = 0x8c21636e;
        crate::registry::p_8c215fe0::run(cpu, mem);
        cpu.r[3] = mem.read_u16(cpu.r[14]).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[2] = mem.read_u32(0x8c2163b8).unwrap_or(0xffff);
        cpu.r[3] &= 0xffff; // extu.w
        cpu.set_t(cpu.r[2] == cpu.r[3]);
        if !cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c2163bc).unwrap_or(0x8c215f6c);
            cpu.r[5] = cpu.r[14];
            cpu.r[4] = 62; // jsr delay
            cpu.pr = 0x8c216380;
            crate::registry::p_8c215f6c::run(cpu, mem);
            cpu.r[4] = cpu.r[0];
            cpu.r[0] = 62;
            cpu.r[3] = mem.read_u16(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[4] &= 0xffff;
            cpu.r[3] &= 0xffff;
            cpu.set_t(cpu.r[4] == cpu.r[3]);
            if cpu.t() {
                cpu.r[0] = 0; // bra delay
            } else {
                cpu.r[0] = (-4i32) as u32; // bra delay
            }
        } else {
            cpu.r[0] = 0; // bra delay
        }
    } else {
        cpu.r[0] = (-6i32) as u32;
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
